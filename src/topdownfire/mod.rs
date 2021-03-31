use bevy::{
    app::{Events, ManualEventReader},
    core::Bytes,
    prelude::*,
    reflect::TypeUuid,
    render::{
        pipeline::{PipelineDescriptor, RenderPipeline},
        render_graph::{
            base::{self, MainPass},
            AssetRenderResourcesNode, RenderGraph, RenderResourcesNode,
        },
        renderer::{RenderResource, RenderResourceType, RenderResources},
        shader::{ShaderStage, ShaderStages},
    },
    sprite::QUAD_HANDLE,
    utils::{HashMap, HashSet},
};

pub const FIRE_COLOR: Color = Color::rgb(0.9245, 0.3224, 0.0654);
pub const FIRE_INTENSITY_1: u8 = 255;
pub const FIRE_INTENSITY_2: u8 = 204;
pub const FIRE_INTENSITY_3: u8 = 153;

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "93fb26fc-6c05-489b-9029-601edf703b6b"]
pub struct FireTexture {
    pub texture: Handle<Texture>,
}

impl From<Handle<Texture>> for FireTexture {
    fn from(texture: Handle<Texture>) -> Self {
        FireTexture { texture }
    }
}

#[derive(RenderResources, TypeUuid, Clone)]
#[render_resources(from_self)]
#[uuid = "539fe49d-df51-48c1-bbfc-d68eb1716354"]
#[repr(C)]
pub struct FireMaterial {
    pub base_color: Color,
    pub flame_height: f32,
    pub distorsion_level: f32,
    pub time: f32,
}

impl Default for FireMaterial {
    fn default() -> Self {
        FireMaterial {
            base_color: FIRE_COLOR,
            flame_height: 0.2,
            distorsion_level: 7.0,
            time: 0.0,
        }
    }
}

impl RenderResource for FireMaterial {
    fn resource_type(&self) -> Option<RenderResourceType> {
        Some(RenderResourceType::Buffer)
    }

    fn buffer_byte_len(&self) -> Option<usize> {
        Some(28)
    }

    fn write_buffer_bytes(&self, buffer: &mut [u8]) {
        let (base_color_buf, rest) = buffer.split_at_mut(16);
        self.base_color.write_bytes(base_color_buf);

        let (flame_height_buf, rest) = rest.split_at_mut(4);
        self.flame_height.write_bytes(flame_height_buf);

        let (distorsion_level_buf, rest) = rest.split_at_mut(4);
        self.distorsion_level.write_bytes(distorsion_level_buf);

        self.time.write_bytes(rest);
    }

    fn texture(&self) -> Option<&Handle<Texture>> {
        None
    }
}

pub struct FirePlugin;

impl Plugin for FirePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset::<FireMaterial>()
            .add_asset::<FireTexture>()
            .add_startup_system(setup.system())
            .add_system(animate_fire.system())
            .add_system(fire_texture_detection_system.system());
    }
}

pub const FIRE_PIPELINE_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(PipelineDescriptor::TYPE_UUID, 2785347845338765446);

