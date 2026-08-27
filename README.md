## `wayclip-core`

[![Crates.io](https://img.shields.io/crates/v/wayclip-core.svg)](https://crates.io/crates/wayclip-core)
[![Docs.rs](https://docs.rs/wayclip-core/badge.svg)](https://docs.rs/wayclip-core)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

The crate `wayclip-core` is used by the [Wayclip](https://github.com/Wayclip/) ecosystem.
This library provides useful types, models, and methods for the API & APP to use.

> **Note:** This is an internal foundation library intended for use by `wayclip-cli` and related Wayclip applications. It is not designed to be used as a standalone application.

## Libraries Required

| Package Name                 | Required? | Minimum Version | Notes                                             |
| ---------------------------- | --------- | --------------- | ------------------------------------------------- |
| `gstreamer-1.0`              | Yes       | `>= 1.14`       | Required by`gstreamer`                            |
| `gstreamer-plugins-base-1.0` | Yes       | `>= 1.14`       | Required by `gstreamer-app` & `gstreamer-pbutils` |
| `ffmpeg`                     | Yes       | `>= 4.4`        | Required by `rust_ffmpeg`                         |
| `libudev`                    | Yes       | `>= 199`        | Required by `gilrs`                               |
| `alsa-lib`                   | Yes       | `>= 1.0.27`     | Required by `rodio`                               |
| `openssl`                    | Yes       | `>= 1.1.1`      | Required by `reqwest`                             |
| `libpipewire-0.3`            | No        | `>= 0.3.0`      | Required only if `errors` feature enabled         |
| `libsecret-1`                | No        | `>= 0.18`       | Used by `keyring`                                 |

## Platforms supported

| Platform        | Status     |
| --------------- | ---------- |
| Linux (Wayland) | Supported  |
| Linux (X11)     | Supported  |
| Windows         | Not Tested |
| MacOS           | Not Tested |

## Feature flags

| Feature Flag | Description                                                                                          | Default |
| ------------ | ---------------------------------------------------------------------------------------------------- | ------- |
| `openapi`    | Enables the generation of [`utoipa`](https://docs.rs/utoipa/latest/utoipa/) types for use in OpenAPI | No      |
| `errors`     | Expands the available error variants in `WayclipError` — accounts for heavier crates                 | No      |

## License

This project is licensed under the [MIT License](LICENSE.md).
