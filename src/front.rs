use bevy::{
    prelude::*,
    render::pass::ClearColor,
    sprite::collide_aabb::{collide, Collision},
};
use std::num;
use std::thread::spawn;

const ARENA_WIDTH: i32 = 10;
const ARENA_HEIGHT: i32 = 10;

struct Material {
    mat : Handle<ColorMaterial>,
}

#[derive(Default, Copy, Clone, PartialEq)]
struct Position {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
}

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

fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Sprite)>) {
    let window = windows.get_primary().unwrap();
    for (sprite_size, mut sprite) in q.iter_mut() {
        sprite.size = Vec2::new(
            sprite_size.width / ARENA_WIDTH as f32 * window.width(),
            sprite_size.height / ARENA_HEIGHT as f32 * window.height(),
        );
    }
}

fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }
    let window = windows.get_primary().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x, window.width(), ARENA_WIDTH as f32),
            convert(pos.y, window.height(), ARENA_HEIGHT as f32),
            0.0,
        );
    }
}

struct Ball;
fn spawn_ball(commands: &mut Commands, material: Res<Material>) {
    commands
        .spawn(SpriteBundle {
            material: material.mat.clone(),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .with(Ball)
        .with(Position {x: 5., y: 5., vx: 0.08, vy: 0.03})
        .with(Size::square(0.8));
}

fn ball_movement(mut position: Query<&mut Position, With<Ball>>) {
    for mut pos in position.iter_mut() {
        if pos.x >= ARENA_WIDTH as f32 - 1.0 || pos.x <= 0. {
            pos.vx = -pos.vx;
        }
        if pos.y >= ARENA_HEIGHT as f32 - 1.0 || pos.y <= 0. {
            pos.vy = -pos.vy;
        }
        
        pos.x += pos.vx;
        pos.y += pos.vy;
    }
}

fn setup(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands
        .spawn(Camera2dBundle::default())
        .insert_resource(Material {
            mat: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
        });
}


fn main() {
    App::build()
        .add_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_resource(WindowDescriptor {
            title: "Pong!".to_string(), 
            width: 500.0,                
            height: 500.0,                
            ..Default::default()        
        })
        .add_startup_system(setup.system())
        .add_startup_stage("game_setup", SystemStage::single(spawn_ball.system()))
        .add_system(position_translation.system())
        .add_system(size_scaling.system())
        .add_system(ball_movement.system())
        .add_plugins(DefaultPlugins)
        .run();
}