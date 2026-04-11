use crate::ecs::world::World;

pub fn system_input(world: &mut World) {
    // for (p, _) in query!((p, v) in (&mut world.physics, &world.controls)) {
    //     if rl.is_key_down(KeyboardKey::KEY_SIX) {
    //         p.velocity.x = HORIZONTAL_VELOCITY;
    //     } else if rl.is_key_down(KeyboardKey::KEY_FOUR) {
    //         p.velocity.x = -HORIZONTAL_VELOCITY;
    //     } else {
    //         p.velocity.x -= p
    //             .velocity
    //             .x
    //             .min(FRICTION * rl.get_frame_time())
    //             .max(-FRICTION * rl.get_frame_time());
    //     }
    //     if rl.is_key_pressed(KeyboardKey::KEY_EIGHT) {
    //         p.velocity.y = JUMP_VELOCITY;
    //     }
    // }
}
