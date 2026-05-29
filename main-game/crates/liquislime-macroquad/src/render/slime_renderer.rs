use liquislime_core::{SlimeGrid, TilePosition};
use macroquad::prelude::*;

pub fn draw_slime_grid(
    grid: &SlimeGrid,
    offset_x: f32,
    offset_y: f32,
    tile_size: f32,
    color: Color,
) {
    for y in 0..grid.height {
        for x in 0..grid.width {
            let amount = grid.get_amount(TilePosition::new(x as _, y as _));
            let amount = amount.as_float() / 1000.0;
            let alpha_value = amount.clamp(0.0, 1.0);
            let slime_color = color.with_alpha(alpha_value);
            draw_rectangle(
                offset_x + x as f32 * tile_size,
                offset_y + y as f32 * tile_size,
                tile_size,
                tile_size,
                slime_color,
            );
        }
    }
}
