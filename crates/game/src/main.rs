use bevy::prelude::*;

fn main() {
    App::new().add_plugins((DefaultPlugins, HelloPlugin)).run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Resource)]
struct GreetTimer(Timer);

pub struct HelloPlugin;
impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .insert_resource(FixedTime::new_from_secs(1.0/60.0)) // 1 60th of a second, for 60FPS
            .add_systems(Startup, setup)
            .add_systems(Startup, add_people)
            .add_systems(Update, greet_people)
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
        transform: Transform::from_translation(Vec3::new(0.0,0.0,0.0)),
        ..default()
    }));
}

fn move_player(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time_step: Res<FixedTime>
) {
    let mut player_transform = query.single_mut();
    let direction = {
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
        (x, y)
    };
    const PLAYER_SPEED: f32 = 35.0;

    let newpos = Vec3::new(
        player_transform.translation.x + direction.0*PLAYER_SPEED*time_step.period.as_secs_f32(),
        player_transform.translation.y + direction.1*PLAYER_SPEED*time_step.period.as_secs_f32(),
        0.0
    );
    player_transform.translation = newpos;
}

fn add_people(mut commands: Commands) {
    commands.spawn_batch([
        (Person, Name("Henry".to_string())),
        (Person, Name("Henzo".to_string())),
        (Person, Name("Steve".to_string())),
    ])
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }
    for name in &query {
        println!("hello, {name}!");
    }
}
