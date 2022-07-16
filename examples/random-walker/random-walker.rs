use bevy::prelude::*;
use noise::{NoiseFn, Perlin};
use rand::prelude::*;

const GAUSSIAN_WALK: bool = true;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(startup_system)
        // .add_system(walk)
        .add_system(perlin_walk)
        .run();
}

#[derive(Component)]
struct Walker;

#[derive(Component)]
struct PerlinNoise {
    x: f64,
    y: f64,
    noise: Perlin,
}

fn startup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("black-circle.png"),
            ..Default::default()
        })
        .insert(Walker)
        .insert(PerlinNoise {
            x: 0.,
            y: 10000.,
            noise: Perlin::new(),
        });
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

fn perlin_walk(mut query: Query<(&mut Transform, &mut PerlinNoise), With<Walker>>) {
    let (mut walker_transform, mut perlin) = query.single_mut();

    let val_x = perlin.noise.get([perlin.x, 0.]);
    let val_y = perlin.noise.get([0., perlin.y]);

    walker_transform.translation.x = map_range(val_x as f32, -1., 1., -500., 500.);
    walker_transform.translation.y = map_range(val_y as f32, -1., 1., -400., 400.);

    perlin.x += 0.01;
    perlin.y += 0.01;
}

/// Maps `input` from the input range to an output range.
fn map_range(
    input: f32,
    input_start: f32,
    input_end: f32,
    output_start: f32,
    output_end: f32,
) -> f32 {
    let slope = (output_end - output_start) / (input_end - input_start);
    let output = output_start + slope * (input - input_start);
    output
}
