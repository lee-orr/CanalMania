use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
};

impl Material for TileMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/tile_shader.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Opaque
    }
}

// This is the struct that will be passed to your shader
#[derive(AsBindGroup, TypeUuid, Debug, Clone, Default)]
#[uuid = "0896d435-17d3-48c9-a698-3a4d6291019f"]
pub struct TileMaterial {
    #[uniform(0)]
    pub base_color: Color,
    #[uniform(1)]
    pub ink_color: Color,
    #[uniform(2)]
    pub world_darkening: f32,
    #[uniform(3)]
    pub vertex_color_strength: f32,
}
