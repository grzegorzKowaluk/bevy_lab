use crate::framework::mesh_material::{MeshMaterial, MeshMaterialConfig};
use crate::game::debug::DebugSphere;
use crate::game::scene::LANE_WIDTH;
use crate::game::GameState;
use avian3d::math::Scalar;
use avian3d::prelude::{
    CoefficientCombine, Collider, Forces, LinearVelocity, LockedAxes, Mass, MassPropertyHelper,
    Restitution, RigidBody, RigidBodyForces, ShapeCastConfig, SpatialQuery, SpatialQueryFilter,
};
use bevy::color::palettes::tailwind::STONE_700;
use bevy::prelude::*;
use bevy_enhanced_input::actions;
use bevy_enhanced_input::prelude::*;

const PLAYER_HEIGHT: f32 = 2.0;
const PLAYER_WIDTH: f32 = 1.0;
const PLAYER_MASS: f32 = 76.0;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<MeshMaterial<Player>>();

    app.add_input_context::<Player>();
    app.add_observer(on_player_movement);
    app.add_observer(on_player_jump);

    app.add_systems(OnEnter(GameState::Playing), spawn_player);
    app.add_systems(
        FixedUpdate,
        (sync_player_transform, ground_detection).run_if(in_state(GameState::Playing)),
    );
}

fn spawn_player(mut commands: Commands, player_assets: Res<MeshMaterial<Player>>) {
    commands.spawn(player_bundle(player_assets));
}

fn player_bundle(player_assets: Res<MeshMaterial<Player>>) -> impl Bundle {
    (
        Player { lane_index: 0 },
        Grounded::default(),
        Collider::capsule(PLAYER_WIDTH * 0.5, PLAYER_HEIGHT),
        Restitution::new(0.0).with_combine_rule(CoefficientCombine::Min),
        RigidBody::Dynamic,
        Mass(PLAYER_MASS),
        LockedAxes::from_bits(0b001_111),
        Mesh3d(player_assets.mesh.clone()),
        MeshMaterial3d(player_assets.material.clone()),
        Transform::from_xyz(0.0, 3.0, 0.0),
        actions!(
            Player[(
                Action::<Movement>::new(),
                DeadZone::default(),
                Bindings::spawn(Bidirectional::new(KeyCode::KeyD, KeyCode::KeyA)),
            ), (
                Action::<Jump>::new(),
                DeadZone::default(),
                bindings![KeyCode::Space]
            )]
        ),
    )
}

fn on_player_movement(movement: On<Start<Movement>>, mut player: Single<&mut Player>) {
    player.lane_index = (player.lane_index as f32 + movement.value).clamp(-1.0, 1.0) as i32;
}

fn on_player_jump(_event: On<Start<Jump>>, query: Single<(&Grounded, Forces), With<Player>>) {
    let (grounded, mut forces) = query.into_inner();
    info!("Jump attempt with grounded: {:?}", grounded);

    let jump_force_base = 6.0;

    if grounded.time_since_grounded > 0.15 {
        forces.apply_linear_impulse(Vec3::Y * jump_force_base * PLAYER_MASS);
    }
}

fn sync_player_transform(query: Single<(&Transform, Forces, &Player)>) {
    let (transform, mut forces, player) = query.into_inner();

    let kp_base = 200.0;
    let kd_base = 30.0;

    let kp = kp_base * PLAYER_MASS;
    let kd = kd_base * PLAYER_MASS;

    // minus because of player looking at Z instead of NEG_Z
    let target_x = player.lane_index as f32 * -LANE_WIDTH;

    let position_error = target_x - transform.translation.x;
    let velocity_error = -forces.linear_velocity().x;

    let force_x = kp * position_error + kd * velocity_error;

    forces.apply_force(Vec3::X * force_x);
}

fn ground_detection(
    time: Res<Time<Fixed>>,
    spatial_query: SpatialQuery,
    mut query: Query<(Entity, &GlobalTransform, &Collider, &mut Grounded)>,
) {
    for (entity, transform, collider, mut grounded) in &mut query {
        let origin = transform.translation();

        let capsule = collider
            .shape()
            .0
            .as_capsule()
            .expect("Grounded collider must be a capsule");

        let half_height = capsule.half_height();
        let radius = capsule.radius;

        // Probe slightly above the feet
        let skin = 0.02;
        let feet_offset = half_height + radius;
        let cast_origin = origin - Vec3::Y * (feet_offset - skin);

        let probe_radius = radius * 0.9;
        let shape = Collider::sphere(probe_radius);

        let max_distance = 0.02; // allow some buffer
        let config = ShapeCastConfig::from_max_distance(max_distance);

        let filter = SpatialQueryFilter::default().with_excluded_entities([entity]);

        if let Some(hit) = spatial_query.cast_shape(
            &shape,
            cast_origin,
            Quat::IDENTITY,
            Dir3::NEG_Y,
            &config,
            &filter,
        ) {
            // Slope-aware grounded check
            let slope_cos = hit.normal1.dot(Vec3::Y);
            if slope_cos > 0.7 {
                grounded.time_since_grounded += time.delta_secs();
            } else {
                grounded.time_since_grounded = 0.0;
            }
        } else {
            grounded.time_since_grounded = 0.0;
        }
    }
}

#[derive(Component)]
struct Player {
    lane_index: i32,
}

#[derive(Component, Default, Debug)]
pub struct Grounded {
    time_since_grounded: f32,
}

#[derive(Debug, InputAction)]
#[action_output(f32)]
struct Movement;

#[derive(Debug, InputAction)]
#[action_output(bool)]
struct Jump;

impl MeshMaterialConfig for Player {
    fn build_mesh() -> Mesh {
        Mesh::from(Capsule3d::new(PLAYER_WIDTH * 0.5, PLAYER_HEIGHT))
    }
    fn build_material() -> StandardMaterial {
        StandardMaterial::from_color(Color::from(STONE_700))
    }
}
