pub mod camera;
pub mod debug;
pub mod player;
pub mod scene;
pub mod ui;

use avian3d::prelude::PhysicsDebugPlugin;
use avian3d::PhysicsPlugins;
use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::window::{WindowMode, WindowResolution};
use bevy_asset_loader::prelude::*;
use bevy_enhanced_input::EnhancedInputPlugin;

pub const WORLD_WIDTH: u32 = 1280;
pub const WORLD_HEIGHT: u32 = 720;
pub struct GamePlugin;

impl Plugin for GamePlugin {
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
                        title: "Runner".to_string(),
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
            PhysicsDebugPlugin,
            EnhancedInputPlugin,
        ));

        app.insert_resource(ClearColor(Color::BLACK));
        app.init_state::<GameState>();
        app.init_state::<AssetsState>();
        app.init_resource::<MenuTimer>();

        app.add_plugins((
            ui::plugin,
            player::plugin,
            scene::plugin,
            camera::plugin,
            debug::plugin,
        ));

        app.add_loading_state(
            LoadingState::new(AssetsState::Loading)
                .continue_to_state(AssetsState::Done)
                .on_failure_continue_to_state(AssetsState::Error)
                .load_collection::<GameAssets>(),
        );

        app.configure_sets(
            Update,
            (GameSystems::TickTimers, GameSystems::Update).chain(),
        );

        app.add_systems(Startup, global_setup);
        app.add_systems(OnEnter(AssetsState::Done), enter_menu);
        app.add_systems(
            Update,
            tick_menu_timer
                .run_if(in_state(GameState::Menu))
                .in_set(GameSystems::TickTimers),
        );
    }
}

/// High-level groupings of systems for the app in the `Update` schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum GameSystems {
    /// Tick timers.
    TickTimers,
    /// Main Update loop
    Update,
}

#[derive(States, Debug, Default, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum GameState {
    #[default]
    Loading,
    Menu,
    Playing,
}

#[derive(States, Debug, Default, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum AssetsState {
    #[default]
    Loading,
    Done,
    Error,
}

#[derive(AssetCollection, Resource)]
pub struct GameAssets {}

#[derive(Resource)]
struct MenuTimer(Timer);

impl Default for MenuTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(1.0, TimerMode::Repeating))
    }
}

fn global_setup(mut commands: Commands) {
    commands.spawn((
        DirectionalLight {
            illuminance: 15_000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::default().looking_to(Vec3::new(-1.0, -1.0, -0.3), Vec3::Y),
    ));
}

fn enter_menu(mut next_state: ResMut<NextState<GameState>>, mut timer: ResMut<MenuTimer>) {
    timer.0.reset();
    next_state.set(GameState::Menu);
}

fn tick_menu_timer(
    time: Res<Time>,
    mut menu_timer: ResMut<MenuTimer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    menu_timer.0.tick(time.delta());

    if menu_timer.0.just_finished() {
        next_state.set(GameState::Playing);
    }
}
