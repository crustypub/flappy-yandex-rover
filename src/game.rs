use bevy::prelude::*;

#[derive(Component)]
struct Background;

#[derive(Component)]
struct Column {
    passed: bool,
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
}

const COLUMN_GAP: f32 = 200.0;
const COLUMN_WIDTH: f32 = 80.0;
const COLUMN_SPEED: f32 = -200.0;
const SPAWN_INTERVAL: f32 = 2.0;

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
        })
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .add_systems(FixedUpdate, (spawn_columns, move_columns, despawn_columns))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, windows: Query<&mut Window>) {
    let background_image = asset_server.load("background.png");
    let pos = Vec3::new(0., 0., 0.);
    let window = windows.single().unwrap();

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

fn spawn_columns(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<ColumnTimer>,
    windows: Query<&Window>,
    asset_server: Res<AssetServer>,
    game_state: Res<GameState>,
) {
    // Обновляем таймер
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
                1.0, // z=1 чтобы быть поверх фона
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
    game_state: Res<GameState>,
) {
    let delta = time.delta_secs();

    for (mut transform, mut column) in &mut columns {
        transform.translation.x += game_state.column_speed * delta;

        if transform.translation.x < 0.0 && !column.passed {
            column.passed = true;
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
