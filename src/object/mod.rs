mod components;
mod plugin;
pub(crate) mod systems;

pub use self::{
    components::*,
    plugin::{ObjectPlugin, ObjectSystem},
    systems::move_objects_system,
};
