use crate::game::AssetsState;
use bevy::color::palettes::basic::RED;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AssetsState::Error), spawn_asset_errors);
}

fn spawn_asset_errors(mut commands: Commands) {
    commands.spawn((
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
        children![(
            Text::new("Failed to load assets"),
            TextColor(RED.into()),
            TextLayout::new_with_justify(Justify::Center)
        )],
    ));
}
