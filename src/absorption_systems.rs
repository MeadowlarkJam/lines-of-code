use bevy::prelude::*;

use crate::components::*;

pub fn check_attachment(
    mut commands: Commands,
    player_query: Query<(&mut GlobalTransform), (With<Player>, Without<PlayerRoot>)>,
    mut player_root_query: Query<(Entity, &Transform, &mut Stats), With<PlayerRoot>>,
    mut attachable_query: Query<
        (Entity, &mut Transform),
        (With<Object>, Without<Player>, Without<PlayerRoot>),
    >,
) {
    // We need the transform of the root, since everything is relative to it and when adding children we need to revert it first
    let (root_entity, root_transform, mut root_stats) = player_root_query.get_single_mut().unwrap();

    for player_transform in player_query.iter() {
        for (attachable_entity, mut attachable_transform) in attachable_query.iter_mut() {
            let distance = player_transform
                .compute_transform()
                .translation
                .distance(attachable_transform.translation);
            // TODO: More exact collision detection
            if distance
                < ((player_transform.compute_transform().scale.x + attachable_transform.scale.x)
                    / 2.)
                    * 8.
            {
                // We only check for attachments using the children, since we created a seperate child
                // on top of the root in the beginning
                commands.entity(root_entity).add_child(attachable_entity);
                commands.entity(attachable_entity).insert(Player {});
                commands.entity(attachable_entity).remove::<Object>();

                // The new translations are offsets from the parent
                attachable_transform.translation = Vec3::new(
                    (attachable_transform.translation.x - root_transform.translation.x),
                    (attachable_transform.translation.y - root_transform.translation.y),
                    0.,
                );

                root_stats.size += 1;
                root_stats.health += 10;
            }
        }
    }
}
