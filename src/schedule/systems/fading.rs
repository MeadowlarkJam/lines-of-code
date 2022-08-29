use crate::schedule::ScheduleTimer;
use bevy::prelude::*;

pub fn fade_out_system(
    timer: Res<ScheduleTimer>,
    mut q_ui_colors: Query<&mut UiColor, With<Button>>,
    mut q_text: Query<&mut Text>,
    mut q_sprites: Query<&mut Sprite>,
) {
    let alpha = 1.0 - timer.0.elapsed().as_secs_f32() / timer.0.duration().as_secs_f32();

    for mut text in &mut q_text {
        for section in &mut text.sections {
            section.style.color.set_a(alpha);
        }
    }

    for mut sprite in &mut q_sprites {
        sprite.color.set_a(alpha);
    }

    for mut ui_color in &mut q_ui_colors {
        ui_color.0.set_a(alpha);
    }
}

pub fn fade_in_system(
    timer: Res<ScheduleTimer>,
    mut q_ui_colors: Query<&mut UiColor, With<Button>>,
    mut q_text: Query<&mut Text>,
    mut q_sprites: Query<&mut Sprite>,
) {
    let alpha = timer.0.elapsed().as_secs_f32() / timer.0.duration().as_secs_f32();

    for mut text in &mut q_text {
        for section in &mut text.sections {
            section.style.color.set_a(alpha);
        }
    }

    for mut sprite in &mut q_sprites {
        sprite.color.set_a(alpha);
    }

    for mut ui_color in &mut q_ui_colors {
        ui_color.0.set_a(alpha);
    }
}
