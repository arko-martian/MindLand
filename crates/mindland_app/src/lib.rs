//! MindLand Application Core
//! 
//! This crate provides the main application structure and engine initialization
//! for MindLand, built on Bevy's high-performance ECS framework.

use bevy::prelude::*;

/// Main MindLand application structure
pub struct MindLandApp {
    bevy_app: App,
}

/// Engine configuration for performance optimization
#[derive(Debug, Clone)]
pub struct EngineConfig {
    pub target_fps: u32,
    pub enable_vsync: bool,
    pub performance_mode: PerformanceMode,
    pub hardware_tier: HardwareTier,
}

/// Performance mode settings
#[derive(Debug, Clone, Copy)]
pub enum PerformanceMode {
    /// Maximum performance, minimal quality
    UltraPerformance,
    /// Balanced performance and quality
    Balanced,
    /// Maximum quality, performance as needed
    Quality,
    /// Optimized specifically for MacBook Pro 2014
    MacBookPro2014,
}

/// Hardware tier detection for automatic optimization
#[derive(Debug, Clone, Copy)]
pub enum HardwareTier {
    Low,
    Medium,
    High,
    UltraHigh,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            target_fps: 60,
            enable_vsync: true,
            performance_mode: PerformanceMode::Balanced,
            hardware_tier: HardwareTier::Medium,
        }
    }
}

impl MindLandApp {
    /// Create a new MindLand application with default configuration
    pub fn new() -> Self {
        Self::with_config(EngineConfig::default())
    }

    /// Create a new MindLand application with custom configuration
    pub fn with_config(config: EngineConfig) -> Self {
        let mut bevy_app = App::new();
        
        // Add core Bevy plugins with optimized settings
        bevy_app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "MindLand - Engine Boot".to_string(),
                resolution: (1920.0, 1080.0).into(),
                present_mode: if config.enable_vsync {
                    bevy::window::PresentMode::AutoVsync
                } else {
                    bevy::window::PresentMode::AutoNoVsync
                },
                ..default()
            }),
            ..default()
        }));

        // Insert configuration as a resource
        bevy_app.insert_resource(config);

        Self { bevy_app }
    }

    /// Run the MindLand application
    pub fn run(self) -> ! {
        self.bevy_app.run();
    }
}

impl Default for MindLandApp {
    fn default() -> Self {
        Self::new()
    }
}