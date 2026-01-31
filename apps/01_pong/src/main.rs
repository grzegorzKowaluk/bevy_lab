// Support configuring Bevy lints within code.
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

mod asset_tracking;
mod ball;
mod input;
mod paddle;
mod scene;
mod ui;

use crate::asset_tracking::ResourceHandles;
use crate::input::RestartAction;
use avian2d::prelude::Gravity;
use avian2d::PhysicsPlugins;
use bevy::asset::AssetMetaCheck;
use bevy::color::palettes::tailwind::BLUE_900;
use bevy::prelude::*;
use bevy::window::{WindowMode, WindowResolution};
use bevy_enhanced_input::actions;
use bevy_enhanced_input::prelude::*;

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
                        title: "Pong".to_string(),
                        resolution: WindowResolution::new(WORLD_WIDTH, WORLD_HEIGHT),
                        mode: WindowMode::Windowed,
                        resizable: false,
                        fit_canvas_to_parent: true,
                        ..default()
                    }
                    .into(),
                    ..default()
                }),
            PhysicsPlugins::default(),
            // PhysicsDebugPlugin,
        ));

        app.insert_resource(ClearColor(BLUE_900.into()));
        app.insert_resource(Gravity(Vec2::ZERO));
        app.init_state::<AppState>();
        app.init_resource::<Score>();
        app.init_resource::<WaitTimer>();
        app.init_resource::<MenuTimer>();
        app.add_observer(start_waiting);

        app.add_plugins((
            asset_tracking::plugin,
            input::plugin,
            paddle::plugin,
            ball::plugin,
            scene::plugin,
            ui::plugin,
        ));

        // Good standard to group systems by the time of execution
        // Order new `AppSystems` variants by adding them here:
        app.configure_sets(
            Update,
            (
                AppSystems::TickTimers,
                AppSystems::RecordInput,
                AppSystems::Update,
            )
                .chain(),
        );

        app.add_systems(Startup, setup_camera);
        app.add_systems(OnEnter(AppState::Menu), reset_score);
        app.add_systems(
            Update,
            enter_menu.run_if(in_state(AppState::Loading).and(all_assets_loaded)),
        );
        app.add_systems(
            Update,
            tick_menu_timer
                .run_if(in_state(AppState::Menu))
                .in_set(AppSystems::TickTimers),
        );
        app.add_systems(
            Update,
            tick_waiting_timer
                .run_if(in_state(AppState::Waiting))
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
    /// Record player input.
    RecordInput,
    /// Do everything else (consider splitting this into further variants).
    Update,
}

#[derive(States, Debug, Default, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum AppState {
    #[default]
    Loading,
    Menu,
    Waiting,
    Match,
}

#[derive(Resource, Default, Debug)]
struct Score {
    left: u32,
    right: u32,
}

#[derive(Event)]
struct ScoreChanged;

#[derive(Resource)]
struct WaitTimer(Timer);

impl Default for WaitTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(2., TimerMode::Once))
    }
}

#[derive(Resource)]
struct MenuTimer(Timer);

impl Default for MenuTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.5, TimerMode::Once))
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d::default(),
        actions!(Camera[(Action::<RestartAction>::new(), bindings![KeyCode::KeyR])]),
    ));
}

fn enter_menu(mut next_state: ResMut<NextState<AppState>>, mut timer: ResMut<MenuTimer>) {
    timer.0.reset();
    next_state.set(AppState::Menu);
}

fn start_waiting(
    _event: On<ScoreChanged>,
    mut next_state: ResMut<NextState<AppState>>,
    mut timer: ResMut<WaitTimer>,
) {
    timer.0.reset();
    next_state.set(AppState::Waiting);
}

fn all_assets_loaded(resource_handles: Res<ResourceHandles>) -> bool {
    resource_handles.is_all_done()
}

fn tick_waiting_timer(
    time: Res<Time>,
    mut timer: ResMut<WaitTimer>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    timer.0.tick(time.delta());
    println!("Tick waiting");

    if timer.0.just_finished() {
        println!("Waiting finished");
        next_state.set(AppState::Match);
    }
}

fn tick_menu_timer(
    time: Res<Time>,
    mut menu_timer: ResMut<MenuTimer>,
    mut wait_timer: ResMut<WaitTimer>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    menu_timer.0.tick(time.delta());
    println!("Tick menu");
    if menu_timer.0.just_finished() {
        println!("Menu finished");
        wait_timer.0.reset();
        next_state.set(AppState::Waiting);
    }
}

fn reset_score(mut score: ResMut<Score>) {
    score.left = 0;
    score.right = 0;
}
