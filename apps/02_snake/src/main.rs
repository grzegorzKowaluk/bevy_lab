// Support configuring Bevy lints within code.
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

mod food;
mod grid;
mod player;
mod ui;

use avian2d::prelude::Gravity;
use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::window::{WindowMode, WindowResolution};
use bevy_asset_loader::prelude::{
    AssetCollection, ConfigureLoadingState, LoadingState, LoadingStateAppExt,
};
use bevy_enhanced_input::EnhancedInputPlugin;

pub const WORLD_WIDTH: u32 = 1280;
pub const WORLD_HEIGHT: u32 = 720;

fn main() -> AppExit {
    App::new().add_plugins(AppPlugin).run()
}

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // Add Bevy plugins.
        app.add_plugins((
            DefaultPlugins
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics on web build on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "Snake".to_string(),
                        resolution: WindowResolution::new(WORLD_WIDTH, WORLD_HEIGHT),
                        mode: WindowMode::Windowed,
                        resizable: false,
                        fit_canvas_to_parent: true,
                        ..default()
                    }
                    .into(),
                    ..default()
                }),
            EnhancedInputPlugin,
        ));

        app.insert_resource(ClearColor(Color::srgb_u8(82, 133, 57)));
        app.insert_resource(Gravity(Vec2::ZERO));
        app.init_state::<AppState>();
        app.init_state::<AssetState>();
        app.init_resource::<MenuTimer>();

        app.add_plugins((ui::plugin, grid::plugin, player::plugin, food::plugin));

        app.add_loading_state(
            LoadingState::new(AssetState::Loading)
                .continue_to_state(AssetState::Done)
                .on_failure_continue_to_state(AssetState::Error)
                .load_collection::<GlobalAssets>(),
        );

        app.configure_sets(
            Update,
            (
                AppSystems::TickTimers,
                AppSystems::Update,
                AppSystems::Render,
            )
                .chain(),
        );

        app.add_systems(Startup, setup_camera);
        app.add_systems(OnEnter(AssetState::Done), enter_menu);
        app.add_systems(
            Update,
            tick_menu_timer
                .run_if(in_state(AppState::Menu))
                .in_set(AppSystems::TickTimers),
        );
    }
}

/// High-level groupings of systems for the app in the `Update` schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum AppSystems {
    /// Tick timers.
    TickTimers,
    /// Main Update loop
    Update,
    /// Sync mental values to render values
    Render,
}

#[derive(States, Debug, Default, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum AppState {
    #[default]
    Loading,
    Menu,
    Playing,
}

#[derive(States, Debug, Default, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum AssetState {
    #[default]
    Loading,
    Done,
    Error,
}

#[derive(AssetCollection, Resource)]
struct GlobalAssets {
    #[asset(path = "audio/food_eaten_sound.ogg")]
    food_eaten_sound: Handle<AudioSource>,
    #[asset(path = "audio/game_over_sound.ogg")]
    game_over_sound: Handle<AudioSource>,
}

#[derive(Resource)]
struct MenuTimer(Timer);

impl Default for MenuTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(1.0, TimerMode::Repeating))
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d,));
}

fn enter_menu(mut next_state: ResMut<NextState<AppState>>, mut timer: ResMut<MenuTimer>) {
    timer.0.reset();
    next_state.set(AppState::Menu);
}

fn tick_menu_timer(
    time: Res<Time>,
    mut menu_timer: ResMut<MenuTimer>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    menu_timer.0.tick(time.delta());

    if menu_timer.0.just_finished() {
        next_state.set(AppState::Playing);
    }
}
