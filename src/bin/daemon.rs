use ashpd::desktop::{
    screencast::{CursorMode, Screencast, SourceType},
    PersistMode, Session,
};
use dirs::config_dir;
use gst::prelude::{Cast, ElementExt, GstBinExt, ObjectExt};
use gstreamer::{self as gst};
use gstreamer_app::AppSink;
use sd_notify::NotifyState;
use std::env;
use std::fs::{create_dir_all, metadata, remove_file};
use std::fs::{read_to_string, write};
use std::os::unix::io::AsRawFd;
use std::process::{exit, Stdio};
use std::sync::{
    atomic::{AtomicBool, AtomicUsize, Ordering},
    Arc, Mutex,
};
use std::time::{Duration, Instant};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixListener;
use tokio::process::Command;
use tokio::signal::unix::{signal, SignalKind};
use tokio::sync::mpsc::{channel, Receiver, Sender};
use wayclip_core::{
    cleanup, desktop::setup_trigger, generate_preview_clip, get_pipewire_node_id,
    handle_bus_messages, log_to, logging::Logger, ring::RingBuffer, send_status_to_gui,
    settings::Settings,
};

const SAVE_COOLDOWN: Duration = Duration::from_secs(2);
const MAX_PORTAL_RETRIES: u32 = 3;

enum EncoderType {
    Nvidia,
    Vaapi,
    Software,
}

fn detect_encoder(logger: &Logger) -> (EncoderType, &'static str) {
    if gst::ElementFactory::find("nvh264enc").is_some() {
        log_to!(logger, Info, [GST] => "NVIDIA NVENC detected. Using nvh264enc.");
        (EncoderType::Nvidia, "nvh264enc")
    } else if gst::ElementFactory::find("vaapih264enc").is_some() {
        log_to!(logger, Info, [GST] => "VA-API detected. Using vaapih264enc for AMD/Intel GPU.");
        (EncoderType::Vaapi, "vaapih264enc")
    } else {
        log_to!(logger, Warn, [GST] => "No hardware encoder detected, falling back to software x264enc.");
        (EncoderType::Software, "x264enc")
    }
}

