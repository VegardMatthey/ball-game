use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

const TIME_STEP: f32 = 1.0 / 60.0;
const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, -50.0, 1.0);
const BALL_SIZE: Vec3 = Vec3::new(30.0, 30.0, 0.0);
const BALL_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);
const BALL_SPEED: f32 = 16.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup)
        .add_system(velocity)
        .add_system(move_ball)
        .add_system(bevy::window::close_on_esc)
        .run();
}

#[derive(Component)]
struct Ball;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::default().into()).into(),
            material: materials.add(ColorMaterial::from(BALL_COLOR)),
            transform: Transform::from_translation(BALL_STARTING_POSITION).with_scale(BALL_SIZE),
            ..default()
        },
        Ball,
    ));
}

fn velocity(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * TIME_STEP;
        transform.translation.y += velocity.y * TIME_STEP;
    }
}

fn move_ball(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<Ball>>) {
    let mut ball_transform = query.single_mut();
    let mut x: f32 = 0.;
    let mut y: f32 = 0.;

    if keyboard_input.pressed(KeyCode::D) {
        x += 1.0;
    }

    if keyboard_input.pressed(KeyCode::A) {
        x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::W) {
        y += 1.0;
    }

    if keyboard_input.pressed(KeyCode::S) {
        y -= 1.0;
    }

    ball_transform.translation = Vec3::new(
        ball_transform.translation.x + x * BALL_SPEED,
        ball_transform.translation.y + y * BALL_SPEED,
        1.0,
    );
}
