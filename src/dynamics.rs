use bevy_ecs::{intern::Interned, schedule::ScheduleLabel};

use crate::{CharacterControllerState, prelude::*};

pub(super) fn plugin(schedule: Interned<dyn ScheduleLabel>) -> impl Fn(&mut App) {
    move |app: &mut App| {
        app.add_systems(
            schedule,
            apply_forces.in_set(AhoySystems::ApplyForcesToDynamicRigidBodies),
        );
    }
}

fn apply_forces(
    kccs: Query<&CharacterControllerState>,
    colliders: Query<&ColliderOf>,
    mut rigid_bodies: Query<(&RigidBody, Forces)>,
) {
    for touch in kccs.iter().flat_map(|state| state.touching_entities.iter()) {
        let Ok(collider_of) = colliders.get(touch.entity) else {
            continue;
        };
        let Ok((rigid_body, mut forces)) = rigid_bodies.get_mut(collider_of.body) else {
            continue;
        };
        if !rigid_body.is_dynamic() {
            continue;
        }
        // TODO: not on step up

        let touch_dir = -touch.normal;
        // TODO: read from character
        let mass = 80.0;
        // TODO: use relative vel
        let velocity = touch.character_velocity;
        let touch_velocity = touch_dir.dot(velocity) * touch_dir;
        let impulse = touch_velocity * mass;
        forces.apply_linear_impulse_at_point(impulse, touch.point);
    }
}
