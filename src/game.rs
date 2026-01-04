use bevy::{audio::AudioPlugin, prelude::*};

pub fn startup() {
    App::new()
        .add_plugins(DefaultPlugins.build().disable::<AudioPlugin>())
        .add_systems(Startup, setup)
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
                stretch_value: 1.0,
            },
            ..default()
        },
        Transform::from_translation(pos),
    ));
    commands.spawn((
        Camera2d,
        Transform::default().looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
