// Case 2 Rust Bevy app
// Enables the Rust shader/SPIR-V path through `bevy-rust-gpu` and renders the cube used in the static color-gradient comparison case.

use bevy::{prelude::*, reflect::TypeUuid, render::render_resource::AsBindGroup};

use bevy_rust_gpu::{
    prelude::{RustGpu, RustGpuMaterialPlugin, RustGpuPlugin},
    EntryPoint, RustGpuMaterial,
};

use bevy::app::AppExit;
use std::time::Duration;

fn main() {
    let mut app = App::default();

    // Use the dark blue background shown in the Case 2 comparison result.
    app.insert_resource(ClearColor(Color::hex("071f3c").unwrap()));

    app.add_plugins(DefaultPlugins);

    // Enable the Rust-GPU path and register the Case 2 material.
    app.add_plugin(RustGpuPlugin::default());
    app.add_plugin(RustGpuMaterialPlugin::<Case2RustMaterial>::default());

    std::fs::create_dir_all("generated/rust-gpu")
        .expect("Failed to create Rust-GPU entry point directory");

    // Export the entry point metadata used by the Rust-GPU pipeline.
    RustGpu::<Case2RustMaterial>::export_to(ENTRY_POINTS_PATH);

    app.add_startup_system(setup);

    // Benchmark helper: close the app automatically after one second.
    app.add_system(exit_system);

    app.run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<RustGpu<Case2RustMaterial>>>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Load the compiled Rust-GPU shader asset.
    let shader = asset_server.load(SHADER_PATH);

    // Render the comparison cube with the Case 2 Rust material.
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        material: materials.add(RustGpu {
            vertex_shader: Some(shader.clone()),
            fragment_shader: Some(shader),
            ..default()
        }),
        ..default()
    });

    // Benchmark helper entity.
    commands.spawn((FuseTime {
        timer: Timer::new(Duration::from_secs(1), TimerMode::Once),
    },));
}

/// Path to the compiled Rust-GPU shader asset used at runtime.
const SHADER_PATH: &'static str = "rust-gpu/shader.rust-gpu.msgpack";

/// Path to the exported Rust-GPU entry point metadata.
const ENTRY_POINTS_PATH: &'static str = "generated/rust-gpu/entry_points.json";

pub enum VertexMainCase2 {}

impl EntryPoint for VertexMainCase2 {
    const NAME: &'static str = "case2::vertex_main_case2";
}

pub enum FragmentMainCase2 {}

impl EntryPoint for FragmentMainCase2 {
    const NAME: &'static str = "case2::fragment_main_case2";
}

#[derive(Debug, Default, Copy, Clone, AsBindGroup, TypeUuid)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct Case2RustMaterial {}

impl Material for Case2RustMaterial {}

impl RustGpuMaterial for Case2RustMaterial {
    type Vertex = VertexMainCase2;
    type Fragment = FragmentMainCase2;
}

#[derive(Component)]
struct FuseTime {
    timer: Timer,
}

fn exit_system(mut exit: EventWriter<AppExit>, mut q: Query<&mut FuseTime>, time: Res<Time>) {
    for mut fuse_timer in q.iter_mut() {
        fuse_timer.timer.tick(time.delta());

        if fuse_timer.timer.finished() {
            exit.send(AppExit);
        }
    }
}
