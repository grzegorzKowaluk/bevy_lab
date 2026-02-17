use crate::{AppState, AppSystems};
use bevy::asset::Assets;
use bevy::mesh::Mesh;
use bevy::prelude::*;
use rand::RngExt;

pub const GRID_LIGHT_COLOR: Color = Color::srgb_u8(170, 215, 81);
pub const GRID_DARK_COLOR: Color = Color::srgb_u8(162, 209, 73);
pub const TILE_SIZE: f32 = 64.0;
pub const GRID_WIDTH: u32 = 11;
pub const GRID_HEIGHT: u32 = 9;
pub const GRID_Z: f32 = 0.0;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::Playing), spawn_grid);
    app.add_systems(
        Update,
        sync_grid_to_transform
            .run_if(in_state(AppState::Playing))
            .in_set(AppSystems::Render),
    );
}

#[derive(Component)]
struct Tile;

#[derive(Component, Copy, Clone, Eq, PartialEq, Hash)]
pub struct GridPos {
    pub x: i32,
    pub y: i32,
}

pub fn random_grid_pos() -> GridPos {
    let mut rng = rand::rng();
    GridPos {
        x: rng.random_range(0..GRID_WIDTH as i32),
        y: rng.random_range(0..GRID_HEIGHT as i32),
    }
}

pub fn grid_to_world(pos: GridPos) -> Vec3 {
    let board_width = GRID_WIDTH as f32 * TILE_SIZE;
    let board_height = GRID_HEIGHT as f32 * TILE_SIZE;

    Vec3::new(
        pos.x as f32 * TILE_SIZE - board_width / 2.0 + TILE_SIZE / 2.0,
        pos.y as f32 * TILE_SIZE - board_height / 2.0 + TILE_SIZE / 2.0,
        GRID_Z,
    )
}

fn spawn_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // One shared quad mesh
    let tile_mesh = meshes.add(Rectangle::new(TILE_SIZE, TILE_SIZE));

    // Two shared materials
    let light = materials.add(GRID_LIGHT_COLOR);
    let dark = materials.add(GRID_DARK_COLOR);

    for y in 0..GRID_HEIGHT as i32 {
        for x in 0..GRID_WIDTH as i32 {
            let grid_pos = GridPos { x, y };
            let is_light = (x + y) % 2 == 0;

            commands.spawn((
                Tile,
                grid_pos,
                Mesh2d(tile_mesh.clone()),
                MeshMaterial2d(if is_light {
                    light.clone()
                } else {
                    dark.clone()
                }),
                Transform::from_translation(grid_to_world(grid_pos)),
                DespawnOnExit(AppState::Playing),
            ));
        }
    }
}

fn sync_grid_to_transform(mut query: Query<(&GridPos, &mut Transform), Changed<GridPos>>) {
    for (grid, mut transform) in &mut query {
        transform.translation = grid_to_world(*grid).with_z(transform.translation.z);
    }
}
