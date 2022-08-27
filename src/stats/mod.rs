mod plugin;
mod resources;
mod systems;

pub use self::{
    plugin::{StatsPlugin, StatsSystem},
    resources::Stats,
};
