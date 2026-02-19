use crate::food::{Food, FoodEaten};
use crate::grid::{grid_to_world, GridPos, GRID_HEIGHT, GRID_WIDTH, TILE_SIZE};
use crate::{AppState, AppSystems, AssetState, GlobalAssets};
use bevy::asset::Assets;
use bevy::color::palettes::css::{BLACK, WHITE};
use bevy::color::palettes::tailwind::{BLUE_400, BLUE_500};
use bevy::mesh::Mesh;
use bevy::prelude::*;
use bevy_enhanced_input::actions;
use bevy_enhanced_input::prelude::*;

const HEAD_COLOR: Color = Color::Srgba(BLUE_500);
const BODY_COLOR: Color = Color::Srgba(BLUE_400);
const SNAKE_Z: f32 = 2.0;
const SNAKE_MOVE_INTERVAL: f32 = 0.24;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<MoveTimer>();
    app.init_resource::<CurrentDirection>();
    app.init_resource::<SnakeSegments>();
    app.init_resource::<PendingGrowth>();

    app.add_input_context::<SnakeHead>();
    app.add_observer(observe_up);
    app.add_observer(observe_down);
    app.add_observer(observe_right);
    app.add_observer(observe_left);

    app.add_systems(OnEnter(AssetState::Loading), setup);
    app.add_systems(
        OnEnter(AppState::Playing),
        (reset_resources, spawn_player).chain(),
    );
    app.add_systems(
        Update,
        tick_move_timer
            .run_if(in_state(AppState::Playing))
            .in_set(AppSystems::TickTimers),
    );
    app.add_systems(
        Update,
        (snake_movement, check_snake_collisions)
            .chain()
            .run_if(in_state(AppState::Playing))
            .in_set(AppSystems::Update),
    );
    app.add_systems(
        Update,
        sync_head_rotation_to_transform
            .run_if(in_state(AppState::Playing))
            .in_set(AppSystems::Render),
    );
}

#[derive(Component)]
struct SnakeSegment;

#[derive(Resource, Default, Clone, Eq, PartialEq)]
pub struct PendingGrowth(pub u32);

#[derive(Resource, Default, Clone, Eq, PartialEq)]
pub struct SnakeSegments(pub Vec<Entity>);

#[derive(Component)]
struct SnakeHead(f32); // rotation in radians

#[derive(InputAction)]
#[action_output(bool)]
struct MoveUp;

#[derive(InputAction)]
#[action_output(bool)]
struct MoveDown;

#[derive(InputAction)]
#[action_output(bool)]
struct MoveRight;

#[derive(InputAction)]
#[action_output(bool)]
struct MoveLeft;

#[derive(Resource, Default, Clone, Eq, PartialEq)]
struct CurrentDirection(Direction);

#[derive(Resource)]
struct MoveTimer(Timer);

impl Default for MoveTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(
            SNAKE_MOVE_INTERVAL,
            TimerMode::Repeating,
        ))
    }
}

#[derive(Resource)]
pub struct SnakeAssets {
    mesh: Handle<Mesh>,
    body_material: Handle<ColorMaterial>,
}

#[derive(Default, Copy, Clone, Eq, PartialEq, Debug)]
enum Direction {
    #[default]
    Right,
    Left,
    Up,
    Down,
}

