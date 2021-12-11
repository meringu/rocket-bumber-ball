use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const SPRITE: &str = "ball.png";
const RADIUS: f32 = 32.0;

const RESTITUTION: f32 = 0.7;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system());
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(asset_server.load(SPRITE).into()),
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle {
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::ball(RADIUS / 2.0),
            material: ColliderMaterial {
                restitution: RESTITUTION,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete);
}
