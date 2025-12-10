use crate::prelude::*;

#[derive(Component, Default, Copy, Reflect, Clone, Debug)]
#[reflect(Component)]
pub struct WaterState {
    pub level: WaterLevel,
    pub speed: f32,
}

#[derive(Default, Copy, Reflect, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum WaterLevel {
    #[default]
    None,
    Touching,
    Center,
}

#[derive(Reflect, Component, Default)]
#[require(Sensor, Transform, GlobalTransform)]
#[reflect(Component)]
pub struct Water {
    pub speed: f32,
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        update_water.before(AhoySystems::MoveCharacters),
    );
}

fn update_water(
    mut objects: Query<(&Position, &mut WaterState, &CollidingEntities)>,
    waters: Query<(&Collider, &Position, &Rotation, &Water)>,
) {
    for (object_position, mut water_state, colliding_entities) in &mut objects {
        water_state.level = WaterLevel::None;
        water_state.speed = f32::MAX;
        let waist = **object_position;
        for (collider, position, rotation, water) in waters.iter_many(colliding_entities.iter()) {
            let level = if collider.contains_point(*position, *rotation, waist) {
                WaterLevel::Center
            } else {
                WaterLevel::Touching
            };

            water_state.level = level.max(water_state.level);
            water_state.speed = water_state.speed.min(water.speed);
        }
    }
}
