use crate::Position;

pub struct Camera {
    /** Top-left of the camera in world coordinates */
    pub position: Position,

    /** One screen pixel equals `zoom` world units */
    pub zoom: f32,
}

impl Camera {
    pub fn new(zoom: f32) -> Self {
        Self {
            position: Position::new(0.0, 0.0),
            zoom,
        }
    }

    pub fn change_zoom(&mut self, zoom_amount: f32) {
        self.zoom *= zoom_amount;
    }

    pub fn pan_by(&mut self, pan_amount: Position) {
        self.position += pan_amount * self.zoom;
    }
}
