use bevy::{input::keyboard::KeyboardInput, prelude::*};

#[derive(Component)]
struct MyCameraMarker;

fn setup_camera(mut commands: Commands) {
    // camera
    commands.spawn((Camera2d, MyCameraMarker));
}

#[derive(Component)]
struct SnakeHead;

const SNAKE_HEAD_COLOR: Color = Color::srgb(0.7, 0.7, 0.7);

fn spawn_snake(mut commands: Commands) {
    commands.spawn((
        Sprite {
            color: SNAKE_HEAD_COLOR,
            custom_size: Some(Vec2::new(50., 50.)),

            ..default()
        },
        SnakeHead,
    ));
}

fn snake_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut head_positions: Query<&mut Transform, With<SnakeHead>>,
) {
    for mut transform in head_positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            transform.translation.y += 2.;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            transform.translation.y -= 2.;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            transform.translation.x += 2.;
        }
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            transform.translation.x -= 2.;
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, spawn_snake)
        .add_systems(Update, snake_movement)
        .run();
}
