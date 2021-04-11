use bevy::{
    prelude::*,
    render::{
        mesh::shape,
        texture::{Extent3d, TextureDimension, TextureFormat},
    },
};

use bevy_zhuose_qi::topdownfire::*;
use rand::Rng;

pub fn main() {
    let mut builder = App::build();
    builder.add_plugins(DefaultPlugins);

    #[cfg(target_arch = "wasm32")]
    builder.add_plugin(bevy_webgl2::WebGL2Plugin::default());

    builder
        .add_plugin(FirePlugin)
        .add_startup_system(setup.system())
        .add_system(wild_fire.system())
        .run();
}

const FIRE_SIZE: usize = 1000;

fn setup(
    mut commands: Commands,
    mut textures: ResMut<Assets<Texture>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut fire_textures: ResMut<Assets<FireTexture>>,
) {
    let mut data = Vec::new();
    for _ in 0..FIRE_SIZE * FIRE_SIZE {
        data.push(0);
    }

    let texture = Texture::new(
        Extent3d::new(FIRE_SIZE as u32, FIRE_SIZE as u32, 1),
        TextureDimension::D2,
        data,
        TextureFormat::R8Unorm,
    );

    let tex_handle = textures.add(texture);

    commands.spawn_bundle(FireBundle {
        mesh: meshes.add(Mesh::from(shape::Quad {
            size: Vec2::new(FIRE_SIZE as f32, FIRE_SIZE as f32),
            ..Default::default()
        })),
        fire_texture: fire_textures.add(tex_handle.clone().into()),
        fire_material: FireMaterial {
            flame_height: 0.1,
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands.insert_resource(tex_handle);
}

// very not-optimized, much useless iterations
fn brush(target_x: u32, target_y: u32, brush_size: f32, data: &mut Vec<u8>) {
    let center = Vec2::new(target_x as f32, target_y as f32);
    let center_index = target_x as usize + target_y as usize * FIRE_SIZE;
    for n in (center_index
        .checked_sub(brush_size as usize * FIRE_SIZE)
        .unwrap_or(0))
        ..(center_index + brush_size as usize * FIRE_SIZE).min(FIRE_SIZE * FIRE_SIZE)
    {
        let x = n % FIRE_SIZE;
        let y = n / FIRE_SIZE;
        let point = Vec2::new(x as f32, y as f32);
        if center.distance(point) < brush_size / 4.0 {
            data[n] = FIRE_INTENSITY_1;
        } else if center.distance(point) < brush_size / 1.7 {
            if data[n] < FIRE_INTENSITY_2 {
                data[n] = FIRE_INTENSITY_2;
            }
        } else if center.distance(point) < brush_size {
            if data[n] == 0 {
                data[n] = FIRE_INTENSITY_3;
            }
        }
    }
}

fn wild_fire(
    mut timer: Local<Option<Timer>>,
    time: Res<Time>,
    texture_handle: Res<Handle<Texture>>,
    mut textures: ResMut<Assets<Texture>>,
) {
    if timer.is_none() {
        *timer = Some(Timer::from_seconds(1.0, true));
    }

    if timer.as_mut().unwrap().tick(time.delta()).just_finished() {
        if let Some(texture) = textures.get_mut(texture_handle.clone()) {
            let x = rand::thread_rng().gen_range(0..FIRE_SIZE as u32);
            let y = rand::thread_rng().gen_range(0..FIRE_SIZE as u32);

            brush(x, y, FIRE_SIZE as f32 / 12.0, &mut texture.data);
        }
    }
}
