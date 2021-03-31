use bevy::{prelude::*, render::mesh::shape};

use bevy_zhuose_qi::topdownfire::*;

pub fn main() {
    let mut builder = App::build();
    builder.add_plugins(DefaultPlugins);

    #[cfg(target_arch = "wasm32")]
    builder.add_plugin(bevy_webgl2::WebGL2Plugin::default());

    builder
        .add_plugin(FirePlugin)
        .add_startup_system(setup.system())
        .run();
}

const FIRE_SIZE: usize = 100;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut fire_textures: ResMut<Assets<FireTexture>>,
) {
    let tex_handle = asset_server.load("fire.png");
    let mesh_handle = meshes.add(Mesh::from(shape::Quad {
        size: Vec2::new(FIRE_SIZE as f32, FIRE_SIZE as f32),
        ..Default::default()
    }));

    let mut x = -(FIRE_SIZE as f32) * 5.0;
    for flame_height in (-10..=10).step_by(2) {
        let mut y = -(FIRE_SIZE as f32) * 5.0;
        for distortion in (-10..=10).step_by(2) {
            commands.spawn_bundle(FireBundle {
                mesh: mesh_handle.clone(),
                fire_material: FireMaterial {
                    flame_height: flame_height as f32 / 10.0,
                    distorsion_level: distortion as f32,
                    ..Default::default()
                },
                fire_texture: fire_textures.add(tex_handle.clone().into()),
                transform: Transform::from_xyz(x, y, 0.0),
                ..Default::default()
            });
            y += FIRE_SIZE as f32;
        }
        x += FIRE_SIZE as f32;
    }

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
