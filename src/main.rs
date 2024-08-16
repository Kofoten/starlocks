mod systems;

use bevy::prelude::*;
use systems::cursor_to_world::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);

    app.insert_resource(Time::<Fixed>::from_hz(60.0));
    app.init_resource::<CursorCoords>();

    app.add_systems(Startup, setup);
    app.add_systems(Update, (cursor_to_world, mouse_input_system, debug_text_system).chain());
    app.add_systems(FixedUpdate, (rotation_system, movement_system).chain());
    app.run();
}

#[derive(Component)]
struct PlayerEntity {
    movement_speed: f32,
    rotation_speed: f32,
    target: Vec2,
    facing: f32,
}

#[derive(Component)]
struct DebugText;

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
            rotation_speed: f32::to_radians(45.),
            movement_speed: 3.,
            target: Vec2::new(0.0, 0.0),
            facing: 0.0,
        },
    ));

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 8.0),
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



    // debug text
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "facing: \n",
                TextStyle {
                    font_size: 12.0,
                    ..default()
                },
            ),
            TextSection::new(
                "rotation: \n",
                TextStyle {
                    font_size: 12.0,
                    ..default()
                },
            ),
            TextSection::new(
                "target: \n",
                TextStyle {
                    font_size: 12.0,
                    ..default()
                },
            ),
            TextSection::new(
                "position: \n",
                TextStyle {
                    font_size: 12.0,
                    ..default()
                },
            ),
        ])
        .with_text_justify(JustifyText::Left)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        }),
        DebugText,
    ));
}

fn rotation_system(time: Res<Time>, mut query: Query<(&PlayerEntity, &mut Transform)>) {
    let (player, mut transform) = query.single_mut();
    let remainder = transform.rotation.z - player.facing;

    if remainder == 0.0 {
        return;
    }

    let facing = player.facing;
    println!("{facing}");
    let rotation = transform.rotation.z;
    println!("{rotation}");
    let remainder_abs = remainder.abs();
    println!("{remainder_abs}");
    let rotation_add = player.rotation_speed * time.delta_seconds();
    println!("{rotation_add}");
    if rotation_add > remainder_abs {
        transform.rotate_local_z(remainder);
        return;
    }
    
    let mut normalized = 1.0;
    if remainder < 0.0 {
        normalized = -1.0;
    }

    transform.rotate_local_z(rotation_add * normalized);
}

fn movement_system(time: Res<Time>, mut query: Query<(&PlayerEntity, &mut Transform)>) {
    let (player, mut transform) = query.single_mut();
    let current_position = Vec2::new(transform.translation.x, transform.translation.y);
    let remainder = player.target - current_position;

    if remainder.length() == 0.0 {
        return;
    }

    let direction = remainder.normalize();
    let movement = direction * (player.movement_speed * time.delta_seconds());
    let abs_rem = remainder.abs();
    let abs_move = movement.abs();

    if abs_rem.x.abs() < abs_move.x.abs() {
        transform.translation.x += remainder.x
    } else {
        transform.translation.x += movement.x;
    }

    if abs_rem.y.abs() < abs_move.y.abs() {
        transform.translation.y += remainder.y
    } else {
        transform.translation.y += movement.y;
    }
}

fn mouse_input_system(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    cursor_coords: Res<CursorCoords>,
    mut query: Query<(&mut PlayerEntity, &Transform)>,
) {
    if mouse_button_input.just_pressed(MouseButton::Right) {
        let (mut player, transform) = query.single_mut();
        player.target = Vec2::new(cursor_coords.global.x, cursor_coords.global.y);
        let direction = transform.translation - cursor_coords.global;
        player.facing = direction.y.atan2(direction.x);
    }
}

fn debug_text_system(
    player_query: Query<(&PlayerEntity, &Transform)>,
    mut query: Query<&mut Text, With<DebugText>>
) {
    let mut text = query.single_mut();
    let (player, transform) = player_query.single();

    let facing = player.facing;
    let rotation = transform.rotation.z;
    let target_x = player.target.x;
    let target_y = player.target.y;
    let position_x = transform.translation.x;
    let position_y = transform.translation.y;

    text.sections[0].value = format!("facing: {facing}\n");
    text.sections[1].value = format!("rotation: {rotation}\n");
    text.sections[2].value = format!("target: (x: {target_x}, y: {target_y})\n");
    text.sections[3].value = format!("position: (x: {position_x}, y: {position_y})\n");
}