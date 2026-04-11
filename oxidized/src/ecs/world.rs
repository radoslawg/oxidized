pub struct KeyboardControl;

pub struct Health {
    pub current: u32,
    pub max_health: u32,
}

// ------- World
pub struct World {
    pub controls: Vec<Option<KeyboardControl>>,
    pub health: Vec<Option<Health>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            controls: Vec::new(),
            health: Vec::new(),
        }
    }

    pub fn spawn_empty(&mut self) -> usize {
        let i = self.controls.len();
        self.controls.push(None);
        self.health.push(None);
        i
    }
}
