use crate::asset_tracking::{ResourceHandles, ResourceLoadState};
use crate::scene::GROUND_HEIGHT;
use crate::{AppState, AssetState, Score, ScoreChanged};
use bevy::color::palettes::css::RED;
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

    app.add_systems(OnEnter(AssetState::Error), spawn_asset_errors);
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

fn spawn_asset_errors(mut commands: Commands, resource_handles: Res<ResourceHandles>) {
    let ResourceLoadState::Failed(errors) = resource_handles.status() else {
        return;
    };

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                width: vw(100),
                height: vh(100),
                justify_content: JustifyContent::Center,
                justify_items: JustifyItems::Center,
                row_gap: px(10),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            DespawnOnEnter(AssetState::Done),
        ))
        .with_children(|parent| {
            parent.spawn(Text::new("Failed to load some assets:"));
            for error in errors {
                parent.spawn((Text::new(error), TextColor(RED.into())));
            }
        });
}
