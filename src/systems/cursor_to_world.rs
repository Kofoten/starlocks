use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Resource, Default)]
pub struct CursorCoords {
    pub global: Vec3
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct WorldPlane;

pub fn cursor_to_world(
    mut cursor_coords: ResMut<CursorCoords>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    plane_query: Query<&GlobalTransform, With<WorldPlane>>,
) {
    let (camera, camera_transform) = camera_query.single();
    let ground_transform = plane_query.single();
    let window = window_query.single();

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let plane_origin = ground_transform.translation();
    let plane = InfinitePlane3d::new(ground_transform.forward());

    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    let Some(distance) = ray.intersect_plane(plane_origin, plane) else {
        return;
    };

    let global_cursor = ray.get_point(distance);
    cursor_coords.global = global_cursor;
}