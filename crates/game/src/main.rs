use std::{future, collections::HashMap};

use bevy::{prelude::*, window::PrimaryWindow, core_pipeline::clear_color::ClearColorConfig, asset::LoadAssets};
use bevy_rapier2d::prelude::*;
use bevy_rapier_collider_gen::{multi_polyline_collider_translated};

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


#[derive(Resource, Default)]
struct GameAssets {
    images: HashMap<String, Handle<Image>>
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct MainCamera;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AppStates {
    #[default]
    Loading, 
    Playing,
}

pub struct HelloPlugin;
impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppStates>()
            .insert_resource(FixedTime::new_from_secs(1.0 / 60.0)) // 1 60th of a second, for 60FPS
            .insert_resource(GameAssets::default())
            .add_systems(Startup, setup)
            .add_systems(FixedUpdate, move_player)
            .add_systems(Update, rotate_player_according_to_mouse)
            .add_systems(Update, camera_follow_player)
            .add_systems(Update, bevy::window::close_on_esc);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
   // mut materials: Res<Assets<ColorMaterial>>,
    images: Res<Assets<Image>>
    ) {

    commands.spawn((
        MainCamera,
        Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(Color::hex("5542FD").expect("failed to parse color")),
            },
            ..Default::default()
        },
    ));
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
            sprite: Sprite {
                flip_x: true,
                flip_y: false,
                ..Default::default()
            },

            transform: Transform::from_xyz(0., 0., 0.).with_scale(Vec3::splat(3.0)),
            ..default()
        },
        Sleeping::disabled(),
        GravityScale(0.1),
        Ccd::enabled(),
        ExternalImpulse {
            impulse: Vec2::new(0., 0.),
            ..Default::default()
        },
    ));
    {
        
        let image_handle: Handle<Image> = asset_server.load("sable.png");
        println!("{:?}",asset_server.get_load_state(image_handle.clone()));
        // let mut material = ColorMaterial::from(image.clone());
        let image = images.get(&image_handle).expect("failed to get image");
        let colliders = multi_polyline_collider_translated(&image); 

        commands.spawn((
            RigidBody::Fixed,
            Ccd::enabled(),
            SpriteBundle{
                texture: image_handle,
                ..Default::default()
            }            
        ));
    }
    
}

fn loadAssets(
    asset_server: Res<AssetServer>,
    mut game_assets: ResMut<GameAssets>, 
) {
    let image_handle: Handle<Image> = asset_server.load("sable.png");
    game_assets.images.insert("sable".to_string(), image_handle);

    game_assets.images = HashMap::from(
        vec![
            "sable.png",
            "character.png"
        ].iter()
        .map(|&s| (s.into(), asset_server.load(s)))
        .collect::<Vec<_>>()
    );
        

}

fn check_assets(
    asset_server: Res<AssetServer>,
    mut state: ResMut<NextState<AppStates>>,
) {

    if asset_server.get_load_state(image_handle.clone()) == LoadState::Loaded {
        state.set(AppStates::Playing).unwrap();
    }
}

fn move_player(
    keyboard: Res<Input<KeyCode>>,
    mut impulses: Query<&mut ExternalImpulse, With<Player>>,
    time_step: Res<FixedTime>,
) {
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

    for mut impulse in impulses.iter_mut() {
        impulse.impulse = movement * PLAYER_SPEED * time_step.period.as_secs_f32();
    }
}

fn rotate_player_according_to_mouse(
    player_transform: Query<&Transform, With<Player>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut sprite: Query<&mut Sprite, With<Player>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = q_camera.single();

    if let Some(position) = windows
        .single()
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        if player_transform.single().translation.x < position.x {
            sprite.single_mut().flip_x = false;
        } else {
            sprite.single_mut().flip_x = true;
        }
    }
}

fn camera_follow_player(
    player_transform: Query<&Transform, With<Player>>,
    mut camera_transform: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
) {
    let player_transform = player_transform.single();
    let mut camera_transform = camera_transform.single_mut();
    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}

