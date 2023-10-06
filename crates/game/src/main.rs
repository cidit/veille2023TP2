use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins((
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            RapierDebugRenderPlugin::default(),
        ))
        .add_plugins(HelloPlugin)
        .run();
}

#[derive(Component)]
struct Player;

pub struct HelloPlugin;
impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FixedTime::new_from_secs(1.0 / 60.0)) // 1 60th of a second, for 60FPS
            .add_systems(Startup, setup)
            .add_systems(FixedUpdate, move_player)
            .add_systems(Update, bevy::window::close_on_esc);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let camera = Camera2dBundle::default();
    camera.frustum
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        Player,
        RigidBody::Dynamic,
        // Collider::ball(50.),
        AdditionalMassProperties::Mass(6.),
        Velocity {
            linvel: Vec2::new(0., 0.),
            angvel: 0.,
        },
        Restitution::coefficient(0.7),
        SpriteBundle {
            texture: asset_server.load("character.png"),
            // sprite: Sprite {
            //     color: Color::rgb(0.1, 0.1, 1.0),
            //     custom_size: Some(Vec2::new(20.0, 20.0)),
            //     ..Default::default()
            // },
            transform: Transform::from_xyz(0., 0., 0.).with_scale(Vec3::splat(3.0)),
            ..default()
        },
        Sleeping::disabled(),
        GravityScale(0.1),
        Ccd::enabled(),
        ExternalImpulse {
            impulse: Vec2::new(0., 0. ),
            ..Default::default()
        },
    ));
}

fn move_player(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    mut impulses: Query<&mut ExternalImpulse, With<Player>>,
    time_step: Res<FixedTime>,
) {
    let mut player_transform = query.single_mut();
    let movement = {
        let (mut x, mut y) = (0.0, 0.0);
        if keyboard.pressed(KeyCode::Right) {
            x += 1.0;
        }
        if keyboard.pressed(KeyCode::Left) {
            x -= 1.0;
        }
        if keyboard.pressed(KeyCode::Up) {
            y += 1.0;
        }
        if keyboard.pressed(KeyCode::Down) {
            y -= 1.0;
        }
        Vec2::new(x, y)
    };
    const PLAYER_SPEED: f32 = 300.0;

    // player_transform.translation += movement * PLAYER_SPEED * time_step.period.as_secs_f32();
    for mut impulse in impulses.iter_mut() {
        impulse.impulse = movement * PLAYER_SPEED * time_step.period.as_secs_f32();    
    }
    
}
