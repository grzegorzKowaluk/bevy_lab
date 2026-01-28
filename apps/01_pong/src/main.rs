// Support configuring Bevy lints within code.
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

mod asset_tracking;
mod ball;
mod scene;

use crate::asset_tracking::ResourceHandles;
use avian2d::PhysicsPlugins;
use avian2d::prelude::PhysicsDebugPlugin;
use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;

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
                        fit_canvas_to_parent: true,
                        ..default()
                    }
                    .into(),
                    ..default()
                }),
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
        ));

        app.init_state::<AppState>();
        app.init_resource::<WarmupTimer>();

        app.add_plugins((
            asset_tracking::plugin,
            // TODO
            // paddle::plugin,
            ball::plugin,
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
        app.add_systems(
            Update,
            enter_warmup.run_if(in_state(AppState::Loading).and(all_assets_loaded)),
        );
        app.add_systems(
            Update,
            tick_warmup_timer
                .run_if(in_state(AppState::Warmup))
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
    Warmup,
    Playing,
}

#[derive(Resource)]
struct WarmupTimer(Timer);

impl Default for WarmupTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(10., TimerMode::Once))
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn enter_warmup(mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::Warmup);
}

fn all_assets_loaded(resource_handles: Res<ResourceHandles>) -> bool {
    resource_handles.is_all_done()
}

fn tick_warmup_timer(
    time: Res<Time>,
    mut timer: ResMut<WarmupTimer>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    timer.0.tick(time.delta());

    if timer.0.just_finished() {
        next_state.set(AppState::Playing);
    }
}
