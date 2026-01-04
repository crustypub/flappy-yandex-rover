use bevy::{prelude::*, window::WindowResized};

#[derive(Component)]
struct Background;

pub fn startup() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, update)
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
