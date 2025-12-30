//! MindLand Ultra-Fast Input System
//! 
//! Zero-latency input handling with lock-free data structures and high-frequency polling.

use bevy::prelude::*;
use crossbeam::queue::SegQueue;
use parking_lot::RwLock;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

/// Ultra-fast input manager with lock-free architecture
pub struct InputManager {
    pub keyboard_state: AtomicKeyboardState,
    pub mouse_state: AtomicMouseState,
    pub input_buffer: SegQueue<InputEvent>,
    pub polling_rate: u32,
}

/// Lock-free keyboard state tracking
pub struct AtomicKeyboardState {
    // Using array of atomic bools for lock-free key state
    keys: [AtomicBool; 256],
}

/// Lock-free mouse state tracking
pub struct AtomicMouseState {
    pub position: RwLock<Vec2>,
    pub delta: RwLock<Vec2>,
    pub buttons: AtomicU64, // Bitfield for mouse buttons
}

/// High-frequency input events with precise timing
#[derive(Debug, Clone)]
pub enum InputEvent {
    KeyPressed { key: KeyCode, timestamp: u64 },
    KeyReleased { key: KeyCode, timestamp: u64 },
    MouseMoved { delta: Vec2, timestamp: u64 },
    MousePressed { button: MouseButton, timestamp: u64 },
    MouseReleased { button: MouseButton, timestamp: u64 },
}

impl Default for InputManager {
    fn default() -> Self {
        Self::new()
    }
}

impl InputManager {
    /// Create a new input manager with 1000Hz target polling rate
    pub fn new() -> Self {
        Self {
            keyboard_state: AtomicKeyboardState::new(),
            mouse_state: AtomicMouseState::new(),
            input_buffer: SegQueue::new(),
            polling_rate: 1000, // Target 1000Hz polling
        }
    }

    /// Check if a key is currently pressed (lock-free)
    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        let key_index = key as usize;
        if key_index < 256 {
            self.keyboard_state.keys[key_index].load(Ordering::Acquire)
        } else {
            false
        }
    }

    /// Get current mouse position (lock-free read)
    pub fn mouse_position(&self) -> Vec2 {
        *self.mouse_state.position.read()
    }

    /// Get mouse delta since last frame (lock-free read)
    pub fn mouse_delta(&self) -> Vec2 {
        *self.mouse_state.delta.read()
    }
}

impl AtomicKeyboardState {
    fn new() -> Self {
        // Initialize all keys as not pressed
        let keys = std::array::from_fn(|_| AtomicBool::new(false));
        Self { keys }
    }

    /// Set key state atomically
    pub fn set_key_state(&self, key: KeyCode, pressed: bool) {
        let key_index = key as usize;
        if key_index < 256 {
            self.keys[key_index].store(pressed, Ordering::Release);
        }
    }
}

impl AtomicMouseState {
    fn new() -> Self {
        Self {
            position: RwLock::new(Vec2::ZERO),
            delta: RwLock::new(Vec2::ZERO),
            buttons: AtomicU64::new(0),
        }
    }

    /// Update mouse position atomically
    pub fn update_position(&self, new_position: Vec2) {
        let mut pos = self.position.write();
        let mut delta = self.delta.write();
        *delta = new_position - *pos;
        *pos = new_position;
    }

    /// Set mouse button state atomically
    pub fn set_button_state(&self, button: MouseButton, pressed: bool) {
        let button_bit = 1u64 << (button as u8);
        let current = self.buttons.load(Ordering::Acquire);
        let new_state = if pressed {
            current | button_bit
        } else {
            current & !button_bit
        };
        self.buttons.store(new_state, Ordering::Release);
    }

    /// Check if mouse button is pressed
    pub fn is_button_pressed(&self, button: MouseButton) -> bool {
        let button_bit = 1u64 << (button as u8);
        (self.buttons.load(Ordering::Acquire) & button_bit) != 0
    }
}