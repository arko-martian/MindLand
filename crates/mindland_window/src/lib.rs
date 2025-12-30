//! MindLand Window Management
//! 
//! Cross-platform window creation and management with optimized graphics backend selection.

use bevy::prelude::*;

/// Cross-platform window manager
pub struct WindowManager {
    pub graphics_backend: GraphicsBackend,
    pub display_settings: DisplaySettings,
}

/// Graphics backend selection based on platform
#[derive(Debug, Clone, Copy)]
pub enum GraphicsBackend {
    /// Metal backend for macOS (optimal performance)
    Metal,
    /// DirectX 12 backend for Windows (high performance)
    DirectX12,
    /// Vulkan backend for Linux and high-performance Windows
    Vulkan,
    /// WebGL backend for web targets
    WebGL,
}

/// Display configuration settings
#[derive(Debug, Clone)]
pub struct DisplaySettings {
    pub resolution: (u32, u32),
    pub refresh_rate: u32,
    pub fullscreen: bool,
    pub vsync: bool,
}

impl Default for DisplaySettings {
    fn default() -> Self {
        Self {
            resolution: (1920, 1080),
            refresh_rate: 60,
            fullscreen: false,
            vsync: true,
        }
    }
}

impl GraphicsBackend {
    /// Automatically select the optimal graphics backend for the current platform
    pub fn auto_select() -> Self {
        #[cfg(target_os = "macos")]
        return Self::Metal;
        
        #[cfg(target_os = "windows")]
        return Self::DirectX12; // TODO: Add Vulkan detection for high-end hardware
        
        #[cfg(target_os = "linux")]
        return Self::Vulkan;
        
        #[cfg(target_arch = "wasm32")]
        return Self::WebGL;
        
        #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux", target_arch = "wasm32")))]
        return Self::Vulkan; // Default fallback
    }
}

impl WindowManager {
    /// Create a new window manager with auto-detected backend
    pub fn new() -> Self {
        Self {
            graphics_backend: GraphicsBackend::auto_select(),
            display_settings: DisplaySettings::default(),
        }
    }

    /// Create a window manager with custom settings
    pub fn with_settings(display_settings: DisplaySettings) -> Self {
        Self {
            graphics_backend: GraphicsBackend::auto_select(),
            display_settings,
        }
    }
}

impl Default for WindowManager {
    fn default() -> Self {
        Self::new()
    }
}