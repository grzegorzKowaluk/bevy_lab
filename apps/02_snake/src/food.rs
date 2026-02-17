use crate::grid::{grid_to_world, random_grid_pos, GridPos, TILE_SIZE};
use crate::player::SnakeSegments;
use crate::{AppState, AssetState};
use bevy::app::App;
use bevy::color::palettes::tailwind::RED_400;
use bevy::prelude::*;

const FOOD_Z: f32 = 1.0;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AssetState::Loading), setup);
    app.add_observer(on_food_eaten);

    app.add_systems(OnEnter(AppState::Playing), spawn_food);
}

#[derive(Component)]
pub struct Food;

#[derive(Resource)]
struct FoodAssets {
    mesh: Handle<Mesh>,
    material: Handle<ColorMaterial>,
}

#[derive(Event)]
pub struct FoodEaten;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let food_size = TILE_SIZE * 0.8;

    let mesh = meshes.add(Rectangle::new(food_size, food_size));
    let material = materials.add(ColorMaterial::from(Color::from(RED_400)));

    commands.insert_resource(FoodAssets { mesh, material });
}

fn on_food_eaten(
    _event: On<FoodEaten>,
    commands: Commands,
    snake_segments: Res<SnakeSegments>,
    positions: Query<&GridPos>,
    food_assets: Res<FoodAssets>,
) {
    spawn_food(commands, snake_segments, positions, food_assets);
}

fn spawn_food(
    mut commands: Commands,
    snake_segments: Res<SnakeSegments>,
    positions: Query<&GridPos>,
    food_assets: Res<FoodAssets>,
) {
    let occupied: Vec<GridPos> = snake_segments
        .0
        .iter()
        .filter_map(|&entity| positions.get(entity).ok())
        .copied()
        .collect();

    loop {
        let pos = random_grid_pos();
        if !occupied.contains(&pos) {
            commands.spawn((
                Food,
                pos,
                Mesh2d(food_assets.mesh.clone()),
                MeshMaterial2d(food_assets.material.clone()),
                Transform::from_translation(grid_to_world(pos).with_z(FOOD_Z)),
                DespawnOnExit(AppState::Playing),
            ));
            break;
        }
    }
}
