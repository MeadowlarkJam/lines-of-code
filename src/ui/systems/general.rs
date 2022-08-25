use crate::consts::{COLOR_BUTTON_CLICKED, COLOR_BUTTON_DEFAULT, COLOR_BUTTON_HOVERED};
use bevy::prelude::*;

pub fn button_highlight_system(
    mut query: Query<(&Interaction, &mut UiColor), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut color) in query.iter_mut() {
        *color = match *interaction {
            Interaction::Hovered => COLOR_BUTTON_HOVERED.into(),
            Interaction::Clicked => COLOR_BUTTON_CLICKED.into(),
            _ => COLOR_BUTTON_DEFAULT.into(),
        }
    }
}
