use crate::{AppState, WORLD_HEIGHT, WORLD_WIDTH};
use avian2d::prelude::*;
use bevy::color::palettes::tailwind::GRAY_800;
use bevy::prelude::*;

pub const GROUND_HEIGHT: f32 = 80.0;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnExit(AppState::Loading), (spawn_ground, spawn_goals));
}

#[derive(Component)]
pub struct Ground;

#[derive(Component)]
pub struct Goal(bool);

impl Goal {
    pub fn is_left(&self) -> bool {
        !self.0
    }
}

fn spawn_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let ground = Rectangle::new(WORLD_WIDTH as f32, GROUND_HEIGHT);
    let ground_color = ColorMaterial::from_color(GRAY_800);

    let mesh = meshes.add(ground);
    let material = materials.add(ground_color);

    commands.spawn((
        Ground,
        Mesh2d(mesh.clone()),
        MeshMaterial2d(material.clone()),
        Position::from_xy(0.0, -((WORLD_HEIGHT / 2) as f32) + GROUND_HEIGHT / 2.0),
        RigidBody::Static,
        Collider::rectangle(WORLD_WIDTH as f32, GROUND_HEIGHT),
    ));

    commands.spawn((
        Ground,
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Position::from_xy(0.0, ((WORLD_HEIGHT / 2) as f32) - GROUND_HEIGHT / 2.0),
        RigidBody::Static,
        Collider::rectangle(WORLD_WIDTH as f32, GROUND_HEIGHT),
    ));
}

fn spawn_goals(mut commands: Commands) {
    commands.spawn((
        Goal(false),
        Position::from_xy(-((WORLD_WIDTH / 2) as f32), 0.0),
        RigidBody::Static,
        Collider::rectangle(10.0, WORLD_HEIGHT as f32),
    ));

    commands.spawn((
        Goal(true),
        Position::from_xy((WORLD_WIDTH / 2) as f32, 0.0),
        RigidBody::Static,
        Collider::rectangle(10.0, WORLD_HEIGHT as f32),
    ));
}
