pub mod components;
pub mod hierarchy;
pub mod transform_propagate_system;

pub mod prelude {
    pub use crate::{components::*, hierarchy::*, TransformPlugin};
}

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_type_registry::RegisterType;
use prelude::{Children, Parent, Transform};

pub(crate) fn transform_systems() -> Vec<Box<dyn System>> {
    let mut systems = Vec::with_capacity(5);

    systems.append(&mut hierarchy::hierarchy_maintenance_systems());
    systems.push(transform_propagate_system::transform_propagate_system.system());

    systems
}

#[derive(Default)]
pub struct TransformPlugin;

impl Plugin for TransformPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.register_component::<Children>()
            .register_component::<Parent>()
            .register_component::<Transform>()
            // add transform systems to startup so the first update is "correct"
            .add_startup_systems(transform_systems())
            .add_systems_to_stage(stage::POST_UPDATE, transform_systems());

            // TODO: DELETEME: It looks like this doesn't improve things, so it isn't the issue. -Nick 2020-09-18
            // // // Add systems to the front of POST_UPDATE so that it updates BEFORE the render system. (In theory, this could fix an issue where there's perceived input latency due to an order-update issue where the render fails to account for the changed transform by a single frame?)
            // // .add_systems_to_stage_front(stage::POST_UPDATE, transform_systems());
    }
}
