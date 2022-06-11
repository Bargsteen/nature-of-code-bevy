use bevy::prelude::*;
use rand::prelude::*;

const GAUSSIAN_WALK: bool = true;

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
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("black-circle.png"),
            ..Default::default()
        })
        .insert(Walker);
}

fn walk(mut query: Query<&mut Transform, With<Walker>>) {
    let speed = montecarlo_step() * 20.;
    let mut walker_transform = query.single_mut();
    let mut rng = rand::thread_rng();
    let dx: f32;
    let dy: f32;
    if GAUSSIAN_WALK {
        dx = rng.gen::<f32>() * speed - (speed / 2.);
        dy = rng.gen::<f32>() * speed - (speed / 2.);
    } else {
        dx = rng.gen_range(-speed..speed);
        dy = rng.gen_range(-speed..speed);
    }
    walker_transform.translation.x += dx;
    walker_transform.translation.y += dy;
}

fn montecarlo_step() -> f32 {
    let mut rng = rand::thread_rng();
    loop {
        let r1: f32 = rng.gen();
        let probability: f32 = r1.sqrt();
        let r2: f32 = rng.gen();
        if r2 < probability {
            return r1;
        }
    }
}
