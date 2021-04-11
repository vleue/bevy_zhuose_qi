use bevy::{
    core::Bytes,
    prelude::*,
    reflect::TypeUuid,
    render::{
        pipeline::{PipelineDescriptor, RenderPipeline},
        render_graph::{
            base::{self, MainPass},
            RenderGraph, RenderResourcesNode,
        },
        renderer::{RenderResource, RenderResourceType, RenderResources},
        shader::{ShaderStage, ShaderStages},
    },
    sprite::QUAD_HANDLE,
};

pub const LASER_COLOR: Color = Color::rgb(0.9245, 0.3224, 0.0654);

#[derive(RenderResources, TypeUuid, Clone)]
#[render_resources(from_self)]
#[uuid = "0fa4eadf-940f-4c21-b1cd-59e608107604"]
#[repr(C)]
pub struct LaserMaterial {
    pub base_color: Color,
    pub width: f32,
    pub time: f32,
}

impl Default for LaserMaterial {
    fn default() -> Self {
        LaserMaterial {
            base_color: LASER_COLOR,
            width: 2.0,
            time: 0.0,
        }
    }
}

impl RenderResource for LaserMaterial {
    fn resource_type(&self) -> Option<RenderResourceType> {
        Some(RenderResourceType::Buffer)
    }

    fn buffer_byte_len(&self) -> Option<usize> {
        Some(24)
    }

    fn write_buffer_bytes(&self, buffer: &mut [u8]) {
        let (base_color_buf, rest) = buffer.split_at_mut(16);
        self.base_color.write_bytes(base_color_buf);

        let (width_buf, rest) = rest.split_at_mut(4);
        self.width.write_bytes(width_buf);

        self.time.write_bytes(rest);
    }

    fn texture(&self) -> Option<&Handle<Texture>> {
        None
    }
}

pub struct LaserPlugin;

impl Plugin for LaserPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset::<LaserMaterial>()
            .add_startup_system(setup.system())
            .add_system(animate_laser.system());
    }
}

pub const LASER_PIPELINE_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(PipelineDescriptor::TYPE_UUID, 2785347855338765446);

fn setup(
    mut commands: Commands,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut render_graph: ResMut<RenderGraph>,
) {
    pipelines.set_untracked(
        LASER_PIPELINE_HANDLE,
        #[cfg(not(target_arch = "wasm32"))]
        PipelineDescriptor::default_config(ShaderStages {
            vertex: shaders.add(Shader::from_glsl(
                ShaderStage::Vertex,
                include_str!("laser.vert"),
            )),
            fragment: Some(shaders.add(Shader::from_glsl(
                ShaderStage::Fragment,
                include_str!("laser.frag"),
            ))),
        }),
        #[cfg(target_arch = "wasm32")]
        PipelineDescriptor::default_config(ShaderStages {
            vertex: shaders.add(Shader::from_glsl(
                ShaderStage::Vertex,
                include_str!("es.laser.vert"),
            )),
            fragment: Some(shaders.add(Shader::from_glsl(
                ShaderStage::Fragment,
                include_str!("es.laser.frag"),
            ))),
        }),
    );

    render_graph.add_system_node(
        "laser_material",
        RenderResourcesNode::<LaserMaterial>::new(true),
    );
    render_graph
        .add_node_edge("laser_material", base::node::MAIN_PASS)
        .unwrap();

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn animate_laser(time: Res<Time>, mut query: Query<&mut LaserMaterial>) {
    for mut laser_material in query.iter_mut() {
        laser_material.time = time.seconds_since_startup() as f32;
    }
}

#[derive(Bundle)]
pub struct LaserBundle {
    pub mesh: Handle<Mesh>,
    pub laser_material: LaserMaterial,
    pub draw: Draw,
    pub visible: Visible,
    pub render_pipelines: RenderPipelines,
    pub main_pass: MainPass,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl Default for LaserBundle {
    fn default() -> Self {
        Self {
            mesh: QUAD_HANDLE.typed(),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                LASER_PIPELINE_HANDLE.typed(),
            )]),
            visible: Visible {
                is_transparent: true,
                ..Default::default()
            },
            main_pass: MainPass,
            draw: Default::default(),
            laser_material: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
        }
    }
}
