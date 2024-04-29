use std::f32::consts::FRAC_PI_2;
use bevy::{input::mouse::MouseMotion, pbr::{light_consts::lux::OVERCAST_DAY, CascadeShadowConfigBuilder}, prelude::*, window::{CursorGrabMode, PrimaryWindow}};
use bevy_wormholes::*;

const MOUSE_SENSITIVITY: Vec2 = Vec2::new(0.001, 0.001);
const CAMERA_MOVE_SPEED: f32 = 1.0;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, WormholesPlugin));

    app.add_systems(Startup, (spawn_wormholes, spawn_props, setup_camera));
    app.add_systems(Update, (flycam_system, camera_gizmos));

    app.run();
}

fn spawn_wormholes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut shaders: ResMut<Assets<WormholeShader>>,
) {
    let mut builder = WormholeBuilder::new();

    builder.both(&|c| {
        c.with_dimensions(Vec2::new(2.0, 2.0));
    });

    builder.orange(|c| {
        let transform = Transform::from_translation(Vec3::new(-3.5, 1.0, 0.0))
            .with_rotation(Quat::from_axis_angle(Vec3::X, -FRAC_PI_2));

        c.with_transform(transform);
    });

    builder.blue(|c| {
        let transform = Transform::from_translation(Vec3::new(3.5, 1.0, 0.0))
            .with_rotation(Quat::from_axis_angle(Vec3::X, -FRAC_PI_2));

        c.with_transform(transform);
    });

    let context = WormholeBuilderContext {
        commands: &mut commands,
        meshes: &mut meshes,
        images: &mut images,
        shaders: &mut shaders,
    };

    builder.build(context).unwrap();
}


fn spawn_props(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn a directional light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(2.0, 5.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 100.0,
            ..default()
        }.into(),
        ..default()
    });

    // Spawn two planes
    let plane_mesh = meshes.add(Plane3d::new(Vec3::Y).mesh().size(5.0, 10.0));
    commands.spawn(PbrBundle {
        mesh: plane_mesh.clone(),
        material: materials.add(StandardMaterial { base_color: Color::BLUE, ..default() }),
        transform: Transform::from_xyz(3.5, 0.0, 0.0),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: plane_mesh.clone(),
        material: materials.add(StandardMaterial { base_color: Color::ORANGE, ..default() }),
        transform: Transform::from_xyz(-3.5, 0.0, 0.0),
        ..default()
    });

    // Identical cubes are spawned at all these positions.
    static CUBE_POSITIONS: &[[f32;2]] = &[
        [-5.0, -1.0],
        [-2.0, 3.0],
        [3.0, 4.0],
        [4.0, -2.0],
    ];

    // Shared cube assets
    let cube_mesh = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    let cube_mat = materials.add(StandardMaterial { base_color: Color::WHITE, ..default() });

    // Spawn some cubes
    for pos in CUBE_POSITIONS {
        commands.spawn(PbrBundle {
            mesh: cube_mesh.clone(),
            material: cube_mat.clone(),
            transform: Transform::from_translation(Vec3::new(pos[0], 0.5, pos[1])),
            ..default()
        });
    }
}

#[derive(Component)]
struct Flycam {
    lock: bool,
    rotation: Vec2,
}

impl Default for Flycam {
    fn default() -> Self {
        Self {
            lock: true,
            rotation: Vec2::ZERO,
        }
    }
}

fn setup_camera(
    mut commands: Commands,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    // Spawn camera
    commands.spawn((
        Flycam::default(),
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 3.0, 10.0),
            ..default()
        },
    ));

    // Lock cursor
    let mut window = windows.single_mut();
    window.cursor.grab_mode = CursorGrabMode::Locked;
    window.cursor.visible = false;
}

fn flycam_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut mouse: EventReader<MouseMotion>,
    mut camera: Query<(&mut Flycam, &mut Transform)>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    // Get the camera components
    let (mut camera, mut transform) = camera.single_mut();

    // Toggle locking
    if keyboard.just_pressed(KeyCode::Escape) {
        camera.lock = !camera.lock;

        let mut window = windows.single_mut();
        match camera.lock {
            true => {
                window.cursor.grab_mode = CursorGrabMode::Locked;
                window.cursor.visible = false;
            },
            false => {
                window.cursor.grab_mode = CursorGrabMode::None;
                window.cursor.visible = true;
            },
        }
    }

    // Only move if locked
    if !camera.lock { return }

    // Turning inputs
    for delta in mouse.read().map(|motion| motion.delta) {
        camera.rotation += delta * MOUSE_SENSITIVITY;
        let rot_yaw = Quat::from_axis_angle(Vec3::Y, -camera.rotation.x);
        let rot_pit = Quat::from_axis_angle(Vec3::X, -camera.rotation.y);
        transform.rotation = rot_yaw * rot_pit;
    }

    // Direction based on rotation
    let up = Vec3::Y;
    let fwd = *transform.forward();
    let rgt = *transform.right();

    // Keyboard inputs
    let mut movement = Vec3::ZERO;
    if keyboard.pressed(KeyCode::KeyW       ) { movement += fwd }
    if keyboard.pressed(KeyCode::KeyS       ) { movement -= fwd }
    if keyboard.pressed(KeyCode::KeyA       ) { movement -= rgt }
    if keyboard.pressed(KeyCode::KeyD       ) { movement += rgt }
    if keyboard.pressed(KeyCode::Space      ) { movement += up  }
    if keyboard.pressed(KeyCode::ControlLeft) |
        keyboard.pressed(KeyCode::ShiftLeft) { movement -= up  }

    // Apply movement
    transform.translation += movement * CAMERA_MOVE_SPEED;
}

fn camera_gizmos(
    mut gizmos: Gizmos,
    cameras: Query<&GlobalTransform, With<WormholeCamera>>,
) {
    for transform in cameras.iter() {
        gizmos.sphere(transform.translation(), Quat::IDENTITY, 0.2, Color::GOLD);
    }
}