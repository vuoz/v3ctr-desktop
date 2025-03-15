use bevy::asset::RenderAssetUsages;
use bevy::input::mouse::MouseMotion;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::render::mesh::PrimitiveTopology;
use std::any::TypeId;

use components::*;
use std::f32::consts::TAU;

#[no_mangle]
pub fn setup_camera(commands: &mut Commands) {
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

#[no_mangle]
pub fn spawn_light(commands: &mut Commands) {
    print!("spawn_light");
    commands.spawn((PointLight::default(), Transform::from_xyz(4.0, 8.0, 4.0)));
}

#[no_mangle]
pub fn spawn_cube(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
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

#[no_mangle]
pub fn spawn_axes(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
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

#[no_mangle]
pub fn camera_zoom_system(
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

#[no_mangle]
pub fn spawn_plane(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
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

#[no_mangle]
pub fn orbit_camera_system(
    mouse_button_input: Res<ButtonInput<bevy::input::mouse::MouseButton>>,
    mut move_ev: EventReader<MouseMotion>,
    mut query: Query<(&mut OrbitCamera, &mut Transform)>,
) {
    if mouse_button_input.pressed(bevy::input::mouse::MouseButton::Left) {
        println!("left pressed");
    }
}

#[no_mangle]
pub fn rotate_cube(mut cubes: Query<(&mut Transform, &Rotatable)>, timer: Res<Time>) {
    for (mut transform, cube) in &mut cubes {
        transform.rotate_y(cube.speed * TAU * timer.delta_secs());
        transform.rotate_x(cube.speed * TAU * timer.delta_secs());
        transform.rotate_z(cube.speed * TAU * timer.delta_secs());
    }
}
#[no_mangle]
pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    setup_camera(&mut commands);
    spawn_light(&mut commands);
    spawn_cube(&mut commands, &mut meshes, &mut materials);
    spawn_axes(&mut commands, &mut meshes, &mut materials);
    spawn_plane(&mut commands, &mut meshes, &mut materials);
    println!("setup done");
    println!("from v3ctr: {:?}", TypeId::of::<MouseButton>());
}
