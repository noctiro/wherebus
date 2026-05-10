<div align="center">

# WhereBus

开源实时公交到站查询

[![License: AGPL-3.0](https://img.shields.io/badge/License-AGPL--3.0-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)
[![Android](https://img.shields.io/badge/Android-7.0%2B-green.svg)](https://developer.android.com)
[![Rust](https://img.shields.io/badge/Rust-2024-orange.svg)](https://www.rust-lang.org)

</div>

## 功能

- 附近站点自动发现，按距离排序
- 线路实时到站信息（到站时间、距离、站数）
- 线路详情与全程站点时间轴
- 实时车辆位置追踪与拥堵状态显示
- 多信息源、多提供者，可扩展接入不同城市数据源
- 多城市切换
- 离线缓存，弱网可用

## 截图

<!-- TODO: 添加截图 -->

## 系统要求

- Android 7.0 (API 24) 及以上

## 架构

- `core/` — Rust 共享核心（Crux 架构），包含业务逻辑、数据缓存、provider 抽象
- `android/` — Android 客户端（Jetpack Compose）

核心通过 UniFFI 桥接暴露给 Android 端，UI 层负责渲染和事件转发。

## 构建

### Core

```
cargo build
```

### Android

用 Android Studio 打开 `android/` 目录，或：

```
cd android && ./gradlew assembleDebug
```

需要 NDK 和 Rust Android targets（`aarch64-linux-android` 等）。

## 支持的数据源

- 掌上公交 (mygolbs)
- 车来了 (chelaile)

通过 provider trait 抽象，可扩展其他城市数据源。

## 致谢

本项目的开发离不开以下优秀的开源项目，在此向它们的开发者和社区表示衷心的感谢！

- [Crux](https://github.com/redbadger/crux) — 跨平台应用架构框架

## License

AGPL-3.0