impl Direction {
    fn delta(self) -> IVec2 {
        match self {
            Direction::Up => IVec2::new(0, 1),
            Direction::Down => IVec2::new(0, -1),
            Direction::Left => IVec2::new(-1, 0),
            Direction::Right => IVec2::new(1, 0),
        }
    }
    fn angle(self) -> f32 {
        match self {
            Direction::Up => std::f32::consts::FRAC_PI_2,    // 90°
            Direction::Down => -std::f32::consts::FRAC_PI_2, // -90°
            Direction::Left => std::f32::consts::PI,         // 180°
            Direction::Right => 0.0,                         // 0°
        }
    }
    fn is_opposite(self, other: Direction) -> bool {
        matches!(
            (self, other),
            (Direction::Up, Direction::Down)
                | (Direction::Down, Direction::Up)
                | (Direction::Left, Direction::Right)
                | (Direction::Right, Direction::Left)
        )
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = meshes.add(Rectangle::new(TILE_SIZE * 0.9, TILE_SIZE * 0.8));
    let body_material = materials.add(ColorMaterial::from_color(BODY_COLOR));

    commands.insert_resource(SnakeAssets {
        mesh,
        body_material,
    });
}

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    snake_assets: Res<SnakeAssets>,
    mut segments: ResMut<SnakeSegments>,
) {
    let eye_x = TILE_SIZE * 0.3;
    let eye_y = TILE_SIZE * 0.2;
    let eye_radius = TILE_SIZE * 0.15;

    let pupil_x = eye_radius * 0.5;
    let pupil_y = eye_radius * 0.2;
    let pupil_radius = eye_radius * 0.3;

    let eye_mesh = meshes.add(Circle::new(eye_radius));
    let pupil_mesh = meshes.add(Circle::new(pupil_radius));

    let head_material = materials.add(ColorMaterial::from_color(HEAD_COLOR));
    let eye_material = materials.add(ColorMaterial::from_color(WHITE));
    let pupil_material = materials.add(ColorMaterial::from_color(BLACK));

    let head_grid_pos = GridPos { x: 3, y: 4 };
    let segment_grid_pos1 = GridPos { x: 2, y: 4 };
    let segment_grid_pos2 = GridPos { x: 1, y: 4 };

    let head_segment = commands
        .spawn((
            SnakeHead(0.0),
            head_grid_pos,
            Mesh2d(snake_assets.mesh.clone()),
            MeshMaterial2d(head_material.clone()),
            Transform::from_translation(grid_to_world(head_grid_pos).with_z(SNAKE_Z)),
            DespawnOnExit(AppState::Playing),
            actions!(
                SnakeHead[
                    (
                    Action::<MoveUp>::new(),
                    bindings![KeyCode::KeyW, KeyCode::ArrowUp]
                ),
                (
                    Action::<MoveDown>::new(),
                    bindings![KeyCode::KeyS, KeyCode::ArrowDown]
                ),
                    (
                    Action::<MoveRight>::new(),
                    bindings![KeyCode::KeyD, KeyCode::ArrowRight]
                ),
                    (
                    Action::<MoveLeft>::new(),
                    bindings![KeyCode::KeyA, KeyCode::ArrowLeft]
                ),
                ]
            ),
            children![
                (
                    Mesh2d(eye_mesh.clone()),
                    MeshMaterial2d(eye_material.clone()),
                    Transform::from_xyz(eye_x, eye_y, 1.0),
                    children![(
                        Mesh2d(pupil_mesh.clone()),
                        MeshMaterial2d(pupil_material.clone()),
                        Transform::from_xyz(pupil_x, -pupil_y, 1.0),
                    )]
                ),
                (
                    Mesh2d(eye_mesh.clone()),
                    MeshMaterial2d(eye_material.clone()),
                    Transform::from_xyz(eye_x, -eye_y, 1.0),
                    children![(
                        Mesh2d(pupil_mesh.clone()),
                        MeshMaterial2d(pupil_material.clone()),
                        Transform::from_xyz(pupil_x, pupil_y, 1.0),
                    )]
                ),
            ],
        ))
        .id();

    let segment1 = commands
        .spawn((
            SnakeSegment,
            segment_grid_pos1,
            Mesh2d(snake_assets.mesh.clone()),
            MeshMaterial2d(snake_assets.body_material.clone()),
            Transform::from_translation(grid_to_world(segment_grid_pos1).with_z(SNAKE_Z)),
            DespawnOnExit(AppState::Playing),
        ))
        .id();

    let segment2 = commands
        .spawn((
            SnakeSegment,
            segment_grid_pos2,
            Mesh2d(snake_assets.mesh.clone()),
            MeshMaterial2d(snake_assets.body_material.clone()),
            Transform::from_translation(grid_to_world(segment_grid_pos2).with_z(SNAKE_Z)),
            DespawnOnExit(AppState::Playing),
        ))
        .id();

    segments.0.extend([head_segment, segment1, segment2]);
}

fn tick_move_timer(time: Res<Time>, mut timer: ResMut<MoveTimer>) {
    timer.0.tick(time.delta());
}

