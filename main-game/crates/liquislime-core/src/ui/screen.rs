use crate::{Camera, Position};

pub struct Screen {
    pub size: Position,
    pub camera: Camera,
}

impl Screen {
    pub fn new(size: Position) -> Self {
        Self {
            size,
            camera: Camera::new(0.1f32),
        }
    }

    pub fn screen_position_to_world(&self, screen_position: Position) -> Position {
        self.camera.position + self.screen_vector_to_world(screen_position)
    }

    pub fn world_position_to_screen(&self, world_position: Position) -> Position {
        self.world_vector_to_screen(world_position - self.camera.position)
    }

    pub fn screen_vector_to_world(&self, screen_vector: Position) -> Position {
        screen_vector * self.camera.zoom
    }

    pub fn world_vector_to_screen(&self, world_vector: Position) -> Position {
        world_vector / self.camera.zoom
    }
}
