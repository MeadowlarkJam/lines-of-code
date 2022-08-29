use bevy::{
    prelude::*,
    window::{PresentMode, WindowMode, WindowResizeConstraints},
};

#[derive(Debug, PartialEq, Eq, Clone, Hash, SystemLabel)]
pub struct WindowSystem;

pub struct WindowPlugin;

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WindowDescriptor {
            title: String::from("Escape Pod"),
            width: 1280.0,
            height: 720.0,
            position: WindowPosition::Automatic,
            resize_constraints: WindowResizeConstraints::default(),
            scale_factor_override: None,
            present_mode: PresentMode::Fifo,
            resizable: true,
            decorations: true,
            cursor_locked: false,
            cursor_visible: true,
            mode: WindowMode::Windowed,
            transparent: false,
            canvas: None,
            fit_canvas_to_parent: false,
        })
        .insert_resource(ClearColor(Color::BLACK));
    }
}