fn snake_movement(
    mut commands: Commands,
    timer: Res<MoveTimer>,
    mut segments: ResMut<SnakeSegments>,
    mut positions: Query<&mut GridPos>,
    current_direction: Res<CurrentDirection>,
    mut head: Single<&mut SnakeHead>,
    mut pending_growth: ResMut<PendingGrowth>,
    snake_assets: Res<SnakeAssets>,
) {
    if !timer.0.just_finished() {
        return;
    }

    head.0 = current_direction.0.angle();
    let head_entity = segments.0[0];
    let old_head = {
        let mut head_pos = positions.get_mut(head_entity).unwrap();
        let old = *head_pos;
        let delta = current_direction.0.delta();
        head_pos.x += delta.x;
        head_pos.y += delta.y;
        old
    };

    let mut previous = old_head;

    for entity in segments.0.iter().skip(1) {
        let mut pos = positions.get_mut(*entity).unwrap();
        previous = std::mem::replace(&mut *pos, previous);
    }

    if pending_growth.0 > 0 {
        pending_growth.0 -= 1;
        let segment = commands
            .spawn((
                SnakeSegment,
                previous,
                Mesh2d(snake_assets.mesh.clone()),
                MeshMaterial2d(snake_assets.body_material.clone()),
                Transform::from_translation(grid_to_world(previous).with_z(SNAKE_Z)),
                DespawnOnExit(AppState::Playing),
            ))
            .id();

        segments.0.push(segment);
    }
}

fn sync_head_rotation_to_transform(head_query: Query<(&SnakeHead, &mut Transform)>) {
    for (head, mut transform) in head_query {
        transform.rotation = Quat::from_rotation_z(head.0);
    }
}

fn observe_up(
    _event: On<Start<MoveUp>>,
    timer: ResMut<MoveTimer>,
    current_direction: ResMut<CurrentDirection>,
) {
    apply_movement(Direction::Up, timer, current_direction);
}
fn observe_down(
    _event: On<Start<MoveDown>>,
    timer: ResMut<MoveTimer>,
    current_direction: ResMut<CurrentDirection>,
) {
    apply_movement(Direction::Down, timer, current_direction);
}
fn observe_right(
    _event: On<Start<MoveRight>>,
    timer: ResMut<MoveTimer>,
    current_direction: ResMut<CurrentDirection>,
) {
    apply_movement(Direction::Right, timer, current_direction);
}
fn observe_left(
    _event: On<Start<MoveLeft>>,
    timer: ResMut<MoveTimer>,
    current_direction: ResMut<CurrentDirection>,
) {
    apply_movement(Direction::Left, timer, current_direction);
}

fn apply_movement(
    new_direction: Direction,
    mut timer: ResMut<MoveTimer>,
    mut current_direction: ResMut<CurrentDirection>,
) {
    if current_direction.0.is_opposite(new_direction) {
        return;
    }

    current_direction.0 = new_direction;

    let timer_duration = timer.0.duration();
    timer.0.set_elapsed(timer_duration);
}

fn check_snake_collisions(
    mut commands: Commands,
    segments: Res<SnakeSegments>,
    positions: Query<&GridPos, Without<Food>>,
    food_query: Query<(Entity, &GridPos), With<Food>>,
    mut pending_growth: ResMut<PendingGrowth>,
    mut next_state: ResMut<NextState<AppState>>,
    global_assets: Res<GlobalAssets>,
) {
    let head_entity = segments.0[0];
    let head_pos = positions.get(head_entity).unwrap();

    // grid boundaries
    if head_pos.x < 0
        || head_pos.x >= GRID_WIDTH as i32
        || head_pos.y < 0
        || head_pos.y >= GRID_HEIGHT as i32
    {
        info!("You went outside of a grid -> Game over!");
        next_state.set(AppState::Menu);
        commands.spawn((
            AudioPlayer::new(global_assets.game_over_sound.clone()),
            PlaybackSettings::ONCE,
        ));
    }

    // food collisions
    for (food_entity, food_pos) in &food_query {
        if head_pos == food_pos {
            info!("You ate a food -> Grow!");
            commands.entity(food_entity).despawn();
            commands.trigger(FoodEaten);
            pending_growth.0 += 1;
            commands.spawn((
                AudioPlayer::new(global_assets.food_eaten_sound.clone()),
                PlaybackSettings::ONCE,
            ));
        }
    }

    // self collisions
    for segment in segments.0.iter().skip(1) {
        let body_pos = positions.get(*segment).unwrap();
        if head_pos == body_pos {
            info!("You touched your own body -> Game over!");
            next_state.set(AppState::Menu);
            commands.spawn((
                AudioPlayer::new(global_assets.game_over_sound.clone()),
                PlaybackSettings::ONCE,
            ));
        }
    }
}

fn reset_resources(
    mut segments: ResMut<SnakeSegments>,
    mut move_timer: ResMut<MoveTimer>,
    mut direction: ResMut<CurrentDirection>,
) {
    segments.0.clear();
    move_timer.0.reset();
    direction.0 = Direction::Right;
}
