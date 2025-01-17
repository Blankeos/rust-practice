use bevy::{
    input::keyboard::KeyboardInput, prelude::*, render::view::ExtractedWindows,
    time::common_conditions::on_real_timer, window::PrimaryWindow,
};
use rand::random;
use std::{slice::Windows, time::Duration};

const ARENA_WIDTH: u32 = 10;
const ARENA_HEIGHT: u32 = 10;
const SNAKE_HEAD_COLOR: Color = Color::srgb(0.7, 0.7, 0.7);
const FOOD_COLOR: Color = Color::srgb(1.0, 0.0, 1.0);
const SNAKE_SEGMENT_COLOR: Color = Color::srgb(0.3, 0.3, 0.3);

#[derive(Resource)]
struct GameTick(Timer);
fn game_tick_ticker(mut timer: ResMut<GameTick>, time: Res<Time>) {
    timer.0.tick(time.delta());
}

#[derive(PartialEq, Copy, Clone)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Size {
    width: f32,
    height: f32,
}
impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

#[derive(Component)]
struct MyCameraMarker;

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, MyCameraMarker));
}

#[derive(Component)]
struct SnakeHead {
    direction: Direction,
}

#[derive(Component)]
struct SnakeSegment;

#[derive(Default, Resource)]
struct SnakeSegments(Vec<Entity>);

fn spawn_snake(mut commands: Commands, mut segments: ResMut<SnakeSegments>) {
    *segments = SnakeSegments(vec![
        commands
            .spawn((Sprite {
                color: SNAKE_HEAD_COLOR,
                ..default()
            },))
            .insert(SnakeHead {
                direction: Direction::Up,
            })
            .insert(Position { x: 3, y: 3 })
            .insert(Size::square(0.8))
            .id(),
        spawn_segment(commands, Position { x: 3, y: 2 }),
    ])
}

fn spawn_segment(mut commands: Commands, position: Position) -> Entity {
    commands
        .spawn(Sprite {
            color: SNAKE_SEGMENT_COLOR,
            ..default()
        })
        .insert(SnakeSegment)
        .insert(position)
        .insert(Size::square(0.65))
        .id()
}

fn size_scaling(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut q: Query<(&Size, &mut Transform)>,
) {
    let window = window_query.single();

    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width / ARENA_WIDTH as f32 * window.width() as f32,
            sprite_size.height / ARENA_HEIGHT as f32 * window.height() as f32,
            1.0,
        );
    }
}

fn position_translation(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut q: Query<(&Position, &mut Transform)>,
) {
    let window = window_query.single();

    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }

    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32),
            convert(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32),
            0.0,
        );
    }
}

fn snake_movement(
    game_ticker: Res<GameTick>,
    snake_segments: ResMut<SnakeSegments>,
    mut heads: Query<(Entity, &SnakeHead)>,
    mut positions: Query<&mut Position>,
) {
    if !game_ticker.0.just_finished() {
        return;
    }

    if let Some((head_entity, head)) = heads.iter_mut().next() {
        // First, collect all current positions
        let segment_positions: Vec<Position> = snake_segments
            .0 // or .segments depending on your field name
            .iter()
            .map(|e| *positions.get(*e).unwrap())
            .collect();

        // Update head position
        let mut head_pos = positions.get_mut(head_entity).unwrap();
        match &head.direction {
            Direction::Left => {
                head_pos.x -= 1;
            }
            Direction::Right => {
                head_pos.x += 1;
            }
            Direction::Up => {
                head_pos.y += 1;
            }
            Direction::Down => {
                head_pos.y -= 1;
            }
        };

        // Update body segments
        for (segment, pos) in snake_segments
            .0
            .iter()
            .skip(1)
            .zip(segment_positions.iter())
        {
            if let Ok(mut segment_pos) = positions.get_mut(*segment) {
                *segment_pos = *pos;
            }
        }
    }
}

fn snake_movement_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut heads: Query<&mut SnakeHead>,
) {
    if let Some(mut head) = heads.iter_mut().next() {
        let dir: Direction = if keyboard_input.pressed(KeyCode::ArrowLeft) {
            Direction::Left
        } else if keyboard_input.pressed(KeyCode::ArrowDown) {
            Direction::Down
        } else if keyboard_input.pressed(KeyCode::ArrowUp) {
            Direction::Up
        } else if keyboard_input.pressed(KeyCode::ArrowRight) {
            Direction::Right
        } else {
            head.direction
        };
        if dir != head.direction.opposite() {
            head.direction = dir;
        }
    }
}

#[derive(Component)]
struct Food;

fn food_spawner(mut commands: Commands) {
    commands
        .spawn(Sprite {
            color: FOOD_COLOR,
            ..default()
        })
        .insert(Food)
        .insert(Position {
            x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
            y: (random::<f32>() * ARENA_HEIGHT as f32) as i32,
        })
        .insert(Size::square(0.8));
}

use bevy::time::common_conditions::on_timer;

fn main() {
    App::new()
        .insert_resource(GameTick(Timer::from_seconds(0.08, TimerMode::Repeating)))
        .insert_resource(SnakeSegments::default())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Snake!".to_string(),
                resolution: (500., 500.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(
            FixedUpdate,
            food_spawner.run_if(on_timer(Duration::from_millis(800))),
        )
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, spawn_snake)
        .add_systems(Update, snake_movement_input)
        .add_systems(FixedUpdate, (game_tick_ticker, snake_movement))
        .add_systems(PostUpdate, (position_translation, size_scaling))
        .run();
}
