FROM rust:1-slim-buster

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    libglib2.0-dev \
    libwayland-dev \
    libpipewire-0.3-dev \
    libgstreamer1.0-dev \
    libgstreamer-plugins-base1.0-dev \
    libavcodec-dev \
    libavformat-dev \
    libavutil-dev \
    libswscale-dev \
    libavfilter-dev \
    libavdevice-dev \
    libswresample-dev \
    libasound2-dev \
    libdbus-1-dev \
    clang \
    git \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/wayclip

COPY . .

RUN cargo build --release --bins --target x86_64-unknown-linux-gnu

RUN mkdir -p /out && cp target/x86_64-unknown-linux-gnu/release/* /out/
