use bevy::prelude::*;
use iyes_loopless::prelude::AppLooplessStateExt;

use crate::app_state::AppState;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(AppState::InGame, build_board);
    }
}

fn build_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let handle = meshes.add(shape::Cube::default().into());
    let material = materials.add(StandardMaterial {
        base_color: Color::hex("e7d2a4").unwrap_or_default(),
        ..Default::default()
    });
    commands.spawn(PbrBundle {
        mesh: handle,
        material,
        ..Default::default()
    });
}
