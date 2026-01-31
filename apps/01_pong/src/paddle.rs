use crate::asset_tracking::LoadResource;
use crate::input::{LeftPaddleMovement, RightPaddleMovement};
use crate::{AppState, WORLD_WIDTH};
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_enhanced_input::action::Action;
use bevy_enhanced_input::actions;
use bevy_enhanced_input::prelude::{Bidirectional, Bindings, Scale, SmoothNudge};

const PADDLE_SIZE: f32 = 150.0;
pub const PADDLE_SPEED: f32 = 800.0;

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<PaddleAssets>();
    app.add_systems(
        OnTransition {
            exited: AppState::Menu,
            entered: AppState::Waiting,
        },
        spawn_paddles,
    );
}

#[derive(Resource, Asset, TypePath, Clone)]
struct PaddleAssets {
    paddle_texture: Handle<Image>,
}

impl FromWorld for PaddleAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        PaddleAssets {
            paddle_texture: asset_server.load("paddle.png"),
        }
    }
}

#[derive(Component)]
pub struct Paddle;

fn spawn_paddles(mut commands: Commands, paddle_assets: Res<PaddleAssets>) {
    let paddle_colliders = vec![
        (
            Position::from_xy(0.0, -(PADDLE_SIZE * 0.32)),
            Rotation::default(),
            Collider::rectangle(PADDLE_SIZE * 0.1, PADDLE_SIZE * 0.3),
        ),
        (
            Position::from_xy(0.0, PADDLE_SIZE * 0.17),
            Rotation::default(),
            Collider::capsule(PADDLE_SIZE * 0.23, PADDLE_SIZE * 0.15),
        ),
    ];

    commands.spawn((
        Paddle,
        Sprite {
            image: paddle_assets.paddle_texture.clone(),
            custom_size: Some(Vec2::new(PADDLE_SIZE, PADDLE_SIZE)),
            ..default()
        },
        Position::from_xy(-((WORLD_WIDTH / 2) as f32) + PADDLE_SIZE / 2.0, 0.0),
        RigidBody::Dynamic,
        Collider::compound(paddle_colliders.clone()),
        DespawnOnEnter(AppState::Menu),
        LockedAxes::ROTATION_LOCKED.lock_translation_x(),
        actions!(
            Paddle[(
                Action::<LeftPaddleMovement>::new(),
                Scale::splat(PADDLE_SPEED),
                SmoothNudge::new(20.0),
                Bindings::spawn(Bidirectional::new(KeyCode::KeyW, KeyCode::KeyS))
            )]
        ),
    ));

    commands.spawn((
        Paddle,
        Sprite {
            image: paddle_assets.paddle_texture.clone(),
            custom_size: Some(Vec2::new(PADDLE_SIZE, PADDLE_SIZE)),
            flip_x: true,
            ..default()
        },
        Position::from_xy((WORLD_WIDTH / 2) as f32 - PADDLE_SIZE / 2.0, 0.0),
        RigidBody::Dynamic,
        Collider::compound(paddle_colliders),
        DespawnOnEnter(AppState::Menu),
        LockedAxes::ROTATION_LOCKED.lock_translation_x(),
        actions!(
            Paddle[(
                Action::<RightPaddleMovement>::new(),
                Scale::splat(PADDLE_SPEED),
                SmoothNudge::new(20.0),
                Bindings::spawn(Bidirectional::new(KeyCode::ArrowUp, KeyCode::ArrowDown))
            )]
        ),
    ));
}
