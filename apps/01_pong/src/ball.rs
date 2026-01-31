use crate::asset_tracking::LoadResource;
use crate::scene::{Goal, Ground};
use crate::{AppState, Score, ScoreChanged};
use avian2d::prelude::*;
use bevy::prelude::*;
use rand::prelude::*;
use std::f32::consts::PI;

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<BallAssets>();
    app.add_systems(OnEnter(AppState::Waiting), spawn_ball);
    app.add_systems(OnEnter(AppState::Match), change_velocity);
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
    commands
        .spawn((
            Ball,
            Sprite {
                image: ball_assets.ball_texture.clone(),
                custom_size: Some(Vec2::new(20.0, 20.0)),
                ..default()
            },
            RigidBody::Dynamic,
            Collider::circle(9.0),
            DespawnOnExit(AppState::Match),
            DespawnOnEnter(AppState::Menu),
            Restitution::new(1.0).with_combine_rule(CoefficientCombine::Max),
            Friction::new(0.0).with_combine_rule(CoefficientCombine::Min),
            CollisionEventsEnabled,
        ))
        .observe(on_ball_collision);
}

fn on_ball_collision(
    event: On<CollisionStart>,
    mut commands: Commands,
    ground_query: Query<&Ground>,
    goal_query: Query<&Goal>,
    mut score: ResMut<Score>,
) {
    let colliding_entity = event.collider2;

    if ground_query.contains(colliding_entity) {
        println!("Ball touched the ground");
        return;
    }

    if let Ok(goal) = goal_query.get(colliding_entity) {
        println!("Ball touched the goal");
        if goal.is_left() {
            score.right += 1;
        } else {
            score.left += 1;
        }
        commands.trigger(ScoreChanged);
    }
}

fn change_velocity(mut velocity: Single<&mut LinearVelocity, With<Ball>>) {
    let mut rng = rand::rng();
    let speed = 500.0;

    let going_right = rng.random_bool(0.5);

    let angle = if going_right {
        rng.random_range(-PI / 4.0..PI / 4.0)
    } else {
        rng.random_range(3.0 * PI / 4.0..5.0 * PI / 4.0)
    };

    velocity.0 = Vec2::new(angle.cos(), angle.sin()) * speed;
}
