use bevy::{
    prelude::*,
    render::pass::ClearColor,
    //sprite::collide_aabb::{collide, Collision},
};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_startup_system(setup.system())
        .add_system(player_action_system.system())
        .add_system(input_system.system())
        .add_system(cooldown_system.system())
        .add_system(bow_projectile_system.system())
        .add_event::<(PlayerAction, Vec3)>()
        .add_event::<BowEvent>()
        .run();
}

struct Player {
    speed: f32,
    health: usize,
    mana: usize,
    destination: Vec3,
    state: PlayerState,
    cooldowns: Cooldowns,
}

struct Cooldowns {
    // timer and cd time in seconds
    dash: Cooldown,
    bow: Cooldown,
}

struct Cooldown {
    timer: Timer,
    cooldown_millis: u64,
}

impl Player {
    fn can_move(&self) -> bool {
        self.state == PlayerState::Idle || self.state == PlayerState::Moving
    }
}

#[derive(PartialEq)]
enum PlayerState {
    Channeling(f32),
    Moving,
    Idle,
}

enum PlayerAction {
    Move,
    Dash,
    Bow,
}

struct MainCamera;

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    _asset_server: Res<AssetServer>,
) {
    // Add the game's entities to our world

    // cameras
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
    commands.spawn_bundle(UiCameraBundle::default());
    // player
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 0.5, 0.5).into()),
            transform: Transform::from_xyz(0.0, -50.0, 1.0),
            sprite: Sprite::new(Vec2::new(60.0, 60.0)),
            ..Default::default()
        })
        .insert(Player {
            speed: 300.0,
            destination: Vec3::new(0.0, 0.0, 1.0).normalize(),
            health: 100,
            mana: 100,
            state: PlayerState::Idle,
            cooldowns: Cooldowns {
                dash: Cooldown {
                    timer: Timer::new(std::time::Duration::from_millis(0), false),
                    cooldown_millis: 3000,
                },
                bow: Cooldown {
                    timer: Timer::new(std::time::Duration::from_millis(0), false),
                    cooldown_millis: 50,
                },
            },
        });
}

fn cooldown_system(time: Res<Time>, mut query: Query<&mut Player>) {
    if let Ok(mut player) = query.single_mut() {
        player.cooldowns.dash.timer.tick(time.delta());
        player.cooldowns.bow.timer.tick(time.delta());
    }
}

fn player_action_system(
    time: Res<Time>,
    mut action_event: EventReader<(PlayerAction, Vec3)>,
    mut query: Query<(&mut Player, &mut Transform)>,
    mut bow_event: EventWriter<BowEvent>,
) {
    let delta_seconds = time.delta_seconds();
    if let Ok((mut player, mut transform)) = query.single_mut() {
        for (action, mouse_coords) in action_event.iter() {
            match action {
                PlayerAction::Move => move_action(&mut player, *mouse_coords),
                PlayerAction::Dash => dash_action(&mut player, *mouse_coords),
                PlayerAction::Bow => {
                    bow_action(&mut player, &transform, *mouse_coords, &mut bow_event)
                }
            }
        }
        match player.state {
            PlayerState::Moving => {
                let direction = player.destination - transform.translation;
                if direction.length() > 5.0 {
                    transform.translation += player.speed * delta_seconds * direction.normalize();
                }
            }
            PlayerState::Channeling(remaining_seconds) => {
                if delta_seconds > remaining_seconds {
                    dash_ability(&mut player, &mut transform);
                } else {
                    player.state = PlayerState::Channeling(remaining_seconds - delta_seconds);
                }
            }
            PlayerState::Idle => (),
        }
    }

    /*
    if let Ok((mut player, _)) = query.single_mut() {
        player.destination = Vec3::new(pos_wld.x, pos_wld.y, 1.0);
    }
    */
}

fn move_action(player: &mut Player, destination: Vec3) {
    if player.can_move() {
        player.state = PlayerState::Moving;
        player.destination = destination;
    }
}

fn dash_action(player: &mut Player, mouse_coords: Vec3) {
    if player.can_move() && player.cooldowns.dash.timer.finished() {
        player.state = PlayerState::Channeling(0.05);
        player.destination = mouse_coords;
        player.cooldowns.dash.timer = Timer::new(
            std::time::Duration::from_millis(player.cooldowns.dash.cooldown_millis),
            false,
        );
    }
}

fn bow_action(
    player: &mut Player,
    transform: &Transform,
    mouse_coords: Vec3,
    bow_event: &mut EventWriter<BowEvent>,
) {
    if player.can_move() && player.cooldowns.bow.timer.finished() {
        player.cooldowns.bow.timer = Timer::new(
            std::time::Duration::from_millis(player.cooldowns.bow.cooldown_millis),
            false,
        );
        bow_event.send(BowEvent {
            player_coords: transform.translation,
            mouse_coords,
        });
    }
}

struct BowProjectile {
    start: Vec3,
    velocity: Vec3,
    range: f32,
}

struct BowEvent {
    player_coords: Vec3,
    mouse_coords: Vec3,
}

fn bow_projectile_system(
    mut commands: Commands,
    mut query: Query<(&BowProjectile, &mut Transform)>,
    mut projectile_event: EventReader<BowEvent>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (projectile, mut transform) in query.iter_mut() {
        transform.translation += projectile.velocity;
    }
    for event in projectile_event.iter() {
        let speed = 30.0;
        let velocity = speed * (event.mouse_coords - event.player_coords).normalize();
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.add(Color::VIOLET.into()),
                transform: Transform::from_translation(event.player_coords),
                sprite: Sprite::new(Vec2::new(15.0, 15.0)),
                ..Default::default()
            })
            .insert(BowProjectile {
                start: event.player_coords,
                velocity,
                range: 2000.0,
            });
    }
}

fn dash_ability(player: &mut Player, transform: &mut Transform) {
    let dash_range = 300.0;
    let direction = player.destination - transform.translation;
    if direction.length() < dash_range {
        transform.translation = player.destination;
    } else {
        transform.translation += dash_range * direction.normalize()
    }
    player.state = PlayerState::Idle;
}

fn input_system(
    wnds: Res<Windows>,
    q_camera: Query<&Transform, With<MainCamera>>,
    mouse_input: Res<Input<MouseButton>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_action: EventWriter<(PlayerAction, Vec3)>,
) {
    // get the primary window
    let wnd = wnds.get_primary().unwrap();

    // check if the cursor is in the primary window
    if let Some(pos) = wnd.cursor_position() {
        // get the size of the window
        let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // the default orthographic projection is in pixels from the center;
        // just undo the translation
        let p = pos - size / 2.0;

        // assuming there is exactly one main camera entity, so this is OK
        let camera_transform = q_camera.single().unwrap();

        // apply the camera transform
        let pos_wld = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);
        let mouse_world_coordinates = Vec3::new(pos_wld.x, pos_wld.y, 1.0);

        if mouse_input.pressed(MouseButton::Right) {
            player_action.send((PlayerAction::Move, mouse_world_coordinates));
        }

        if keyboard_input.pressed(KeyCode::Q) {
            player_action.send((PlayerAction::Dash, mouse_world_coordinates));
        }

        if keyboard_input.pressed(KeyCode::W) {
            player_action.send((PlayerAction::Bow, mouse_world_coordinates));
        }
    }
}
