use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef, ShaderType},
};

impl Material for TileMaterial {
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
}

impl Default for InkSettings {
    fn default() -> Self {
        Self {
            base_color: Color::WHITE,
            ink_color: Color::rgb_u8(130, 127, 106),
            parchment_base: Color::rgb_u8(203, 172, 113),
            parchment_burn: Color::rgba_u8(146, 90, 45, 255),
            parchment_dark: Color::rgba_u8(56, 25, 25, 255),
            added_params: Vec4::new(1., 1., 0.5, 0.1),
        }
    }
}

// This is the struct that will be passed to your shader
#[derive(AsBindGroup, TypeUuid, Debug, Clone, Default, Reflect)]
#[uuid = "0896d435-17d3-48c9-a698-3a4d6291019f"]
pub struct TileMaterial {
    #[uniform(0)]
    pub settings: InkSettings,
}
