pub mod arena;
pub mod ball;
pub mod camera;
pub mod rocket;
pub mod window;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn main() {
    let mut app = App::build();
    app.add_plugins(DefaultPlugins);

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin)
        .add_plugin(crate::window::WebFullscreenPlugin);

    app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_startup_system(setup.system());

    app.add_plugin(arena::ArenaPlugin)
        .add_plugin(ball::BallPlugin)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(rocket::RocketPlugin);

    app.run();
}

fn setup(mut rapier_configuration: ResMut<RapierConfiguration>) {
    rapier_configuration.gravity = Vec2::ZERO.into();
}
