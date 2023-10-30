use std::{collections::HashMap, ops::Neg, f32::consts::PI, rc::Rc};

use bevy::{
    asset::LoadState, core_pipeline::clear_color::ClearColorConfig, prelude::*,
    window::PrimaryWindow,
};
use bevy_rapier2d::prelude::*;
use bevy_rapier_collider_gen::single_heightfield_collider_translated;

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
    images: HashMap<String, Handle<Image>>,
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Spear;


#[derive(Component)]
struct Terrain;

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
            .add_systems(OnEnter(AppStates::Loading), load_assets)
            .add_systems(OnExit(AppStates::Loading), setup)
            .add_systems(
                FixedUpdate,
                move_player.run_if(in_state(AppStates::Playing)),
            )
            .add_systems(
                Update,
                (
                    rotate_spear_according_to_mouse.run_if(in_state(AppStates::Playing)),
                    rotate_player_according_to_mouse.run_if(in_state(AppStates::Playing)),
                    camera_follow_player.run_if(in_state(AppStates::Playing)),
                    check_assets.run_if(in_state(AppStates::Loading)),
                ),
            )
            .add_systems(Update, bevy::window::close_on_esc);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // mut materials: Res<Assets<ColorMaterial>>,
    images: Res<Assets<Image>>,
) {
    commands.spawn((
        MainCamera,
        Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(
                    Color::hex("5542FD").expect("failed to parse color"),
                ),
            },
            ..Default::default()
        },
    ));
    commands.spawn((
        Player,
        RigidBody::Dynamic,
        Collider::cuboid(10., 20.),
        AdditionalMassProperties::Mass(6.),
        Velocity {
            linvel: Vec2::new(0., 0.),
            angvel: 0.,
        },
        Restitution::coefficient(0.3f32),
        Damping {
            linear_damping: 0.5,
            angular_damping: 1.0,
        },
        LockedAxes::ROTATION_LOCKED,
        SpriteBundle {
            texture: asset_server.load("character.png"),
            sprite: Sprite {
                flip_x: true,
                flip_y: false,
                ..Default::default()
            },

            transform: Transform::from_xyz(0., -10., 0.).with_scale(Vec3::splat(3.0)),

            ..default()
        },
        Sleeping::disabled(),
        GravityScale(0.1),
        Ccd::enabled(),
        ExternalImpulse {
            impulse: Vec2::new(0., 0.),
            ..Default::default()
        },    
    )).with_children(
        | player | {
            player.spawn((
                Spear,
                SpriteBundle {
                    texture: asset_server.load("spear.png"),
                    sprite: Sprite {
                        flip_x: true,
                        flip_y: false,
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(0., 0., 0.).with_scale(Vec3::splat(0.08)),
                    ..default()
                },
                Collider::cuboid(10., 20.),
                Ccd::enabled(),
            ));
        }
    );
    {
        let image_handle: Handle<Image> = asset_server.load("terrain.png");
        println!("{:?}", asset_server.get_load_state(image_handle.clone()));
        // let mut material = ColorMaterial::from(image.clone());
        let image = images.get(&image_handle).expect("failed to get image");
        let collider = single_heightfield_collider_translated(&image);

        commands.spawn((
            Terrain,
            collider.clone(),
            RigidBody::Fixed,
            Ccd::enabled(),
            SpriteBundle {
                texture: image_handle.clone(),
                transform: Transform::from_xyz(0f32, 0f32, 0f32).with_scale(Vec3::splat(5.0)),
                ..Default::default()
            },
        ));
        let width = image.size().x;
        commands.spawn((
            Terrain,
            collider.clone(),
            RigidBody::Fixed,
            Ccd::enabled(),
            SpriteBundle {
                texture: image_handle.clone(),
                transform: Transform::from_xyz(width * 5f32, 0f32, 0f32).with_scale(Vec3::splat(5.0)),
                ..Default::default()
            },
        ));
    }


}

fn load_assets(asset_server: Res<AssetServer>, mut game_assets: ResMut<GameAssets>) {
    game_assets.images = HashMap::from([
        ("terrain".to_string(), asset_server.load("terrain.png")),
        ("Character".to_string(), asset_server.load("character.png")),
        ("trash".to_string(), asset_server.load("trash.png")),
        ("spear".to_string(), asset_server.load("spear.png")),
    ]);
}

fn check_assets(
    asset_server: Res<AssetServer>,
    mut state: ResMut<NextState<AppStates>>,
    game_assets: ResMut<GameAssets>,
) {
    for handle in game_assets.images.values() {
        if asset_server.get_load_state(handle.clone()) != LoadState::Loaded {
            return;
        }
    }

    state.set(AppStates::Playing);
}

fn move_player(
    keyboard: Res<Input<KeyCode>>,
    mut impulses: Query<&mut ExternalImpulse, With<Player>>,
    time_step: Res<FixedTime>,
) {
    //impulses.single_mut().impulse = impulses.single_mut().impulse / 10f32;

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
    const PLAYER_SPEED: f32 = 1000.0;

    impulses.single_mut().impulse = movement * PLAYER_SPEED * time_step.period.as_secs_f32();
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


fn rotate_spear_according_to_mouse(
    mut spear_transform: Query<& mut Transform, With<Spear>>,
    spear_global_transform: Query<&GlobalTransform, With<Spear>>,
    windows: Query<&Window, With<PrimaryWindow>>,   
) {
    let Some(cursor_pos) = windows.single().cursor_position() else {
        return;
    };
    let mut spear_transform = spear_transform.single_mut();
    let sgt = spear_global_transform.single().compute_transform();

    let local_cursor_pos = cursor_pos + sgt.translation.truncate();
    let angle_vec = sgt.translation.truncate() - local_cursor_pos;
    
    dbg!(angle_vec.angle_between(local_cursor_pos));
    // let quat_angle = (local_cursor_pos.y - spear_transform.translation.y).atan2(local_cursor_pos.x - spear_transform.translation.x);
    // dbg!( quat_angle);
    spear_transform.rotation = Quat::from_rotation_z(
        // (cursor_pos.y - sgt.translation.y).atan2(cursor_pos.x - sgt.translation.x) *  2
        angle_vec.angle_between(local_cursor_pos)
    );

    
}


fn animate_player_to_mouse_gizmo(
    mut gizmos: Gizmos,
    time_step: Res<FixedTime>,
    q_p_transfrom: Query<&Transform, With<Player>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_cameras: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (cam, cam_transform) = q_cameras.single();
    let Some(world_pos) = q_windows.single()
        .cursor_position()
        .and_then(|c| cam.viewport_to_world(cam_transform, c)) 
        .map(|r| r.origin.truncate())
    else {
        return;
    };
}