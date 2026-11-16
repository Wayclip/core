The templates provided will be installed by package manager.

```sh
install -Dm644 assets/wayclip-daemon.service \
  "$HOME/.config/systemd/user/wayclip-daemon.service"
install -Dm644 assets/com.wayclip.daemon.desktop \
  "$HOME/.local/share/applications/com.wayclip.daemon.desktop"
systemctl --user daemon-reload
systemctl --user enable --now wayclip-daemon
```
