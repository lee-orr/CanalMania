use bevy::{
    prelude::*,
    reflect::Reflect,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

use crate::game::tile_shader::{InkSettings, TileMaterial};

#[derive(Resource, Reflect)]
pub struct BoardRuntimeAssets {
    pub tile_info_map: Handle<Image>,
    pub tile_base_material: Handle<TileMaterial>,
    pub decoration_material: Handle<TileMaterial>,
    pub selector: Handle<Mesh>,
    pub selector_base: Handle<StandardMaterial>,
    pub selector_hovered: Handle<StandardMaterial>,
    pub selector_pressed: Handle<StandardMaterial>,
    pub selector_selected: Handle<StandardMaterial>,
}

pub fn setup_board_materials(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut tile_materials: ResMut<Assets<TileMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
) {
    let size = Extent3d {
        width: 1,
        height: 1,
        depth_or_array_layers: 1,
    };
    let format = TextureFormat::Rgba8Unorm;
    let data = vec![0u8, 0u8, 0u8, 0u8];
    let tile_info_map = images.add(Image::new_fill(size, TextureDimension::D2, &data, format));
    let tile_base_material = tile_materials.add(TileMaterial {
        settings: InkSettings::default(),
        info_map: tile_info_map.clone(),
    });

    let material = TileMaterial {
        settings: InkSettings {
            added_params: Vec4::new(0., 0.7, 0.5, 0.1),
            world_offset: Vec4::new(0., 0., 0., 1.),
            ..Default::default()
        },
        info_map: tile_info_map.clone(),
    };
    let decoration_material = tile_materials.add(material);

    let selector = meshes.add(shape::Box::new(1., 0.5, 1.).into());

    let selector_base = materials.add(StandardMaterial {
        base_color: Color::rgba(0., 0., 0., 0.0),
        alpha_mode: AlphaMode::Blend,
        ..Default::default()
    });

    let selector_hovered = materials.add(StandardMaterial {
        base_color: Color::rgba(0., 0.7, 0.5, 0.5),
        alpha_mode: AlphaMode::Blend,
        ..Default::default()
    });

    let selector_pressed = materials.add(StandardMaterial {
        base_color: Color::rgba(0., 0.7, 0.5, 0.7),
        alpha_mode: AlphaMode::Blend,
        ..Default::default()
    });

    let selector_selected = materials.add(StandardMaterial {
        base_color: Color::rgba(0., 0.9, 0.7, 0.5),
        alpha_mode: AlphaMode::Blend,
        ..Default::default()
    });

    commands.insert_resource(BoardRuntimeAssets {
        tile_info_map,
        tile_base_material,
        decoration_material,
        selector,
        selector_base,
        selector_hovered,
        selector_pressed,
        selector_selected,
    });
}
