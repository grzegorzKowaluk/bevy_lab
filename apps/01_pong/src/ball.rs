use crate::AppState;
use crate::asset_tracking::LoadResource;
use avian2d::prelude::*;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<BallAssets>();
    app.add_systems(OnEnter(AppState::Warmup), spawn_ball);
    app.add_systems(OnEnter(AppState::Playing), change_velocity);
}

#[derive(Resource, Asset, TypePath, Clone)]
struct BallAssets {
    ball_texture: Handle<Image>,
}

impl FromWorld for BallAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        BallAssets {
            ball_texture: asset_server.load("ball.png"),
        }
    }
}

#[derive(Component)]
struct Ball;

fn spawn_ball(mut commands: Commands, ball_assets: Res<BallAssets>) {
    commands.spawn((
        Ball,
        Sprite {
            image: ball_assets.ball_texture.clone(),
            custom_size: Some(Vec2::new(20.0, 20.0)),
            ..default()
        },
        Position::default(),
        RigidBody::Dynamic,
        Collider::circle(9.0),
        DespawnOnEnter(AppState::Loading),
        LinearVelocity::default(),
    ));
}

fn change_velocity(mut query: Query<&mut LinearVelocity, With<Ball>>) {
    for mut velocity in &mut query {
        velocity.0 = Vec2::new(-20.0, 5.0);
    }
}
