mod components;
mod plugin;
pub(crate) mod systems;

pub use self::{
    components::*,
    systems::move_objects_system,
    plugin::{ObjectPlugin, ObjectSystem},
};
