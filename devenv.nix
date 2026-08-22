{pkgs, ...}: {
  languages = {
    rust.enable = true;
  };

  packages = with pkgs; [
    systemd
    alsa-lib
    gst_all_1.gstreamer
    gst_all_1.gst-plugins-base
  ];
}
