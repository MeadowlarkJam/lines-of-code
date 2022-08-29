use crate::camera::{MainCamera, RandomNumberResource};

use super::Starfield;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::{Material2d, Mesh2dHandle};
use bevy::{prelude::Transform, sprite::MaterialMesh2dBundle};

pub fn spawn_starfield_system(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    random: Res<RandomNumberResource>,
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
            material: material_handle.clone(),
            transform: Transform::from_xyz(random.rand1, random.rand2, -1.0),
            ..default()
        })
        .insert(Starfield {
            handle: material_handle,
        });
}

pub fn update_starfield_size_system(
    mut windows: ResMut<Windows>,
    mut meshes: ResMut<Assets<Mesh>>,
    q_mesh: Query<&Mesh2dHandle, With<Starfield>>,
    q_camera: Query<&Transform, With<MainCamera>>,
) {
    let window = windows.get_primary_mut().unwrap();
    let starfield_handle = q_mesh.single();
    let camera = q_camera.single();

    if let Some(mesh) = meshes.get_mut(&starfield_handle.0) {
        *mesh = Mesh::from(shape::Quad::new(Vec2::new(
            window.physical_width() as f32 * camera.scale.x,
            window.physical_height() as f32 * camera.scale.x,
        )));
    }
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
