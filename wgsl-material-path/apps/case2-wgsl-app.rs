// Case 2 WGSL Bevy app
// Registers the WGSL material path and renders the cube used in the static color-gradient comparison case.

use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
};

use bevy::app::AppExit;
use std::time::Duration;

fn main() {
    let mut app = App::default();

    // Use the dark blue background shown in the Case 2 comparison result.
    app.insert_resource(ClearColor(Color::hex("071f3c").unwrap()));

    app.add_plugins(DefaultPlugins);
    app.add_plugin(MaterialPlugin::<Case2WgslMaterial>::default());

    app.add_startup_system(setup);

    // Benchmark helper: close the app automatically after one second.
    app.add_system(exit_system);

    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<Case2WgslMaterial>>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Render the comparison cube with the Case 2 WGSL material.
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        material: materials.add(Case2WgslMaterial {}),
        ..default()
    });

    // Benchmark helper entity.
    commands.spawn((FuseTime {
        timer: Timer::new(Duration::from_secs(1), TimerMode::Once),
    },));
}

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
struct Case2WgslMaterial {}

impl Material for Case2WgslMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/case2/case2-gradient-wgsl.wgsl".into()
    }
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
