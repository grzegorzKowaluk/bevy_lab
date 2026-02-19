use crate::framework::mesh_material::{MeshMaterial, MeshMaterialConfig};
use crate::game::GameState;
use avian3d::prelude::{Collider, RigidBody};
use bevy::color::palettes::tailwind::GREEN_500;
use bevy::prelude::*;

const CHUNK_LENGTH: f32 = 40.0;
const CHUNK_WIDTH: f32 = 20.0;
pub const LANE_WIDTH: f32 = CHUNK_WIDTH * 0.25;
const NUMBER_OF_CHUNKS: f32 = 6.0;
pub const DESPAWN_THRESHOLD: f32 = -50.0;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<ChunkLibrary>();
    app.init_resource::<TrackState>();
    app.init_resource::<MeshMaterial<DevChunk>>();

    app.add_systems(Startup, setup);
    app.add_systems(OnEnter(GameState::Playing), setup_world_root);
    app.add_systems(
        FixedUpdate,
        (spawn_chunks, move_world, despawn_chunks).run_if(in_state(GameState::Playing)),
    );
}

fn setup(
    mut commands: Commands,
    dev_chunk_assets: Res<MeshMaterial<DevChunk>>,
    mut scenes: ResMut<Assets<Scene>>,
) {
    let mut world = World::new();

    world.spawn((
        Mesh3d(dev_chunk_assets.mesh.clone()),
        MeshMaterial3d(dev_chunk_assets.material.clone()),
    ));

    let scene_handle = scenes.add(Scene::new(world));

    let chunks = vec![ChunkDefinition {
        scene: scene_handle,
        length: CHUNK_LENGTH,
        difficulty: ChunkDifficulty::default(),
    }];

    commands.insert_resource(ChunkLibrary { chunks })
}

fn setup_world_root(mut commands: Commands) {
    commands.spawn(WorldRoot);
}

fn spawn_chunks(
    mut commands: Commands,
    chunk_library: Res<ChunkLibrary>,
    mut track_state: ResMut<TrackState>,
    world_root_query: Single<(Entity, &Transform), With<WorldRoot>>,
) {
    let (world_root, world_transform) = world_root_query.into_inner();

    let spawn_distance_ahead = CHUNK_LENGTH * NUMBER_OF_CHUNKS;

    let world_offset = world_transform.translation.z;

    let visible_forward = -world_offset + spawn_distance_ahead;

    let chunk_definition = chunk_library.chunks.first().unwrap();

    if track_state.next_spawn_z < visible_forward {
        spawn_chunk(
            &mut commands,
            chunk_definition,
            &mut track_state,
            world_root,
        );
    }
}

fn spawn_chunk(
    commands: &mut Commands,
    chunk_definition: &ChunkDefinition,
    track: &mut TrackState,
    world_root: Entity,
) {
    let z = track.next_spawn_z;

    commands.entity(world_root).with_children(|parent| {
        parent.spawn((
            SceneRoot(chunk_definition.scene.clone()),
            Transform::from_xyz(0.0, 0.0, z),
            RigidBody::Kinematic,
            Collider::cuboid(CHUNK_WIDTH, 0.1, CHUNK_LENGTH),
            ActiveChunk {
                end_z: z + chunk_definition.length,
            },
        ));
    });

    track.next_spawn_z += chunk_definition.length;
}

fn move_world(
    fixed_time: Res<Time<Fixed>>,
    mut world_root_transform: Single<&mut Transform, With<WorldRoot>>,
) {
    let speed = 30.0;

    world_root_transform.translation.z -= speed * fixed_time.delta_secs();
}

fn despawn_chunks(
    mut commands: Commands,
    world_root_transform: Single<&Transform, With<WorldRoot>>,
    chunks: Query<(Entity, &ActiveChunk)>,
) {
    let world_z = world_root_transform.translation.z;

    for (entity, chunk) in chunks.iter() {
        let chunk_end_world = chunk.end_z + world_z;

        if chunk_end_world < DESPAWN_THRESHOLD {
            commands.entity(entity).despawn();
        }
    }
}

#[derive(Component)]
#[require(Transform, Visibility)]
struct WorldRoot;

#[derive(Component)]
struct DevChunk;

impl MeshMaterialConfig for DevChunk {
    fn build_mesh() -> Mesh {
        Mesh::from(Plane3d::new(
            Vec3::Y,
            Vec2::new(CHUNK_WIDTH * 0.5, CHUNK_LENGTH * 0.5),
        ))
    }
    fn build_material() -> StandardMaterial {
        StandardMaterial::from_color(Color::from(GREEN_500))
    }
}

#[derive(Default)]
enum ChunkDifficulty {
    #[default]
    Easy,
    Medium,
    Hard,
}

struct ChunkDefinition {
    scene: Handle<Scene>,
    length: f32,
    difficulty: ChunkDifficulty,
}

#[derive(Resource, Default)]
struct ChunkLibrary {
    chunks: Vec<ChunkDefinition>,
}

#[derive(Component)]
struct ActiveChunk {
    end_z: f32,
}

#[derive(Resource, Default)]
struct TrackState {
    next_spawn_z: f32,
}