pub async fn build_and_configure_pipeline(
    settings: &Settings,
    pipewire_fd: &std::os::unix::prelude::RawFd,
    node_id: u32,
    ring_buffer: Arc<Mutex<RingBuffer>>,
    logger: &Logger,
) -> anyhow::Result<gst::Element> {
    let mut pipeline_parts = Vec::new();
    let has_audio = settings.include_bg_audio || settings.include_mic_audio;
    pipeline_parts.push("matroskamux name=mux ! appsink name=sink".to_string());

    pipeline_parts.clear();

    pipeline_parts
        .push("matroskamux name=mux ! queue max-size-buffers=2 ! appsink name=sink".to_string());

    let (width, height) = {
        let parts: Vec<&str> = settings.clip_resolution.split('x').collect();
        if parts.len() == 2 {
            let w = parts[0].parse::<i32>().unwrap_or(1920);
            let h = parts[1].parse::<i32>().unwrap_or(1080);
            (w, h)
        } else {
            log_to!(logger, Warn, [DAEMON] => "Invalid video_resolution format '{}'. Using default 1920x1080.", settings.clip_resolution);
            (1920, 1080)
        }
    };

    log_to!(logger, Info, [GST] => "Setting output resolution to {}x{}", width, height);

    let (encoder_type, encoder_name) = detect_encoder(logger);

    let video_encoder_pipeline = match encoder_type {
        EncoderType::Nvidia => {
            log_to!(logger, Info, [GST] => "Using bitrate: {} kbps for nvh264enc", settings.video_bitrate);
            format!(
                "cudaupload ! {encoder_name} bitrate={} rc-mode=cbr ! h264parse config-interval=-1",
                settings.video_bitrate
            )
        }
        EncoderType::Vaapi => {
            log_to!(logger, Info, [GST] => "Using bitrate: {} kbps for vaapih264enc", settings.video_bitrate);
            format!(
                "{encoder_name} bitrate={} ! h264parse config-interval=-1",
                settings.video_bitrate
            )
        }
        EncoderType::Software => {
            log_to!(logger, Info, [GST] => "Using bitrate: {} kbps for x264enc", settings.video_bitrate);
            format!(
                "{encoder} bitrate={bitrate} speed-preset=ultrafast tune=zerolatency ! h264parse config-interval=-1",
                encoder = encoder_name,
                bitrate = settings.video_bitrate
            )
        }
    };

    pipeline_parts.push(format!(
        "pipewiresrc do-timestamp=true fd={fd} path={path} ! \
        queue max-size-buffers=8 leaky=downstream ! \
        videoconvert ! videoscale ! \
        video/x-raw,width={width},height={height},format=(string)NV12 ! \
        videorate ! video/x-raw,framerate={fps}/1 ! \
        queue max-size-buffers=8 leaky=downstream ! \
        {video_encoder_pipeline} ! queue ! mux.video_0",
        fd = *pipewire_fd,
        path = node_id,
        fps = settings.clip_fps,
        width = width,
        height = height,
        video_encoder_pipeline = video_encoder_pipeline,
    ));

    if has_audio {
        pipeline_parts
            .push("audiomixer name=mix ! audioconvert ! audio/x-raw,channels=2 ! opusenc ! opusparse ! queue ! mux.audio_0".to_string());

        if settings.include_bg_audio {
            log_to!(logger, Info,
                [GST] => "Enabling DESKTOP audio recording for device {}",
                settings.bg_node_name
            );
            match get_pipewire_node_id(&settings.bg_node_name, logger).await {
                Ok(bg_node_id) => {
                    pipeline_parts.push(format!(
                        "pipewiresrc do-timestamp=true path={bg_node_id} ! \
                        queue ! \
                        audio/x-raw,rate=48000,channels=2 ! \
                        audioconvert ! audioresample ! mix.sink_0",
                    ));
                }
                Err(e) => {
                    log_to!(logger, Error, [GST] => "Could not find monitor source '{}': {}. Background audio will not be recorded.", settings.bg_node_name, e);
                }
            }
        }

        if settings.include_mic_audio {
            log_to!(logger, Info,
                [GST] => "Enabling MICROPHONE audio recording for device {}",
                settings.mic_node_name
            );
            match get_pipewire_node_id(&settings.mic_node_name, logger).await {
                Ok(mic_node_id) => {
                    pipeline_parts.push(format!(
                        "pipewiresrc do-timestamp=true path={mic_node_id} ! \
                        queue ! \
                        audio/x-raw,rate=48000,channels=2 ! \
                        audioconvert ! audioresample ! mix.sink_1",
                    ));
                }
                Err(e) => {
                    log_to!(logger, Error, [GST] => "Could not find microphone source '{}': {}. Mic audio will not be recorded.", settings.mic_node_name, e);
                }
            }
        }
    }

    let pipeline_str = pipeline_parts.join(" ");

    log_to!(logger, Info, [GST] => "Parsing pipeline: {}", pipeline_str);
    let pipeline = gst::parse::launch(&pipeline_str)
        .map_err(|e| anyhow::anyhow!("Failed to parse pipeline: {}", e))?;

    let pipeline_bin = pipeline
        .clone()
        .dynamic_cast::<gst::Bin>()
        .expect("Pipeline should be a Bin");

    if settings.include_bg_audio {
        let desktop_vol = settings.bg_volume as f64 / 100.0;
        let sink_0_pad = pipeline_bin
            .by_name("mix")
            .expect("Failed to get mixer")
            .static_pad("sink_0")
            .expect("Failed to get mixer sink_0");
        sink_0_pad.set_property("volume", desktop_vol);
        log_to!(logger, Info, [GST] => "Set desktop audio volume to {}", desktop_vol);
    }

    if settings.include_mic_audio {
        let mic_vol = settings.mic_volume as f64 / 100.0;
        let sink_1_pad = pipeline_bin
            .by_name("mix")
            .expect("Failed to get mixer")
            .static_pad("sink_1")
            .expect("Failed to get mixer sink_1");
        sink_1_pad.set_property("volume", mic_vol);
        log_to!(logger, Info, [GST] => "Set mic audio volume to {}", mic_vol);
    }

    let appsink = pipeline_bin
        .by_name("sink")
        .expect("Failed to get appsink")
        .dynamic_cast::<AppSink>()
        .expect("Failed to cast to appsink");

    appsink.set_property("drop", true);
    appsink.set_property("max-buffers", 5u32);

    let rb_clone = ring_buffer.clone();
    let logger_clone_for_callback = logger.clone();
    appsink.set_callbacks(
        gstreamer_app::AppSinkCallbacks::builder()
            .new_sample(move |sink| {
                let sample = sink.pull_sample().map_err(|_| gst::FlowError::Eos)?;
                let buffer = sample.buffer().ok_or(gst::FlowError::Error)?;
                let pts = buffer.pts();
                let is_header = buffer.flags().contains(gst::BufferFlags::HEADER);
                let map = buffer.map_readable().map_err(|_| gst::FlowError::Error)?;
                let data = map.as_slice().to_vec();

                log_to!(logger_clone_for_callback, Debug, [DEBUG] => "Writing chunk to file. PTS: {:?}, Size: {}, IsHeader: {}", pts, data.len(), is_header);
                if let Ok(mut rb) = rb_clone.try_lock() {
                    rb.push(data, is_header, pts);
                } else {
                    log_to!(logger_clone_for_callback, Warn, [RING] => "Failed to acquire lock on ring buffer, frame dropped.");
                }

                Ok(gst::FlowSuccess::Ok)
            })
            .build(),
    );

    Ok(pipeline)
}

