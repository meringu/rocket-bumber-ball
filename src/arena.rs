use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const HALF_HEIGHT: f32 = 720.0 / 2.0;
const HALF_WIDTH: f32 = 1280.0 / 2.0;
const WALL_HALF_THICKNESS: f32 = 10.0;

const RESTITUTION: f32 = 0.7;

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system());
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(RigidBodyBundle {
            position: Vec2::new(HALF_WIDTH + WALL_HALF_THICKNESS, 0.0).into(),
            body_type: RigidBodyType::Static,
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(WALL_HALF_THICKNESS, HALF_HEIGHT),
            material: ColliderMaterial {
                restitution: RESTITUTION,
                ..Default::default()
            },
            ..Default::default()
        });

    commands
        .spawn_bundle(RigidBodyBundle {
            position: Vec2::new(-HALF_WIDTH - WALL_HALF_THICKNESS, 0.0).into(),
            body_type: RigidBodyType::Static,

            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(WALL_HALF_THICKNESS, HALF_HEIGHT),
            material: ColliderMaterial {
                restitution: RESTITUTION,
                ..Default::default()
            },
            ..Default::default()
        });

    commands
        .spawn_bundle(RigidBodyBundle {
            position: Vec2::new(0.0, HALF_HEIGHT + WALL_HALF_THICKNESS).into(),
            body_type: RigidBodyType::Static,
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(HALF_WIDTH, WALL_HALF_THICKNESS),
            material: ColliderMaterial {
                restitution: RESTITUTION,
                ..Default::default()
            },
            ..Default::default()
        });

    commands
        .spawn_bundle(RigidBodyBundle {
            position: Vec2::new(0.0, -HALF_HEIGHT - WALL_HALF_THICKNESS).into(),
            body_type: RigidBodyType::Static,
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(HALF_WIDTH, WALL_HALF_THICKNESS),
            material: ColliderMaterial {
                restitution: RESTITUTION,
                ..Default::default()
            },
            ..Default::default()
        });
}
