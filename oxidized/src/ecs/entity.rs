// -- Create Builder system for Entities.

use crate::ecs::world::{Health, KeyboardControl, World};

struct Entity<'a> {
    world: &'a mut World,
    entity_id: usize,
}

impl<'a> Entity<'a> {
    pub fn new(world: &'a mut World) -> Self {
        let entity_id = world.spawn_empty();
        Self { world, entity_id }
    }

    pub fn with_keyboard_control(self) -> Self {
        self.world.controls[self.entity_id] = Some(KeyboardControl {});
        self
    }

    pub fn with_health(self, max_health: u32) -> Self {
        self.world.health[self.entity_id] = Some(Health {
            current: max_health,
            max_health,
        });
        self
    }

    pub fn build(self) -> usize {
        self.entity_id
    }
}
