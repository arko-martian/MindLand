//! MindLand Asset Management System
//! 
//! High-performance asset loading and caching with LRU cache and async loading.

use bevy::{
    prelude::*,
    render::render_resource::TextureFormat,
};
use slotmap::{SlotMap, DefaultKey};
use lru::LruCache;
use std::collections::VecDeque;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU32, Ordering};
use anyhow::Result;
use thiserror::Error;

/// High-performance asset manager with LRU caching
pub struct AssetManager {
    pub textures: SlotMap<TextureId, ManagedTexture>,
    pub meshes: SlotMap<MeshId, ManagedMesh>,
    pub materials: SlotMap<MaterialId, ManagedMaterial>,
    pub asset_cache: LruCache<AssetPath, AssetId>,
    pub loading_queue: VecDeque<AssetLoadRequest>,
}

/// Unique identifiers for different asset types
pub type TextureId = DefaultKey;
pub type MeshId = DefaultKey;
pub type MaterialId = DefaultKey;

/// Generic asset identifier
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum AssetId {
    Texture(TextureId),
    Mesh(MeshId),
    Material(MaterialId),
}

/// Asset path for cache lookup
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct AssetPath {
    pub path: PathBuf,
    pub asset_type: AssetType,
}

/// Supported asset types
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum AssetType {
    Texture,
    Mesh,
    Material,
}

/// Managed texture with usage tracking
pub struct ManagedTexture {
    pub handle: Handle<Image>,
    pub size: (u32, u32),
    pub format: TextureFormat,
    pub mip_levels: u32,
    pub usage_count: AtomicU32,
    pub path: PathBuf,
}

/// Managed mesh with bounding information
pub struct ManagedMesh {
    pub handle: Handle<Mesh>,
    pub vertex_count: u32,
    pub index_count: u32,
    pub bounding_box: BoundingBox,
    pub usage_count: AtomicU32,
    pub path: PathBuf,
}

/// Managed material with shader information
pub struct ManagedMaterial {
    pub handle: Handle<StandardMaterial>,
    pub shader_type: ShaderType,
    pub usage_count: AtomicU32,
    pub path: PathBuf,
}

/// Shader type for material optimization
#[derive(Debug, Clone, Copy)]
pub enum ShaderType {
    Standard,
    Unlit,
    PBR,
    Custom(u32),
}

/// Bounding box for culling optimization
#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
    pub min: Vec3,
    pub max: Vec3,
}

/// Asset loading request for async processing
#[derive(Debug, Clone)]
pub struct AssetLoadRequest {
    pub path: AssetPath,
    pub priority: LoadPriority,
}

/// Loading priority for asset queue management
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LoadPriority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
}

/// Asset loading errors
#[derive(Error, Debug)]
pub enum AssetError {
    #[error("Asset not found: {path}")]
    NotFound { path: PathBuf },
    #[error("Unsupported format: {format}")]
    UnsupportedFormat { format: String },
    #[error("Loading failed: {reason}")]
    LoadingFailed { reason: String },
    #[error("Cache full")]
    CacheFull,
}

impl Default for AssetManager {
    fn default() -> Self {
        Self::new()
    }
}

impl AssetManager {
    /// Create a new asset manager with default cache size
    pub fn new() -> Self {
        Self::with_cache_size(1000) // Default 1000 asset cache
    }

    /// Create asset manager with custom cache size
    pub fn with_cache_size(cache_size: usize) -> Self {
        Self {
            textures: SlotMap::new(),
            meshes: SlotMap::new(),
            materials: SlotMap::new(),
            asset_cache: LruCache::new(cache_size.try_into().unwrap()),
            loading_queue: VecDeque::new(),
        }
    }

    /// Load a texture asset (returns cached version if available)
    pub fn load_texture(&mut self, path: PathBuf) -> Result<TextureId, AssetError> {
        let asset_path = AssetPath {
            path: path.clone(),
            asset_type: AssetType::Texture,
        };

        // Check cache first
        if let Some(AssetId::Texture(texture_id)) = self.asset_cache.get(&asset_path).cloned() {
            if let Some(texture) = self.textures.get(texture_id) {
                texture.usage_count.fetch_add(1, Ordering::Relaxed);
                return Ok(texture_id);
            }
        }

        // Load new texture (placeholder implementation)
        let texture_id = self.textures.insert(ManagedTexture {
            handle: Handle::default(), // Would load actual texture in full implementation
            size: (256, 256), // Placeholder
            format: TextureFormat::Rgba8UnormSrgb,
            mip_levels: 1,
            usage_count: AtomicU32::new(1),
            path: path.clone(),
        });

        // Cache the loaded asset
        self.asset_cache.put(asset_path, AssetId::Texture(texture_id));

        Ok(texture_id)
    }

    /// Queue an asset for async loading
    pub fn queue_load(&mut self, path: AssetPath, priority: LoadPriority) {
        let request = AssetLoadRequest { path, priority };
        
        // Insert based on priority (higher priority first)
        let insert_pos = self.loading_queue
            .iter()
            .position(|req| req.priority < priority)
            .unwrap_or(self.loading_queue.len());
        
        self.loading_queue.insert(insert_pos, request);
    }

    /// Process next item in loading queue
    pub fn process_loading_queue(&mut self) -> Option<Result<AssetId, AssetError>> {
        let request = self.loading_queue.pop_front()?;
        
        // Process based on asset type
        match request.path.asset_type {
            AssetType::Texture => {
                match self.load_texture(request.path.path) {
                    Ok(texture_id) => Some(Ok(AssetId::Texture(texture_id))),
                    Err(e) => Some(Err(e)),
                }
            }
            AssetType::Mesh => {
                // TODO: Implement mesh loading
                Some(Err(AssetError::UnsupportedFormat { 
                    format: "Mesh loading not yet implemented".to_string() 
                }))
            }
            AssetType::Material => {
                // TODO: Implement material loading
                Some(Err(AssetError::UnsupportedFormat { 
                    format: "Material loading not yet implemented".to_string() 
                }))
            }
        }
    }

    /// Get texture by ID
    pub fn get_texture(&self, texture_id: TextureId) -> Option<&ManagedTexture> {
        self.textures.get(texture_id)
    }

    /// Release an asset (decrements usage count)
    pub fn release_texture(&mut self, texture_id: TextureId) {
        if let Some(texture) = self.textures.get(texture_id) {
            let usage = texture.usage_count.fetch_sub(1, Ordering::Relaxed);
            
            // Remove from cache if no longer used (optional optimization)
            if usage <= 1 {
                // Could implement automatic cleanup here
            }
        }
    }
}

impl BoundingBox {
    /// Create a new bounding box
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    /// Check if point is inside bounding box
    pub fn contains_point(&self, point: Vec3) -> bool {
        point.x >= self.min.x && point.x <= self.max.x &&
        point.y >= self.min.y && point.y <= self.max.y &&
        point.z >= self.min.z && point.z <= self.max.z
    }

    /// Get center of bounding box
    pub fn center(&self) -> Vec3 {
        (self.min + self.max) * 0.5
    }

    /// Get size of bounding box
    pub fn size(&self) -> Vec3 {
        self.max - self.min
    }
}