use bevy::prelude::*;

#[derive(Component)]
struct Background;

#[derive(Component)]
struct Column {
    passed: bool,
}

#[derive(Component)]
struct Bird {
    velocity: f32,
    alive: bool,
}

#[derive(Resource)]
struct ColumnTimer {
    timer: Timer,
}

#[derive(Resource)]
struct GameState {
    gap_height: f32,
    column_width: f32,
    column_speed: f32,
    min_height: f32,
    max_height: f32,
    gravity: f32,
    jump_force: f32,
    bird_size: f32,
    game_score: f32,
}

const COLUMN_GAP: f32 = 300.0;
const COLUMN_WIDTH: f32 = 80.0;
const COLUMN_SPEED: f32 = -400.0;
const SPAWN_INTERVAL: f32 = 1.5;
const GRAVITY: f32 = -800.0;
const JUMP_FORCE: f32 = 250.0;

// bird resolution = 972x1280px
const BIRD_SIZE_WIDTH: f32 = 97.2;
const BIRD_SIZE_HEIGHT: f32 = 128.0;

pub fn startup() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ColumnTimer {
            timer: Timer::from_seconds(SPAWN_INTERVAL, TimerMode::Repeating),
        })
        .insert_resource(GameState {
            gap_height: COLUMN_GAP,
            column_width: COLUMN_WIDTH,
            column_speed: COLUMN_SPEED,
            min_height: 100.0,
            max_height: 400.0,
            gravity: GRAVITY,
            jump_force: JUMP_FORCE,
            bird_size: BIRD_SIZE_WIDTH,
            game_score: 0.0,
        })
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                spawn_columns,
                move_columns,
                despawn_columns,
                update,
                bird_input,
                bird_movement,
                check_collisions,
                restart_game,
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Query<&mut Window>,
    game_state: Res<GameState>,
) {
    let background_image = asset_server.load("background.png");
    let pos = Vec3::new(0., 0., 0.);
    let window = windows.single().unwrap();
    let text_justification = Justify::Left;

    commands
        .spawn((Node {
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::FlexStart,
            overflow: Overflow::visible(),
            max_width: Val::Px(0.0),
            ..default()
        },))
        .with_children(|builder| {
            builder.spawn((
                Text::new("Score: ".to_string() + &game_state.game_score.to_string()),
                TextFont::from_font_size(24.0),
                TextLayout::new_with_justify(text_justification).with_no_wrap(),
                // TextColor(text.color),
            ));
        });
    commands.spawn((
        Sprite {
            image: background_image,
            custom_size: Some(Vec2::new(window.width(), window.height())),
            image_mode: SpriteImageMode::Tiled {
                tile_x: true,
                tile_y: false,
                stretch_value: 1.5,
            },
            ..default()
        },
        Transform::from_translation(pos),
        Background,
    ));

    commands.spawn((
        Camera2d,
        Transform::default().looking_at(Vec3::ZERO, Vec3::Y),
    ));

    let bird_texture = asset_server.load("rover.png");
    commands.spawn((
        Sprite {
            image: bird_texture,
            custom_size: Some(Vec2::new(BIRD_SIZE_WIDTH, BIRD_SIZE_HEIGHT)),
            ..default()
        },
        Transform::from_translation(Vec3::new(-100.0, 0.0, 2.0)),
        Bird {
            velocity: 0.0,
            alive: false,
        },
    ));
}

fn update(mut backgrounds: Query<&mut Sprite, With<Background>>, windows: Query<&Window>) {
    let window = windows.single().unwrap();
    let window_size = Vec2::new(window.width(), window.height());

    for mut sprite in &mut backgrounds {
        if sprite.custom_size != Some(window_size) {
            sprite.custom_size = Some(window_size);
        }
    }
}

fn bird_input(
    mut birds: Query<&mut Bird>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    game_state: Res<GameState>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) || mouse_input.just_pressed(MouseButton::Left) {
        for mut bird in &mut birds {
            if bird.alive {
                bird.velocity = game_state.jump_force;
            }
        }
    }
}

fn bird_movement(
    mut birds: Query<(&mut Bird, &mut Transform)>,
    time: Res<Time>,
    game_state: Res<GameState>,
    windows: Query<&Window>,
) {
    let window = windows.single().unwrap();
    let delta = time.delta_secs();

    for (mut bird, mut transform) in &mut birds {
        if bird.alive {
            bird.velocity += game_state.gravity * delta;

            transform.translation.y += bird.velocity * delta;

            let rotation = (bird.velocity / game_state.jump_force * 0.5).clamp(-0.5, 0.5);
            transform.rotation = Quat::from_rotation_z(rotation);

            if transform.translation.y > window.height() / 2.0 - game_state.bird_size / 2.0 {
                transform.translation.y = window.height() / 2.0 - game_state.bird_size / 2.0;
                bird.velocity = 0.0;
            }
        }
    }
}

