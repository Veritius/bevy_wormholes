use bevy::{input::mouse::MouseMotion, prelude::*, window::{CursorGrabMode, PrimaryWindow}};
use bevy_wormholes::*;

const MOUSE_SENSITIVITY: Vec2 = Vec2::new(0.001, 0.001);
const CAMERA_MOVE_SPEED: f32 = 1.0;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, WormholesPlugin));

    app.add_systems(Startup, (spawn_props, setup_camera));
    app.add_systems(Update, flycam_system);

    app.run();
}

fn spawn_props(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn a light
    commands.spawn(PointLightBundle {
        point_light: PointLight { range: 100.0, ..default() },
        transform: Transform::from_xyz(0.0, 10.0, 0.0),
        ..default()
    });

    // Spawn a plane as the floor
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::new(Vec3::Y).mesh().size(100.0, 100.0)),
        material: materials.add(StandardMaterial { base_color: Color::GREEN, ..default() }),
        ..default()
    });

    // Shared cube assets
    let cube_mesh = meshes.add(Cuboid::new(2.0, 2.0, 2.0));
    let cube_mat = materials.add(StandardMaterial { base_color: Color::WHITE, ..default() });

    // Spawn a cube
    commands.spawn(PbrBundle {
        mesh: cube_mesh,
        material: cube_mat,
        transform: Transform::from_xyz(6.0, 1.0, 2.0),
        ..default()
    });
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
    let up = *transform.up();
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