use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::Material2d;
use bevy::{prelude::Transform, sprite::MaterialMesh2dBundle};

use super::Starfield;

pub fn spawn_starfield_system(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    asset_server: Res<AssetServer>,
) {
    asset_server.watch_for_changes().unwrap();

    let window = windows.get_primary_mut().unwrap();

    let quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
        window.physical_width() as f32,
        window.physical_height() as f32,
    ))));

    let material_handle = materials.add(CustomMaterial {
        pos: Vec4::new(0.0, 0.0, 0.0, 0.0),
    });

    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: quad_handle.into(),
            material: material_handle.clone().into(),
            transform: Transform::from_xyz(0.0, 0.0, -1.0),
            ..default()
        })
        .insert(Starfield {
            handle: material_handle,
        });
}

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct CustomMaterial {
    #[uniform(0)]
    pub pos: Vec4,
}

impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/starfield.wgsl".into()
    }
}
