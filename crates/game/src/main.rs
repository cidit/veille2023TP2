use bevy::prelude::*;

fn main() {
    App::new().add_plugins((DefaultPlugins, HelloPlugin)).run();
}

#[derive(Component)]
struct Player;

pub struct HelloPlugin;
impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(FixedTime::new_from_secs(1.0/60.0)) // 1 60th of a second, for 60FPS
            .add_systems(Startup, setup)
            .add_systems(FixedUpdate, move_player)
        ;
    }
}

fn setup(
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((Player, SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.1, 0.1, 1.0),
            custom_size: Some(Vec2::new(20.0, 20.0)),
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    }));
}

fn move_player(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time_step: Res<FixedTime>
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
        Vec3::new(x, y, 0.0)
    };
    const PLAYER_SPEED: f32 = 35.0;

    player_transform.translation += movement * PLAYER_SPEED*time_step.period.as_secs_f32();
}


