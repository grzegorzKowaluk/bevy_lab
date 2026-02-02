use crate::paddle::Paddle;
use crate::{AppState, MenuTimer};
use avian2d::prelude::LinearVelocity;
use bevy::prelude::*;
use bevy_enhanced_input::EnhancedInputPlugin;
use bevy_enhanced_input::prelude::{Fire, InputAction, InputContextAppExt};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(EnhancedInputPlugin);
    app.add_input_context::<Paddle>();
    app.add_input_context::<Camera>();
    app.add_observer(apply_left_paddle_movement);
    app.add_observer(apply_right_paddle_movement);
    app.add_observer(apply_restart);
}

#[derive(InputAction)]
#[action_output(f32)]
pub struct LeftPaddleMovement;

#[derive(InputAction)]
#[action_output(f32)]
pub struct RightPaddleMovement;

#[derive(InputAction)]
#[action_output(bool)]
pub struct RestartAction;

fn apply_left_paddle_movement(
    movement: On<Fire<LeftPaddleMovement>>,
    mut paddles: Query<&mut LinearVelocity, With<Paddle>>,
) {
    if let Ok(mut velocity) = paddles.get_mut(movement.context) {
        velocity.y = movement.value;
    }
}

fn apply_right_paddle_movement(
    movement: On<Fire<RightPaddleMovement>>,
    mut paddles: Query<&mut LinearVelocity, With<Paddle>>,
) {
    if let Ok(mut velocity) = paddles.get_mut(movement.context) {
        velocity.y = movement.value;
    }
}

fn apply_restart(
    _event: On<Fire<RestartAction>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut timer: ResMut<MenuTimer>,
) {
    timer.0.reset();
    next_state.set(AppState::Menu);
}
