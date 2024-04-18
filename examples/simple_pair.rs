use bevy::prelude::*;
use bevy_wormholes::*;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, WormholesPlugin));

    app.add_systems(Startup, spawn_props);

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