//! A high-level way to load collections of asset handles as resources.

use crate::AssetState;
use bevy::asset::{DependencyLoadState, LoadState, RecursiveDependencyLoadState};
use bevy::prelude::*;
use std::collections::VecDeque;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<ResourceHandles>();
    app.add_systems(
        PreUpdate,
        load_resource_assets.run_if(in_state(AssetState::Loading)),
    );
}

pub trait LoadResource {
    /// This will load the [`Resource`] as an [`Asset`]. When all of its asset dependencies
    /// have been loaded, it will be inserted as a resource. This ensures that the resource only
    /// exists when the assets are ready.
    fn load_resource<T: Resource + Asset + Clone + FromWorld>(&mut self) -> &mut Self;
}

impl LoadResource for App {
    fn load_resource<T: Resource + Asset + Clone + FromWorld>(&mut self) -> &mut Self {
        self.init_asset::<T>();
        let world = self.world_mut();
        let value = T::from_world(world);
        let assets = world.resource::<AssetServer>();
        let handle = assets.add(value);
        let mut handles = world.resource_mut::<ResourceHandles>();
        handles
            .waiting
            .push_back((handle.untyped(), |world, handle| {
                let assets = world.resource::<Assets<T>>();
                if let Some(value) = assets.get(handle.id().typed::<T>()) {
                    world.insert_resource(value.clone());
                }
            }));
        self
    }
}

/// A function that inserts a loaded resource.
type InsertLoadedResource = fn(&mut World, &UntypedHandle);

pub enum ResourceLoadState {
    Loading,
    Done,
    Failed(Vec<String>), // list of failed asset paths
}

#[derive(Resource, Default)]
pub struct ResourceHandles {
    // Use a queue for waiting assets so they can be cycled through and moved to
    // `finished` one at a time.
    waiting: VecDeque<(UntypedHandle, InsertLoadedResource)>,
    finished: Vec<UntypedHandle>,
    failed: Vec<(UntypedHandle, String)>,
}

impl ResourceHandles {
    pub fn status(&self) -> ResourceLoadState {
        if !self.failed.is_empty() {
            let errors: Vec<String> = self
                .failed
                .iter()
                .map(|(_, error)| error.to_string())
                .collect();
            ResourceLoadState::Failed(errors)
        } else if self.waiting.is_empty() {
            ResourceLoadState::Done
        } else {
            ResourceLoadState::Loading
        }
    }
}

fn load_resource_assets(world: &mut World) {
    world.resource_scope(|world, mut resource_handles: Mut<ResourceHandles>| {
        world.resource_scope(|world, assets: Mut<AssetServer>| {
            for _ in 0..resource_handles.waiting.len() {
                let (handle, insert_fn) = resource_handles.waiting.pop_front().unwrap();
                let Some(load_states) = assets.get_load_states(&handle) else {
                    resource_handles.waiting.push_back((handle, insert_fn));
                    continue;
                };

                match load_states {
                    (
                        LoadState::Loaded,
                        DependencyLoadState::Loaded,
                        RecursiveDependencyLoadState::Loaded,
                    ) => {
                        info!("Asset loaded");
                        insert_fn(world, &handle);
                        resource_handles.finished.push(handle);
                    }
                    (LoadState::Failed(error), _, _) => {
                        error!("Asset loading failed");
                        resource_handles.failed.push((handle, error.to_string()));
                    }
                    (_, DependencyLoadState::Failed(error), _) => {
                        error!("Dependency Asset loading failed");
                        resource_handles.failed.push((handle, error.to_string()));
                    }
                    _ => {
                        resource_handles.waiting.push_back((handle, insert_fn));
                    }
                }
            }
        });
    });
}
