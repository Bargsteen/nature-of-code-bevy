use bevy::prelude::*;
use rand::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(startup_system)
        .add_system(walk)
        .run();
}

#[derive(Component)]
struct Walker;

fn startup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("black-circle.png"),
        ..Default::default()
    }).insert(Walker);
}

fn walk(mut query: Query<&mut Transform, With<Walker>>) {
    let speed = 5.;
    let mut rng = rand::thread_rng();
    let mut walker_transform = query.single_mut();
    walker_transform.translation.x += rng.gen_range(-speed..speed);
    walker_transform.translation.y += rng.gen_range(-speed..speed);
}
