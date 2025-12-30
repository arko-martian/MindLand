# MindLand Assets

This directory contains all game assets organized by type:

## Directory Structure

- **`textures/`** - Block textures, UI elements, and other 2D graphics
- **`models/`** - 3D models and meshes
- **`shaders/`** - Custom GPU shaders for advanced rendering effects
- **`sounds/`** - Audio files for sound effects and music

## Asset Guidelines

- Use PNG format for textures with transparency
- Use JPG format for opaque textures to save space
- Keep texture sizes power-of-2 for optimal GPU performance
- Use texture atlases when possible to reduce draw calls
- Compress audio files appropriately for web delivery

## Performance Considerations

- All assets are loaded through the high-performance asset management system
- LRU caching ensures frequently used assets stay in memory
- Async loading prevents frame drops during asset loading
- Fallback assets are provided for missing or failed loads