async fn setup_screencast<'a>(
    proxy: &'a Screencast<'a>,
    token_file: &std::path::Path,
    logger: &Logger,
) -> Result<(Session<'a, Screencast<'a>>, u32, std::os::unix::io::OwnedFd), ashpd::Error> {
    let restore_token = match read_to_string(token_file) {
        Ok(token) => {
            log_to!(logger, Info, [ASH] => "Loaded existing restore token from '{}'", token_file.display());
            Some(token)
        }
        Err(e) => {
            log_to!(logger, Warn, [ASH] => "No restore token found at '{}': {}", token_file.display(), e);
            None
        }
    };

    let mut retry_count = 0;
    let source_type = SourceType::Monitor;
    let mut last_error = None;

    while retry_count < MAX_PORTAL_RETRIES {
        log_to!(logger, Info, [ASH] => "Attempting to create session with source type: {:?}", source_type);
        match proxy.create_session().await {
            Ok(session) => {
                log_to!(logger, Info, [ASH] => "Selecting screencast sources with restore token: {:?}", restore_token);
                match proxy
                    .select_sources(
                        &session,
                        CursorMode::Hidden,
                        enumflags2::BitFlags::from(source_type),
                        false,
                        restore_token.as_deref(),
                        PersistMode::Application,
                    )
                    .await
                {
                    Ok(_request) => {
                        log_to!(logger, Info, [ASH] => "Starting screencast session");
                        match proxy.start(&session, None).await {
                            Ok(response) => {
                                let response = response.response()?;
                                let stream = response.streams().first().ok_or_else(|| {
                                    ashpd::Error::Portal(ashpd::PortalError::Failed(
                                        "No streams found in response".to_string(),
                                    ))
                                })?;
                                let node_id = stream.pipe_wire_node_id();
                                log_to!(logger, Info, [ASH] => "Streams: {:?}", stream);
                                if stream.source_type() != Some(SourceType::Monitor) {
                                    log_to!(logger, Warn, [ASH] => "Expected source type Monitor, got {:?}", stream.source_type());
                                }

                                if let Some(new_token) = response.restore_token() {
                                    log_to!(logger, Info, [ASH] => "New restore token received, saving to '{}'", token_file.display());
                                    if let Err(e) = write(token_file, new_token) {
                                        log_to!(logger, Error, [ASH] => "Failed to save restore token to '{}': {}", token_file.display(), e);
                                    } else {
                                        log_to!(logger, Info, [ASH] => "Successfully saved restore token");
                                    }
                                } else {
                                    log_to!(logger, Warn, [ASH] => "Portal did not provide a restore token for source type {:?}.", source_type);
                                    log_to!(logger, Debug, [ASH] => "Response details: {:?}", response);
                                }

                                let pipewire_fd = proxy.open_pipe_wire_remote(&session).await?;
                                log_to!(logger, Info, [ASH] => "Pipewire fd: {:?}", pipewire_fd.as_raw_fd());
                                return Ok((session, node_id, pipewire_fd));
                            }
                            Err(e) => {
                                retry_count += 1;
                                last_error = Some(e);
                                log_to!(logger, Error, [ASH] => "Failed to start screencast session (attempt {}/{}): {}", retry_count, MAX_PORTAL_RETRIES, last_error.as_ref().unwrap());
                                tokio::time::sleep(Duration::from_millis(500)).await;
                            }
                        }
                    }
                    Err(e) => {
                        retry_count += 1;
                        last_error = Some(e);
                        log_to!(logger, Error, [ASH] => "Failed to select sources (attempt {}/{}): {}", retry_count, MAX_PORTAL_RETRIES, last_error.as_ref().unwrap());
                        tokio::time::sleep(Duration::from_millis(500)).await;
                    }
                }
            }
            Err(e) => {
                retry_count += 1;
                last_error = Some(e);
                log_to!(logger, Error, [ASH] => "Failed to create session (attempt {}/{}): {}", retry_count, MAX_PORTAL_RETRIES, last_error.as_ref().unwrap());
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
        }
    }

    Err(last_error.unwrap_or_else(|| {
        ashpd::Error::Portal(ashpd::PortalError::Failed(format!(
            "Failed to set up screencast after {MAX_PORTAL_RETRIES} retries",
        )))
    }))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let settings = Settings::load().await?;
    let log_dir = "/tmp/wayclip";
    create_dir_all(log_dir).expect("Failed to create log directory");
    let timestamp = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();

    let logger = Logger::new(format!("{log_dir}/wayclip-{timestamp}.log"))
        .expect("Failed to create daemon logger");

    log_to!(logger, Info, [DAEMON] => "Starting...");
    log_to!(logger, Debug, [DAEMON] => "Settings loaded: {:?}", settings);

    env::set_var(
        "GST_DEBUG",
        "pipewiresrc:4,audiomixer:4,audioconvert:4,audioresample:4,opusenc:4,matroskamux:4,3",
    );
    gst::init().expect("Failed to init gstreamer");
    if metadata(&settings.daemon_socket_path).is_ok() {
        if let Err(e) = remove_file(&settings.daemon_socket_path) {
            log_to!(logger, Error, [UNIX] => "Failed to remove existing daemon socket file: {}", e);
            exit(1);
        }
    }

    // Log the GUI socket path for debugging
    log_to!(logger, Debug, [DAEMON] => "GUI socket path: {}", settings.gui_socket_path);

    send_status_to_gui(
        settings.gui_socket_path.clone(),
        String::from("Starting"),
        &logger,
    );

    let listener =
        UnixListener::bind(&settings.daemon_socket_path).expect("Failed to bind unix socket");

    setup_trigger(&settings.trigger_path, &logger).await;

    let proxy = Screencast::new()
        .await
        .expect("Failed to create screencast proxy");

    let token_dir = config_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not find config dir"))?
        .join("wayclip");
    let token_file = token_dir.join("portal-restore-token");

    if let Err(e) = create_dir_all(&token_dir) {
        log_to!(logger, Error, [ASH] => "Failed to create token directory '{}': {}", token_dir.display(), e);
        return Err(e.into());
    }

    let (session, node_id, pipewire_fd) = setup_screencast(&proxy, &token_file, &logger)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to set up screencast: {}", e))?;

    tokio::time::sleep(Duration::from_millis(1000)).await;

    let clip_duration = gst::ClockTime::from_seconds(settings.clip_length_s);
    let ring_buffer = Arc::new(Mutex::new(RingBuffer::new(clip_duration, &logger)));
    let is_saving = Arc::new(AtomicBool::new(false));

    let mut pipeline = build_and_configure_pipeline(
        &settings,
        &pipewire_fd.as_raw_fd(),
        node_id,
        ring_buffer.clone(),
        &logger,
    )
    .await?;

    tokio::spawn(handle_bus_messages(
        pipeline.clone().dynamic_cast::<gst::Pipeline>().unwrap(),
        logger.clone(),
    ));

    log_to!(logger, Info, [GST] => "Setting pipeline to playing for constant recording");
    let max_retries = 3u32;
    let mut retry_count = 0;
    loop {
        match pipeline.set_state(gst::State::Playing) {
            Ok(_) => {
                log_to!(logger, Info, [GST] => "Pipeline transitioned to Playing successfully.");
                break;
            }
            Err(err) => {
                retry_count += 1;
                log_to!(logger, Error, [GST] => "Pipeline state change failed (attempt {}/{}): {}. Retrying...", retry_count, max_retries, err);
                if retry_count >= max_retries {
                    log_to!(logger, Error, [GST] => "Max retries exceeded. Shutting down.");
                    let _ = pipeline.set_state(gst::State::Null);
                    return Err(anyhow::anyhow!("Pipeline state change failed: {}", err));
                }
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
        }
    }

    let _ = sd_notify::notify(true, &[NotifyState::Ready]);
    log_to!(logger, Info, [DAEMON] => "Service is ready and recording.");

    send_status_to_gui(
        settings.gui_socket_path.clone(),
        String::from("Recording"),
        &logger,
    );

    let (tx, mut rx): (Sender<String>, Receiver<String>) = channel(32);

    let listener_logger = logger.clone();
    tokio::spawn(async move {
        loop {
            if let Ok((stream, _)) = listener.accept().await {
                let mut reader = BufReader::new(stream);
                let mut buf = String::new();
                loop {
                    buf.clear();
                    match reader.read_line(&mut buf).await {
                        Ok(0) => break,
                        Ok(_) => {
                            let msg = buf.trim().to_string();
                            log_to!(listener_logger, Info, [UNIX] => "Message received: {}", msg);
                            if tx.send(msg).await.is_err() {
                                log_to!(listener_logger, Error, [UNIX] => "Receiver dropped, cannot send message.");
                                break;
                            }
                        }
                        Err(e) => {
                            log_to!(listener_logger, Error, [UNIX] => "Failed to read from socket: {}", e);
                            break;
                        }
                    }
                }
            }
        }
    });

    let job_id_counter = Arc::new(AtomicUsize::new(1));
    let mut last_save_time = Instant::now() - SAVE_COOLDOWN;
    let mut term_signal =
        signal(SignalKind::terminate()).expect("Failed to install SIGTERM handler");
    loop {
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {
                log_to!(logger, Info, [DAEMON] => "Ctrl+C received, initiating shutdown.");
                break;
            },
            _ = term_signal.recv() => {
                log_to!(logger, Info, [DAEMON] => "SIGTERM received, initiating shutdown.");
                break;
            },

            Some(msg) = rx.recv() => {
                match msg.as_str() {
                    "save" => {
                        if last_save_time.elapsed() < SAVE_COOLDOWN {
                            log_to!(logger, Warn, [UNIX] => "Ignoring save request: Cooldown active.");
                            continue;
                        }
                        send_status_to_gui(settings.gui_socket_path.clone(), String::from("Saving clip..."), &logger);

                        if is_saving.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
                            last_save_time = Instant::now();
                            let job_id = job_id_counter.fetch_add(1, Ordering::SeqCst);
                            log_to!(logger, Info, [UNIX] => "[JOB {}] Save command received, starting process.", job_id);

                            log_to!(logger, Info, [GST] => "[JOB {}] Stopping pipeline to save clip.", job_id);
                            pipeline.set_state(gst::State::Null)?;
                            log_to!(logger, Info, [GST] => "[JOB {}] Pipeline stopped.", job_id);

                            let saved_chunks = {
                                let mut rb = ring_buffer.lock().unwrap();
                                rb.get_and_clear()
                            };

                            let is_saving_clone = is_saving.clone();
                            let settings_clone = settings.clone();
                            let ffmpeg_logger = logger.clone();
                            tokio::spawn(async move {
                                log_to!(ffmpeg_logger, Info, [FFMPEG] => "[JOB {}] Spawning to save {} Matroska chunks.", job_id, saved_chunks.len());

                                if saved_chunks.is_empty() {
                                    log_to!(ffmpeg_logger, Warn, [FFMPEG] => "[JOB {}] No chunks in buffer. Aborting save.", job_id);
                                    is_saving_clone.store(false, Ordering::SeqCst);
                                    return;
                                }

                                let home_dir = env::var("HOME").expect("HOME not set");
                                let output_dir = std::path::Path::new(&home_dir).join(&settings_clone.save_path_from_home_string);
                                create_dir_all(&output_dir).expect("Failed to create output directory");
                                let output_filename = output_dir.join(format!("{}.mp4", chrono::Local::now().format(&settings_clone.clip_name_formatting)));

                                let mut ffmpeg_child = Command::new("ffmpeg").args([
                                    "-y",
                                    "-i",
                                    "-",
                                    "-c:v",
                                    "copy",
                                    "-c:a",
                                    "copy",
                                    output_filename.to_str().unwrap()
                                ])
                                    .stdin(Stdio::piped())
                                    .stdout(Stdio::null())
                                    .stderr(Stdio::piped())
                                    .spawn()
                                    .expect("Failed to spawn ffmpeg");
                                let mut stdin = ffmpeg_child.stdin.take().expect("Failed to get ffmpeg stdin");
                                let mut stderr = BufReader::new(ffmpeg_child.stderr.take().unwrap());

                                let logger_for_stderr = ffmpeg_logger.clone();
                                tokio::spawn(async move {
                                    let mut error_output = String::new();
                                    use tokio::io::AsyncReadExt;
                                    stderr.read_to_string(&mut error_output).await.unwrap();
                                    if !error_output.is_empty() {
                                        log_to!(logger_for_stderr, Warn, [FFMPEG] => "[JOB {}] {}", job_id, error_output.trim());
                                    }
                                });

                                for chunk in saved_chunks {
                                    if let Err(e) = stdin.write_all(&chunk).await {
                                        log_to!(ffmpeg_logger, Error, [FFMPEG] => "[JOB {}] Process failed while writing chunks: {}", job_id, e);
                                        break;
                                    }
                                }
                                drop(stdin);

                                match ffmpeg_child.wait().await {
                                    Ok(status) if status.success() => {
                                        log_to!(ffmpeg_logger, Info, [FFMPEG] => "[JOB {}] Done! Saved to {:?}", job_id, output_filename);
                                        let gui_path = settings_clone.gui_socket_path.clone();
                                        let ffmpeg_logger_clone = ffmpeg_logger.clone();
                                        tokio::spawn(async move {
                                            if let Err(e) = generate_preview_clip(&output_filename, &Settings::config_path().join("wayclip").join("previews")).await {
                                                log_to!(&ffmpeg_logger_clone, Error, [FFMPEG] => "Failed to generate preview, {}", e)
                                            };
                                            send_status_to_gui(gui_path, String::from("Saved!"), &ffmpeg_logger_clone);
                                        });
                                    },
                                    Ok(status) => {
                                        log_to!(ffmpeg_logger, Error, [FFMPEG] => "[JOB {}] Exited with error: {}", job_id, status);
                                        send_status_to_gui(settings_clone.gui_socket_path.clone(), String::from("Error during saving"), &ffmpeg_logger);
                                    },
                                    Err(e) => {
                                        log_to!(ffmpeg_logger, Error, [FFMPEG] => "[JOB {}] Process failed: {}", job_id, e);
                                    }
                                }
                                is_saving_clone.store(false, Ordering::SeqCst);
                                log_to!(ffmpeg_logger, Info, [FFMPEG] => "[JOB {}] Task finished and save lock released.", job_id);
                            });

                            log_to!(logger, Info, [GST] => "Rebuilding and restarting pipeline post-save...");
                            pipeline = build_and_configure_pipeline(
                                &settings,
                                &pipewire_fd.as_raw_fd(),
                                node_id,
                                ring_buffer.clone(),
                                &logger,
                            ).await?;
                            pipeline.set_state(gst::State::Playing)?;
                            tokio::spawn(handle_bus_messages(pipeline.clone().dynamic_cast::<gst::Pipeline>().unwrap(), logger.clone()));
                            log_to!(logger, Info, [GST] => "Pipeline restarted successfully.");
                            send_status_to_gui(settings.gui_socket_path.clone(), "Recording".into(), &logger);

                        } else {
                            log_to!(logger, Warn, [UNIX] => "Ignoring save request: A save is already in progress.");
                        }
                    }
                    "exit" => {
                        log_to!(logger, Info, [UNIX] => "Exit command received, initiating shutdown.");
                        break;
                    }
                    _ => {
                        log_to!(logger, Warn, [UNIX] => "Unknown message received: {}", msg);
                    }
                }
            },
            else => {
                log_to!(logger, Warn, [DAEMON] => "Listener channel closed. Shutting down.");
                break;
            }
        }
    }

    let _ = sd_notify::notify(true, &[NotifyState::Stopping]);
    cleanup(&pipeline, &session, settings, logger).await;
    Ok(())
}