fn setup(
    mut commands: Commands,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut render_graph: ResMut<RenderGraph>,
) {
    pipelines.set_untracked(
        FIRE_PIPELINE_HANDLE,
        #[cfg(not(target_arch = "wasm32"))]
        PipelineDescriptor::default_config(ShaderStages {
            vertex: shaders.add(Shader::from_glsl(
                ShaderStage::Vertex,
                include_str!("fire.vert"),
            )),
            fragment: Some(shaders.add(Shader::from_glsl(
                ShaderStage::Fragment,
                include_str!("fire.frag"),
            ))),
        }),
        #[cfg(target_arch = "wasm32")]
        PipelineDescriptor::default_config(ShaderStages {
            vertex: shaders.add(Shader::from_glsl(
                ShaderStage::Vertex,
                include_str!("es.fire.vert"),
            )),
            fragment: Some(shaders.add(Shader::from_glsl(
                ShaderStage::Fragment,
                include_str!("es.fire.frag"),
            ))),
        }),
    );

    render_graph.add_system_node(
        "fire_texture",
        AssetRenderResourcesNode::<FireTexture>::new(true),
    );
    render_graph
        .add_node_edge("fire_texture", base::node::MAIN_PASS)
        .unwrap();

    render_graph.add_system_node(
        "fire_material",
        RenderResourcesNode::<FireMaterial>::new(true),
    );
    render_graph
        .add_node_edge("fire_material", base::node::MAIN_PASS)
        .unwrap();

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

#[allow(clippy::type_complexity)]
fn fire_texture_detection_system(
    mut texture_to_fire: Local<HashMap<Handle<Texture>, HashSet<Handle<FireTexture>>>>,
    mut fire_to_texture: Local<HashMap<Handle<FireTexture>, Handle<Texture>>>,
    materials: Res<Assets<FireTexture>>,
    mut texture_events: EventReader<AssetEvent<Texture>>,
    (mut fire_events_reader, mut fire_events): (
        Local<ManualEventReader<AssetEvent<FireTexture>>>,
        ResMut<Events<AssetEvent<FireTexture>>>,
    ),
) {
    for event in fire_events_reader.iter(&fire_events) {
        match event {
            AssetEvent::Created { handle } => {
                if let Some(texture) = materials.get(handle).map(|mat| &mat.texture) {
                    fire_to_texture.insert(handle.clone_weak(), texture.clone_weak());
                    texture_to_fire
                        .entry(texture.clone_weak())
                        .or_default()
                        .insert(handle.clone_weak());
                }
            }
            AssetEvent::Modified { handle } => {
                let old_texture = fire_to_texture.get(handle).cloned();
                match (materials.get(handle).map(|mat| &mat.texture), old_texture) {
                    (None, None) => (),
                    (Some(texture), None) => {
                        fire_to_texture.insert(handle.clone_weak(), texture.clone_weak());
                        texture_to_fire
                            .entry(texture.clone_weak())
                            .or_default()
                            .insert(handle.clone_weak());
                    }
                    (None, Some(texture)) => {
                        fire_to_texture.remove(handle);
                        texture_to_fire
                            .entry(texture.clone_weak())
                            .or_default()
                            .remove(handle);
                    }
                    (Some(new_texture), Some(old_texture)) => {
                        if &old_texture == new_texture {
                            continue;
                        }
                        fire_to_texture.insert(handle.clone_weak(), new_texture.clone_weak());
                        texture_to_fire
                            .entry(new_texture.clone_weak())
                            .or_default()
                            .insert(handle.clone_weak());
                        texture_to_fire
                            .entry(old_texture.clone_weak())
                            .or_default()
                            .remove(handle);
                    }
                }
            }
            AssetEvent::Removed { handle } => {
                if let Some(texture) = materials.get(handle).map(|mat| &mat.texture) {
                    fire_to_texture.remove(handle);
                    texture_to_fire
                        .entry(texture.clone_weak())
                        .or_default()
                        .remove(handle);
                }
            }
        }
    }

    let mut changed_textures = HashSet::default();
    for event in texture_events.iter() {
        match event {
            AssetEvent::Created { handle }
            | AssetEvent::Modified { handle }
            | AssetEvent::Removed { handle } => {
                changed_textures.insert(handle);
            }
        }
    }

    for texture_handle in changed_textures.iter() {
        if let Some(materials) = texture_to_fire.get(texture_handle) {
            for material in materials.iter() {
                fire_events.send(AssetEvent::Modified {
                    handle: material.clone_weak(),
                });
            }
        }
    }
}
fn animate_fire(time: Res<Time>, mut query: Query<&mut FireMaterial>) {
    for mut fire_material in query.iter_mut() {
        fire_material.time = time.seconds_since_startup() as f32;
    }
}

#[derive(Bundle)]
pub struct FireBundle {
    pub fire_texture: Handle<FireTexture>,
    pub mesh: Handle<Mesh>,
    pub fire_material: FireMaterial,
    pub draw: Draw,
    pub visible: Visible,
    pub render_pipelines: RenderPipelines,
    pub main_pass: MainPass,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl Default for FireBundle {
    fn default() -> Self {
        Self {
            mesh: QUAD_HANDLE.typed(),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                FIRE_PIPELINE_HANDLE.typed(),
            )]),
            visible: Visible {
                is_transparent: true,
                ..Default::default()
            },
            main_pass: MainPass,
            draw: Default::default(),
            fire_material: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            fire_texture: Default::default(),
        }
    }
}