fn check_collisions(
    mut birds: Query<(&mut Bird, &Transform)>,
    columns: Query<(&Transform, &Sprite), With<Column>>,
    windows: Query<&Window>,
    game_state: Res<GameState>,
) {
    let window = windows.single().unwrap();

    for (mut bird, bird_transform) in &mut birds {
        if !bird.alive {
            continue;
        }

        let bird_pos = bird_transform.translation;
        let half_bird_size = game_state.bird_size / 2.0;

        if bird_pos.y < -window.height() / 2.0 + half_bird_size {
            bird.alive = false;
            bird.velocity = 0.0;
            continue;
        }

        for (column_transform, column_sprite) in &columns {
            let column_pos = column_transform.translation;
            let column_size = column_sprite.custom_size.unwrap_or(Vec2::ZERO);

            let half_column_width = column_size.x / 2.0;
            let half_column_height = column_size.y / 2.0;

            let x_overlap = (bird_pos.x - column_pos.x).abs() < half_bird_size + half_column_width;
            let y_overlap = (bird_pos.y - column_pos.y).abs() < half_bird_size + half_column_height;

            if x_overlap && y_overlap {
                bird.alive = false;
                bird.velocity = 0.0;
                break;
            }
        }
    }
}

fn restart_game(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    birds: Query<(Entity, &Bird)>,
    columns: Query<Entity, With<Column>>,
    asset_server: Res<AssetServer>,
) {
    let just_pressed_keys: Vec<KeyCode> = keyboard_input.get_just_pressed().cloned().collect();
    let game_active = birds.iter().any(|bird| bird.1.alive);

    if !just_pressed_keys.is_empty() && !game_active {
        for column_entity in &columns {
            commands.entity(column_entity).despawn();
        }

        for (bird_entity, _) in &birds {
            commands.entity(bird_entity).despawn();

            let bird_texture = asset_server.load("rover.png");
            commands.spawn((
                Sprite {
                    image: bird_texture,
                    custom_size: Some(Vec2::new(BIRD_SIZE_WIDTH, BIRD_SIZE_HEIGHT)),
                    ..default()
                },
                Transform::from_translation(Vec3::new(-100.0, 0.0, 2.0)),
                Bird {
                    velocity: 0.0,
                    alive: true,
                },
            ));
        }
    }
}

fn spawn_columns(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<ColumnTimer>,
    windows: Query<&Window>,
    asset_server: Res<AssetServer>,
    game_state: Res<GameState>,
) {
    timer.timer.tick(time.delta());

    if timer.timer.just_finished() {
        let window = windows.single().unwrap();
        let column_texture = asset_server.load("column.png");

        let bottom_height = rand::random::<f32>() * (game_state.max_height - game_state.min_height)
            + game_state.min_height;
        let top_height = window.height() - bottom_height - game_state.gap_height;

        let spawn_x = window.width() / 2.0 + game_state.column_width / 2.0;

        commands.spawn((
            Sprite {
                image: column_texture.clone(),
                custom_size: Some(Vec2::new(game_state.column_width, bottom_height)),
                ..default()
            },
            Transform::from_translation(Vec3::new(
                spawn_x,
                -window.height() / 2.0 + bottom_height / 2.0,
                1.0,
            )),
            Column { passed: false },
        ));

        commands.spawn((
            Sprite {
                image: column_texture,
                custom_size: Some(Vec2::new(game_state.column_width, top_height)),
                flip_y: true,
                ..default()
            },
            Transform::from_translation(Vec3::new(
                spawn_x,
                window.height() / 2.0 - top_height / 2.0,
                1.0,
            )),
            Column { passed: false },
        ));
    }
}

fn move_columns(
    mut columns: Query<(&mut Transform, &mut Column)>,
    time: Res<Time>,
    mut game_state: ResMut<GameState>,
    birds: Query<&Bird>,
) {
    let delta = time.delta_secs();
    let game_active = birds.iter().any(|bird| bird.alive);

    for (mut transform, mut column) in &mut columns {
        if game_active {
            transform.translation.x += game_state.column_speed * delta;
        }

        if transform.translation.x < 0.0 && !column.passed {
            column.passed = true;
            game_state.game_score = game_state.game_score + 1.0;
        }
    }
}

fn despawn_columns(
    mut commands: Commands,
    columns: Query<(Entity, &Transform), With<Column>>,
    windows: Query<&Window>,
) {
    let window = windows.single().unwrap();

    for (entity, transform) in &columns {
        if transform.translation.x < -window.width() / 2.0 - 100.0 {
            commands.entity(entity).despawn();
        }
    }
}
