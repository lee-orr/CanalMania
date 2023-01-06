use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef, ShaderType},
};

impl Material for TileMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/tile_shader.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/tile_shader.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Opaque
    }
}

#[derive(Debug, Clone, ShaderType, Reflect)]
pub struct InkSettings {
    pub base_color: Color,
    pub ink_color: Color,
    pub parchment_base: Color,
    pub parchment_burn: Color,
    pub parchment_dark: Color,
    /// Params: world_darkening, vertex_color_strength, parchment_low_mix, parchment_high_mix
    pub added_params: Vec4,
    pub world_offset: Vec4,
    pub size: Vec4,
}

impl Default for InkSettings {
    fn default() -> Self {
        Self {
            base_color: Color::WHITE,
            ink_color: Color::rgb_u8(130, 127, 106),
            parchment_base: Color::rgb_u8(253, 231, 192),
            parchment_burn: Color::rgb_u8(180, 156, 93),
            parchment_dark: Color::rgb_u8(110, 67, 49),
            added_params: Vec4::new(1., 0.7, 0.5, 0.1),
            world_offset: Vec4::ZERO,
            size: Vec4::ZERO,
        }
    }
}

// This is the struct that will be passed to your shader
#[derive(AsBindGroup, TypeUuid, Debug, Clone, Default, Reflect)]
#[uuid = "0896d435-17d3-48c9-a698-3a4d6291019f"]
pub struct TileMaterial {
    #[uniform(0)]
    pub settings: InkSettings,
    #[texture(1)]
    #[sampler(2)]
    pub info_map: Handle<Image>,
}
