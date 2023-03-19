use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    sprite::MaterialMesh2dBundle,
};

const TIME_STEP: f32 = 1.0 / 60.0;
const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, -50.0, 1.0);
const BALL_SIZE: Vec3 = Vec3::new(30.0, 30.0, 0.0);
const BALL_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const BALL_SPEED: f32 = 16.0;

const BRICK_SIZE: Vec2 = Vec2::new(100., 30.);
const BRICK_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const BRICK_SPEED: f32 = 512.0;

const WALL_THICKNESS: f32 = 10.0;
const WALL_COLOR: Color = Color::rgb(0.2, 0.2, 0.2);
const LEFT_WALL: f32 = -450.; // x coordinates
const RIGHT_WALL: f32 = 450.;
const BOTTOM_WALL: f32 = -300.; // y coordinates
const TOP_WALL: f32 = 300.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup)
        .add_event::<CollisionEvent>()
        .add_systems(
            (
                collision,
                velocity.before(collision),
                move_ball.before(collision).after(velocity),
            )
                .in_schedule(CoreSchedule::FixedUpdate),
        )
        .insert_resource(FixedTime::new_from_secs(TIME_STEP))
        .add_system(bevy::window::close_on_esc)
        .run();
}

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Brick;

#[derive(Component)]
struct Health {
    points: u16,
}

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Collider;

#[derive(Default)]
struct CollisionEvent;

#[derive(Bundle)]
struct WallBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

impl WallLocation {
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(LEFT_WALL, 0.),
            WallLocation::Right => Vec2::new(RIGHT_WALL, 0.),
            WallLocation::Bottom => Vec2::new(0., BOTTOM_WALL),
            WallLocation::Top => Vec2::new(0., TOP_WALL),
        }
    }

    fn size(&self) -> Vec2 {
        let arena_height = TOP_WALL - BOTTOM_WALL;
        let arena_width = RIGHT_WALL - LEFT_WALL;
        // Make sure we haven't messed up our constants
        assert!(arena_height > 0.0);
        assert!(arena_width > 0.0);

        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS)
            }
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS)
            }
        }
    }
}

impl WallBundle {
    fn new(location: WallLocation) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: location.position().extend(0.0),
                    scale: location.size().extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                ..default()
            },
            collider: Collider,
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));
    commands.spawn(WallBundle::new(WallLocation::Bottom));
    commands.spawn(WallBundle::new(WallLocation::Top));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::default().into()).into(),
            material: materials.add(ColorMaterial::from(BALL_COLOR)),
            transform: Transform::from_translation(BALL_STARTING_POSITION).with_scale(BALL_SIZE),
            ..default()
        },
        Ball,
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: BRICK_COLOR,
                ..default()
            },
            transform: Transform {
                translation: Vec2::new(100.0, 100.0).extend(0.0),
                scale: Vec3::new(BRICK_SIZE.x, BRICK_SIZE.y, 1.0),
                ..default()
            },
            ..default()
        },
        Brick,
        Collider,
        Velocity(Vec2::new(1.0, 1.0).normalize() * BRICK_SPEED),
        //Health { points: 3 },
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

    let left_bound = LEFT_WALL + WALL_THICKNESS / 2.0 + BALL_SIZE.x / 2.0;
    let right_bound = RIGHT_WALL - WALL_THICKNESS / 2.0 - BALL_SIZE.x / 2.0;
    let bottom_bound = BOTTOM_WALL + WALL_THICKNESS / 2.0 + BALL_SIZE.x / 2.0;
    let top_bound = TOP_WALL - WALL_THICKNESS / 2.0 - BALL_SIZE.x / 2.0;

    ball_transform.translation = Vec3::new(
        (ball_transform.translation.x + x * BALL_SPEED).clamp(left_bound, right_bound),
        (ball_transform.translation.y + y * BALL_SPEED).clamp(bottom_bound, top_bound),
        1.0,
    );
}

fn collision(
    mut brick_query: Query<(&mut Velocity, &Transform), With<Brick>>,
    collider_query: Query<&Transform, With<Collider>>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let (mut brick_velocity, brick_transform) = brick_query.single_mut();
    let brick_size = brick_transform.scale.truncate();

    for transform in &collider_query {
        let collision = collide(
            brick_transform.translation,
            brick_size,
            transform.translation,
            transform.scale.truncate(),
        );
        if let Some(collision) = collision {
            //brick_health.points -= 1;
            collision_events.send_default();

            let mut reflect_x = false;
            let mut reflect_y = false;

            match collision {
                Collision::Left => reflect_x = brick_velocity.x > 0.0,
                Collision::Right => reflect_x = brick_velocity.x < 0.0,
                Collision::Top => reflect_y = brick_velocity.y < 0.0,
                Collision::Bottom => reflect_y = brick_velocity.y > 0.0,
                Collision::Inside => { /* do nothing */ }
            }

            if reflect_x {
                brick_velocity.x = -brick_velocity.x;
            }
            if reflect_y {
                brick_velocity.y = -brick_velocity.y;
            }
        }
    }
}
