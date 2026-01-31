use crate::scene::GROUND_HEIGHT;
use crate::{AppState, Score, ScoreChanged};
use bevy::prelude::*;
use bevy::text::LineHeight;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnTransition {
            exited: AppState::Menu,
            entered: AppState::Waiting,
        },
        (spawn_score_ui, spawn_restart_ui),
    );
    app.add_observer(update_score_ui);
}

#[derive(Component)]
struct ScoreText;

fn spawn_score_ui(mut commands: Commands) {
    commands.spawn((
        ScoreText,
        Node {
            position_type: PositionType::Absolute,
            top: px(0),
            width: percent(100),
            height: px(GROUND_HEIGHT),
            ..default()
        },
        Text::new("0 : 0"),
        TextLayout::new_with_justify(Justify::Center),
        TextFont::from_font_size(42.0),
        LineHeight::Px(GROUND_HEIGHT),
        DespawnOnEnter(AppState::Menu),
    ));
}

fn spawn_restart_ui(mut commands: Commands) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            bottom: px(0),
            width: percent(100),
            height: px(GROUND_HEIGHT),
            ..default()
        },
        Text::new("Press 'r' to restart"),
        TextLayout::new_with_justify(Justify::Center),
        TextFont::from_font_size(20.0),
        LineHeight::Px(GROUND_HEIGHT),
        DespawnOnEnter(AppState::Menu),
    ));
}

fn update_score_ui(
    _event: On<ScoreChanged>,
    score: Res<Score>,
    mut text: Single<&mut Text, With<ScoreText>>,
) {
    text.0 = format!("{} : {}", score.left, score.right);
}
