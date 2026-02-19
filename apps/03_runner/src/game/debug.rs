use crate::framework::mesh_material::{MeshMaterial, MeshMaterialConfig};
use bevy::color::palettes::basic::YELLOW;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<MeshMaterial<DebugSphere>>();
}

fn cleanup_debug_objects(mut commands: Commands, query: Query<Entity, With<DebugSphere>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

#[derive(Component)]
#[require(Transform)]
pub struct DebugSphere;

pub fn debug_sphere_bundle(debug_assets: &Res<MeshMaterial<DebugSphere>>) -> impl Bundle {
    (
        DebugSphere,
        Mesh3d(debug_assets.mesh.clone()),
        MeshMaterial3d(debug_assets.material.clone()),
    )
}

impl MeshMaterialConfig for DebugSphere {
    fn build_mesh() -> Mesh {
        Mesh::from(Sphere::new(10.0))
    }
    fn build_material() -> StandardMaterial {
        StandardMaterial {
            base_color: Color::Srgba(YELLOW),
            alpha_mode: AlphaMode::Blend,
            ..Default::default()
        }
    }
}
