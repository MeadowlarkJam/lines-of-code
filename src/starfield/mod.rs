mod components;
mod plugin;
mod systems;

pub use self::{
    components::Starfield, plugin::StarfieldPlugin, systems::spawn_starfield_system,
    systems::CustomMaterial,
};
