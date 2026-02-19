use crate::game::scene::DESPAWN_THRESHOLD;
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<CameraSlots>();

    app.add_input_context::<MyCamera>();

    app.add_observer(on_camera_change_slot);

    app.add_systems(Startup, spawn_camera);
}

#[derive(Component)]
#[require(
    Camera3d,
    Transform = init_transform(),
)]
pub struct MyCamera;

pub fn camera_bundle() -> impl Bundle {
    (
        MyCamera,
        actions!(
            MyCamera[(
                Action::<ChangeSlot>::new(),
                DeadZone::default(),
                Bindings::spawn((Bidirectional::new(KeyCode::ArrowRight, KeyCode::ArrowLeft),)),
            )]
        ),
    )
}

fn init_transform() -> Transform {
    *CameraConfigPosition::all()[0].transform()
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(camera_bundle());
}

#[derive(Resource)]
pub struct CameraSlots {
    pub positions: Vec<CameraConfigPosition>,
    pub active_index: usize,
}

#[derive(Debug, Clone)]
pub enum CameraConfigPosition {
    Front(Transform),
    Back(Transform),
    Right(Transform),
    Left(Transform),
    DevHighAngle(Transform),
    DevTopDown(Transform),
    DevDespawn(Transform),
}

impl CameraConfigPosition {
    pub fn all() -> Vec<Self> {
        let target = Vec3::new(0.0, 2.0, 0.0);

        let base_height = 3.0;
        let distance = 10.0;

        vec![
            // Gameplay perspectives
            CameraConfigPosition::Front(
                Transform::from_xyz(0.0, base_height, -distance).looking_at(target, Vec3::Y),
            ),
            CameraConfigPosition::Back(
                Transform::from_xyz(0.0, base_height, distance).looking_at(target, Vec3::Y),
            ),
            CameraConfigPosition::Right(
                Transform::from_xyz(distance, base_height, 0.0).looking_at(target, Vec3::Y),
            ),
            CameraConfigPosition::Left(
                Transform::from_xyz(-distance, base_height, 0.0).looking_at(target, Vec3::Y),
            ),
            // Developer inspection cameras

            // 1. High 3/4 inspection view
            CameraConfigPosition::DevHighAngle(
                Transform::from_xyz(distance, 8.0, -distance).looking_at(target, Vec3::Y),
            ),
            // 2. True top-down debugging view
            CameraConfigPosition::DevTopDown(
                Transform::from_xyz(0.0, 20.0, 0.0).looking_at(target, Vec3::Z),
            ),
            CameraConfigPosition::DevDespawn(
                Transform::from_xyz(0.0, 20.0, -DESPAWN_THRESHOLD).looking_at(target, Vec3::NEG_Z),
            ),
        ]
    }

    pub fn transform(&self) -> &Transform {
        match self {
            CameraConfigPosition::Front(t)
            | CameraConfigPosition::Back(t)
            | CameraConfigPosition::Right(t)
            | CameraConfigPosition::Left(t)
            | CameraConfigPosition::DevHighAngle(t)
            | CameraConfigPosition::DevTopDown(t)
            | CameraConfigPosition::DevDespawn(t) => t,
        }
    }
}

impl Default for CameraSlots {
    fn default() -> Self {
        Self {
            positions: CameraConfigPosition::all(),
            active_index: 0,
        }
    }
}

impl CameraSlots {
    fn active(&self) -> &CameraConfigPosition {
        &self.positions[self.active_index]
    }

    fn next(&mut self) {
        self.active_index = (self.active_index + 1) % self.positions.len();
    }

    fn previous(&mut self) {
        self.active_index = (self.active_index + self.positions.len() - 1) % self.positions.len();
    }
}

#[derive(Debug, InputAction)]
#[action_output(f32)]
struct ChangeSlot;

fn on_camera_change_slot(
    change_slot: On<Start<ChangeSlot>>,
    mut camera_transform: Single<&mut Transform, With<MyCamera>>,
    mut camera_slots: ResMut<CameraSlots>,
) {
    if change_slot.value == -1.0 {
        camera_slots.previous();
    } else if change_slot.value == 1.0 {
        info!("Next");
        camera_slots.next();
    }
    **camera_transform = *camera_slots.active().transform();
}
