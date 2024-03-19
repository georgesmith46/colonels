use crate::GameState;
use bevy::prelude::*;

pub struct WorldPlugin;

#[derive(Component)]
pub struct Block;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_world);
    }
}

fn spawn_world(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.5, 0.5, 1.0),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(-150.0, -150.0, 0.0),
                scale: Vec3::new(100., 100., 1.0),
                ..default()
            },
            ..default()
        })
        .insert(Block);
}
