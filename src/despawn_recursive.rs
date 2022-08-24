use bevy::prelude::*;

/// Despawns every entity with a specific component recursively.
pub fn despawn_entities_recursive_system<T: Component>(
    mut commands: Commands,
    query: Query<Entity, With<T>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
