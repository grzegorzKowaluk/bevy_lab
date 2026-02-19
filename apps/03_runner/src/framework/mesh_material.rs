use bevy::prelude::*;
use std::marker::PhantomData;

pub trait MeshMaterialConfig {
    fn build_mesh() -> Mesh;
    fn build_material() -> StandardMaterial;
}

#[derive(Resource)]
pub struct MeshMaterial<T: Component> {
    _marker: PhantomData<T>,
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}

impl<T: Component + MeshMaterialConfig> FromWorld for MeshMaterial<T> {
    fn from_world(world: &mut World) -> Self {
        let mut meshes = world
            .get_resource_mut::<Assets<Mesh>>()
            .expect("Assets<Mesh> not found");

        let mesh_handle = meshes.add(T::build_mesh());

        let mut materials = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .expect("Assets<StandardMaterial> not found");

        let material_handle = materials.add(T::build_material());

        MeshMaterial {
            _marker: PhantomData,
            mesh: mesh_handle,
            material: material_handle,
        }
    }
}
