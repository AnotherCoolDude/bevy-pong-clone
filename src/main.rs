use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};

mod components;
use components::{Ball, Collider, FPSText, Goal, Paddle, Player, ScoreText, Wall, TextContainer};

mod systems;
use systems::{ball_collision, ball_movement, paddle_movement, window_resize_listener, update_text};

mod bundles;
use bundles::PaddleBundle;

mod events;
use events::WindowResizeListenerState;

fn main() {
    App::build()
        .add_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_resource(WindowDescriptor::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(ball_movement.system())
        .add_system(paddle_movement.system())
        .add_system(window_resize_listener.system())
        .add_system(ball_collision.system())
        .add_system(update_text.system())
        .run();
}

fn setup(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>, asset_server: Res<AssetServer>) {
    let init_win_size: (f32, f32) = (1280., 720.);

    commands.spawn(Camera2dBundle::default());
    commands.spawn(CameraUiBundle::default());
    spawn_ball(commands, init_win_size);
    spawn_paddle(commands, Player::Left, &mut materials, init_win_size);
    spawn_paddle(commands, Player::Right, &mut materials, init_win_size);
    spawn_wall(commands, Wall::Top, &mut materials);
    spawn_wall(commands, Wall::Bottom, &mut materials);
    spawn_wall(commands, Wall::Left, &mut materials);
    spawn_wall(commands, Wall::Right, &mut materials);
    spawn_goal(commands, Goal::Left, &mut materials);
    spawn_goal(commands, Goal::Right, &mut materials);
    spawn_text(commands, &asset_server, &mut materials);
    commands.insert_resource(WindowResizeListenerState::default());
}

fn spawn_ball(commands: &mut Commands, init_win_size: (f32, f32)) {
    let size: f32 = init_win_size.1 * 0.05;
    commands
        .spawn(SpriteBundle {
            sprite: Sprite::new(Vec2::new(size, size)),
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..Default::default()
        })
        .with(Ball::new(init_win_size.1 / 1.5));
}

fn spawn_paddle(
    commands: &mut Commands,
    player: Player,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    init_win_size: (f32, f32),
) {
    let size: (f32, f32) = (Paddle::WIDTH, init_win_size.1 * 0.2);
    let x_translation = match player {
        Player::Left => -init_win_size.0 * 0.5 + Paddle::MARGIN,
        Player::Right => init_win_size.0 * 0.5 - Paddle::MARGIN - Paddle::WIDTH,
    };

    commands
        .spawn(SpriteBundle {
            material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
            sprite: Sprite::new(Vec2::new(size.0, size.1)),
            transform: Transform::from_translation(Vec3::new(x_translation, 0., 0.)),
            ..Default::default()
        })
        .with_bundle(PaddleBundle::new(player));
}

fn spawn_wall(commands: &mut Commands, wall: Wall, materials: &mut ResMut<Assets<ColorMaterial>>) {
    commands
        .spawn(SpriteBundle {
            material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
            sprite: Sprite::new(wall.size()),
            transform: Transform::from_translation(wall.position()),
            ..Default::default()
        })
        .with(wall)
        .with(Collider::Wall);
}

fn spawn_goal(commands: &mut Commands, goal: Goal, materials: &mut ResMut<Assets<ColorMaterial>>) {
    let (size, pos) = match goal {
        Goal::Left => (Wall::Left.size(), Wall::Left.position()),
        Goal::Right => (Wall::Right.size(), Wall::Right.position()),
    };
    commands
        .spawn(SpriteBundle {
            material: materials.add(Color::NONE.into()),
            sprite: Sprite::new(size),
            transform: Transform::from_translation(pos),
            ..Default::default()
        })
        .with(goal)
        .with(Collider::Goal(goal));
}

fn spawn_text(commands: &mut Commands, asset_server: &Res<AssetServer>, materials: &mut ResMut<Assets<ColorMaterial>>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Px(30.)),
                margin: Rect::all(Val::Px(10.)),
                align_self: AlignSelf::FlexEnd,
                justify_content: JustifyContent::SpaceBetween,
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(TextBundle {
                    style: Style {
                        margin: Rect::all(Val::Px(5.)),
                        ..Default::default()
                    },
                    text: Text {
                        value: "FPS:".to_string(),
                        style: TextStyle {
                            font_size: 30.0,
                            color: Color::BLUE,
                            ..Default::default()
                        },
                        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                    },
                    ..Default::default()
                })
                .with(FPSText)
                .spawn(TextBundle {
                    style: Style {
                        margin: Rect::all(Val::Px(5.)),
                        ..Default::default()
                    },
                    text: Text {
                        value: ScoreText::default().to_string(),
                        style: TextStyle {
                            font_size: 30.0,
                            color: Color::BLUE,
                            ..Default::default()
                        },
                        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                    },
                    ..Default::default()
                })
                .with(ScoreText::default());
        })
        .with(TextContainer);
}