use bevy::app::App;
use bevy::asset::RenderAssetUsages;
use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::render::mesh::PrimitiveTopology;
use bevy::DefaultPlugins;
use std::f32::consts::TAU;

#[derive(Component)]
struct Rotatable {
    speed: f32,
}
fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        OrbitCamera {
            target: Vec3::new(2.0, 1.0, 0.0),
            ..Default::default()
        },
        Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        GlobalTransform::default(),
    ));
}
fn spawn_light(mut commands: Commands) {
    commands.spawn((PointLight::default(), Transform::from_xyz(4.0, 8.0, 4.0)));
}

fn spawn_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(Cuboid::default()))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: bevy::color::Color::hsl(323.0, 1.0, 0.45),
            ..Default::default()
        })),
        Rotatable { speed: 0.2 },
    ));
}
fn spawn_axes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let extend = 3000.0;
    let vertices: Vec<[f32; 3]> = vec![
        [-extend, 0.0, 0.0],
        [extend, 0.0, 0.0],
        [0.0, -extend, 0.0],
        [0.0, extend, 0.0],
        [0.0, 0.0, -extend],
        [0.0, 0.0, extend],
    ];

    let colors: Vec<[f32; 4]> = vec![
        [1.0, 0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0, 1.0],
        [0.0, 1.0, 0.0, 1.0],
        [0.0, 1.0, 0.0, 1.0],
        [0.0, 0.0, 1.0, 1.0],
        [0.0, 0.0, 1.0, 1.0],
    ];

    let mut mesh = Mesh::new(PrimitiveTopology::LineList, RenderAssetUsages::default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);

    let custom_material = StandardMaterial {
        unlit: true,
        ..Default::default()
    };

    commands.spawn((
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(materials.add(custom_material)),
    ));
}
fn camera_zoom_system(
    mut scroll_events: EventReader<MouseWheel>,
    mut query: Query<&mut Projection, With<Camera3d>>,
) {
    let mut scroll_amount = 0.0;
    for ev in scroll_events.read() {
        scroll_amount += ev.y;
    }
    if scroll_amount.abs() > 0.0 {
        if let Ok(mut projection) = query.get_single_mut() {
            if let Projection::Perspective(ref mut perspective) = *projection {
                perspective.fov -= scroll_amount * 0.05;
                perspective.fov = perspective.fov.clamp(0.1, std::f32::consts::PI / 2.0);
            }
        }
    }
}
#[derive(Component)]
struct OrbitCamera {
    target: Vec3,
    distance: f32,
    yaw: f32,
    pitch: f32,
}

impl Default for OrbitCamera {
    fn default() -> Self {
        Self {
            target: Vec3::ZERO,
            distance: 10.0,
            yaw: 0.0,
            pitch: 0.0,
        }
    }
}

fn spawn_plane(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let material = StandardMaterial {
        base_color: Color::hsla(194.0, 100.0, 38.6, 0.2),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..Default::default()
    };
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0).subdivisions(10))),
        MeshMaterial3d(materials.add(material)),
    ));
}

fn orbit_camera_system(
    _time: Res<Time>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    mut query: Query<(&mut OrbitCamera, &mut Transform)>,
) {
    for (mut orbit, mut transform) in query.iter_mut() {
        if mouse_button_input.pressed(MouseButton::Left) {
            let delta = accumulated_mouse_motion.delta;
            orbit.yaw -= delta.x * 0.005;
            orbit.pitch -= delta.y * 0.005;
            orbit.pitch = orbit.pitch.clamp(-1.5, 1.5);
        }
        let rotation =
            Quat::from_axis_angle(Vec3::Y, orbit.yaw) * Quat::from_axis_angle(Vec3::X, orbit.pitch);
        transform.translation = orbit.target + rotation * Vec3::new(0.0, 0.0, orbit.distance);
        transform.look_at(orbit.target, Vec3::Y);
    }
}

fn rotate_cube(mut cubes: Query<(&mut Transform, &Rotatable)>, timer: Res<Time>) {
    for (mut transform, cube) in &mut cubes {
        transform.rotate_y(cube.speed * TAU * timer.delta_secs());
        transform.rotate_x(cube.speed * TAU * timer.delta_secs());
        transform.rotate_z(cube.speed * TAU * timer.delta_secs());
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, spawn_light)
        .add_systems(Startup, spawn_cube)
        .add_systems(Startup, spawn_plane)
        .add_systems(Startup, spawn_axes)
        .add_systems(Update, camera_zoom_system)
        .add_systems(Update, orbit_camera_system)
        .add_systems(Update, rotate_cube)
        .run();
}
