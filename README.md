# MindLand - Ultra-High Performance Voxel Sandbox

MindLand is a next-generation voxel sandbox game built in Rust with Bevy ECS, designed to outperform Minecraft by 300% while maintaining 60 FPS on MacBook Pro 2014 hardware.

## Project Structure

This is a Cargo workspace with the following crates:

- **`mindland_app`** - Main application and engine initialization
- **`mindland_window`** - Cross-platform window management with optimized graphics backends
- **`mindland_input`** - Ultra-fast input handling with lock-free data structures
- **`mindland_camera`** - Smooth first-person camera with quaternion-based rotation
- **`mindland_render`** - Ultra-optimized 3D rendering pipeline with GPU acceleration
- **`mindland_assets`** - High-performance asset management with LRU caching
- **`mindland_performance`** - Real-time performance monitoring and thermal management

## Performance Goals

- **3x better performance than Minecraft** on equivalent hardware
- **60 FPS guarantee** on MacBook Pro 2014 with silent fans
- **Sub-millisecond input latency** for competitive-level responsiveness
- **Zero-allocation hot paths** for consistent frame times
- **SIMD-optimized mathematics** for maximum CPU efficiency

## Build Configuration

The project uses aggressive optimization settings:

- **LTO (Link Time Optimization)** for maximum performance
- **Native CPU targeting** for platform-specific optimizations
- **SIMD acceleration** for mathematical operations
- **Zero-cost abstractions** throughout the codebase

## Getting Started

1. Ensure you have Rust 1.70+ installed
2. Clone the repository
3. Run `cargo build --release` for optimized builds
4. Run `cargo run --release` to start the engine

## Development

- Use `cargo check` for fast compilation checks
- Use `cargo test` to run the test suite
- Use `cargo bench` for performance benchmarking
- Use the `macbook` profile for MacBook Pro 2014 optimization: `cargo build --profile macbook`

## Architecture

Built on Bevy's high-performance ECS framework with:

- Modular crate structure for clean separation of concerns
- Cross-platform compatibility (macOS, Windows, Linux)
- Automatic graphics backend selection (Metal, DirectX12, Vulkan)
- Real-time performance monitoring and automatic optimization
- Hardware-specific optimization presets

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.