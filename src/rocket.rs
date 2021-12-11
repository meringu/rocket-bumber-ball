use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::f32::consts::FRAC_PI_2;

const BLUE_SPRITE: &str = "blue-rocket.png";
const RED_SPRITE: &str = "red-rocket.png";
const BOOSTER_SPRITE: &str = "booster.png";

const SPRITE_WIDTH: f32 = 32.0;

const ROTATION_FULCRUM: f32 = SPRITE_WIDTH * 10.0;
const ROTATION_POWER: f32 = 35.0;
const ROTATION_DAMPING: f32 = -5.0;

const BOOSTER_POWER: f32 = 500_000.0;
const DRAG: f32 = -1000.0;

const START_DISTANCE: f32 = 400.0;

const RESTITUTION: f32 = 0.7;

#[derive(Debug)]
enum Rocket {
    Blue,
    Red,
}

impl Rocket {
    fn up_key(&self) -> KeyCode {
        match self {
            Rocket::Blue => KeyCode::Up,
            Rocket::Red => KeyCode::W,
        }
    }

    fn left_key(&self) -> KeyCode {
        match self {
            Rocket::Blue => KeyCode::Left,
            Rocket::Red => KeyCode::A,
        }
    }

    fn right_key(&self) -> KeyCode {
        match self {
            Rocket::Blue => KeyCode::Right,
            Rocket::Red => KeyCode::D,
        }
    }
}

pub struct RocketPlugin;

impl Plugin for RocketPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(input.system())
            .add_startup_system(setup.system());
    }
}

fn input(
    input: Res<Input<KeyCode>>,
    mut query: Query<(
        &Rocket,
        &RigidBodyPosition,
        &RigidBodyVelocity,
        &mut RigidBodyForces,
        &RigidBodyMassProps,
        &Children,
    )>,
    mut booster_query: Query<&mut Visible>,
) {
    for (rocket, position, velocity, mut forces, props, children) in query.iter_mut() {
        for &child in children.iter() {
            if let Ok(mut booster_visible) = booster_query.get_mut(child) {
                forces.apply_force_at_point(
                    props,
                    velocity.linvel * DRAG,
                    position.position.translation.vector.into(),
                );

                if input.pressed(rocket.up_key()) {
                    forces.apply_force_at_point(
                        props,
                        (Vec2::new(
                            -position.position.rotation.angle().sin(),
                            position.position.rotation.angle().cos(),
                        ) * BOOSTER_POWER)
                            .into(),
                        position.position.translation.vector.into(),
                    );
                    if !booster_visible.is_visible {
                        booster_visible.is_visible = true;
                    }
                } else if booster_visible.is_visible {
                    booster_visible.is_visible = false;
                }

                let mut rot_force_mag = velocity.angvel * ROTATION_DAMPING;

                if input.pressed(rocket.left_key()) {
                    rot_force_mag += ROTATION_POWER;
                }

                if input.pressed(rocket.right_key()) {
                    rot_force_mag -= ROTATION_POWER;
                }

                forces.apply_force_at_point(
                    props,
                    (Vec2::new(
                        -position.position.rotation.angle().cos(),
                        -position.position.rotation.angle().sin(),
                    ) * rot_force_mag
                        * ROTATION_FULCRUM)
                        .into(),
                    position
                        .position
                        .transform_point(&(Vec2::new(0.0, ROTATION_FULCRUM)).into()),
                );
            }
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let booster_sprite = materials.add(asset_server.load(BOOSTER_SPRITE).into());

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(asset_server.load(RED_SPRITE).into()),
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle {
            position: (Vec2::new(-START_DISTANCE, 0.0), -FRAC_PI_2).into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::ball(SPRITE_WIDTH / 2.0),
            material: ColliderMaterial {
                restitution: RESTITUTION,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete)
        .insert(Rocket::Red)
        .with_children(|parent| {
            parent.spawn_bundle(SpriteBundle {
                transform: Transform {
                    translation: -Vec3::Y * SPRITE_WIDTH,
                    ..Default::default()
                },
                material: booster_sprite.clone(),
                ..Default::default()
            });
        });

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(asset_server.load(BLUE_SPRITE).into()),
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle {
            position: (Vec2::new(START_DISTANCE, 0.0), FRAC_PI_2).into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::ball(SPRITE_WIDTH / 2.0),
            material: ColliderMaterial {
                restitution: RESTITUTION,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete)
        .insert(Rocket::Blue)
        .with_children(|parent| {
            parent.spawn_bundle(SpriteBundle {
                transform: Transform {
                    translation: -Vec3::Y * SPRITE_WIDTH,
                    ..Default::default()
                },
                material: booster_sprite.clone(),
                ..Default::default()
            });
        });
}
