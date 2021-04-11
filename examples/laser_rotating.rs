use std::f32::consts::PI;

use bevy::{prelude::*, render::mesh::shape};

use bevy_zhuose_qi::laser::*;

pub fn main() {
    let mut builder = App::build();
    builder.add_plugins(DefaultPlugins);

    #[cfg(target_arch = "wasm32")]
    builder.add_plugin(bevy_webgl2::WebGL2Plugin::default());

    builder
        .add_plugin(LaserPlugin)
        .add_startup_system(setup.system())
        .add_system(rotate.system())
        .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    commands.spawn_bundle(LaserBundle {
        mesh: meshes.add(Mesh::from(shape::Quad {
            size: Vec2::new(100.0, 1500.0),
            ..Default::default()
        })),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            rotation: Quat::from_rotation_z(0.0 * PI / 3.0),
            ..Default::default()
        },
        laser_material: LaserMaterial {
            base_color: Color::RED,
            width: 10.0,
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn_bundle(LaserBundle {
        mesh: meshes.add(Mesh::from(shape::Quad {
            size: Vec2::new(100.0, 1500.0),
            ..Default::default()
        })),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.1),
            rotation: Quat::from_rotation_z(1.0 * PI / 3.0),
            ..Default::default()
        },
        laser_material: LaserMaterial {
            base_color: Color::GREEN,
            width: 10.0,
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn_bundle(LaserBundle {
        mesh: meshes.add(Mesh::from(shape::Quad {
            size: Vec2::new(100.0, 1500.0),
            ..Default::default()
        })),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.2),
            rotation: Quat::from_rotation_z(2.0 * PI / 3.0),
            ..Default::default()
        },
        laser_material: LaserMaterial {
            base_color: Color::BLUE,
            width: 10.0,
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn rotate(mut transforms: Query<&mut Transform, With<LaserMaterial>>, time: Res<Time>) {
    for mut transform in transforms.iter_mut() {
        transform.rotate(Quat::from_rotation_z(time.delta_seconds() * 0.1));
    }
}
