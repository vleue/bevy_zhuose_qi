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

const FIRE_SIZE: usize = 400;

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

    commands.spawn_bundle(FireBundle {
        mesh: mesh_handle.clone(),
        fire_material: FireMaterial {
            base_color: Color::RED,
            ..Default::default()
        },
        fire_texture: fire_textures.add(tex_handle.clone().into()),
        transform: Transform::from_xyz(-400.0, 0.0, 0.0),
        ..Default::default()
    });
    commands.spawn_bundle(FireBundle {
        mesh: mesh_handle.clone(),
        fire_material: FireMaterial {
            base_color: Color::GREEN,
            ..Default::default()
        },
        fire_texture: fire_textures.add(tex_handle.clone().into()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });
    commands.spawn_bundle(FireBundle {
        mesh: mesh_handle.clone(),
        fire_material: FireMaterial {
            base_color: Color::BLUE,
            ..Default::default()
        },
        fire_texture: fire_textures.add(tex_handle.clone().into()),
        transform: Transform::from_xyz(400.0, 0.0, 0.0),
        ..Default::default()
    });

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
