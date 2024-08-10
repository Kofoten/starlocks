mod systems;

use bevy::prelude::*;
use systems::cursor_to_world::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);

    app.insert_resource(Time::<Fixed>::from_hz(60.0));
    app.init_resource::<CursorCoords>();

    app.add_systems(Startup, setup);
    app.add_systems(Update, (cursor_to_world, mouse_input_system).chain());
    app.add_systems(FixedUpdate, (rotation_system, movement_system).chain());
    app.run();
}

#[derive(Component)]
struct PlayerEntity {
    movement_speed: f32,
    rotation_speed: f32,
    target: Vec2,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // circular base
    commands.spawn((
        WorldPlane,
        PbrBundle {
            mesh: meshes.add(Circle::new(4.0)),
            material: materials.add(Color::WHITE),  
            transform: Transform::default(),
            ..default()
        }
    ));

    // cube
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Capsule3d::default()),
            material: materials.add(Color::srgb_u8(124, 144, 255)),
            transform: Transform::from_xyz(1.0, 1.0, 0.0),
                //.with_rotation(Quat::from_rotation_x(f32::to_radians(90.))),
            ..default()
        },
        PlayerEntity {
            rotation_speed: f32::to_radians(10.),
            movement_speed: 1.,
            target: Vec2::new(0.0, 0.0),
        },
    ));

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // camera
    commands.spawn((
        MainCamera,
        Camera3dBundle {
            transform: Transform::from_xyz(0., 0., 9.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
    ));
}

fn rotation_system(time: Res<Time>, mut query: Query<(&PlayerEntity, &mut Transform)>) {
    let (player, mut transform) = query.single_mut();
    transform.rotate_z(player.rotation_speed * time.delta_seconds());
}

fn movement_system(time: Res<Time>, mut query: Query<(&PlayerEntity, &mut Transform)>) {
    let (player, mut transform) = query.single_mut();
    let current_position = Vec2::new(transform.translation.x, transform.translation.y);
    println!("curr: {}:{}", current_position.x, current_position.y);
    let direction = (player.target - current_position).normalize();
    println!("dir: {}:{}", direction.x, direction.y);
    let movement = direction * (player.movement_speed * time.delta_seconds());
    println!("{}:{}", movement.x, movement.y);
    transform.translation.x += movement.x;
    transform.translation.y += movement.y;
}

fn mouse_input_system(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    cursor_coords: Res<CursorCoords>,
    mut query: Query<(&mut PlayerEntity, &Transform)>,
) {
    if mouse_button_input.just_pressed(MouseButton::Right) {
        let (mut player, _) = query.single_mut();
        player.target = Vec2::new(cursor_coords.global.x, cursor_coords.global.y);
    }
}