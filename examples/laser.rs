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
        .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    commands.spawn_bundle(LaserBundle {
        mesh: meshes.add(Mesh::from(shape::Quad {
            size: Vec2::new(1280.0, 768.0),
            ..Default::default()
        })),
        ..Default::default()
    });

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
