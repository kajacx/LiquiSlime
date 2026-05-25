use liquislime_core::{Faction, GameState, SlimeAmount, SlimeGrid, TilePosition, TimeInterval};
use macroquad::{input, prelude::*};

mod setup;
mod texture_atlas;

#[macroquad::main("Liquislime")]
async fn main() {
    setup::setup_panic_hook();

    let faction0 = Faction::new(0, liquislime_core::Color::new(255, 128, 0));
    let faction1 = Faction::new(1, liquislime_core::Color::new(0, 255, 64));

    let mut state = GameState::new(50, 50);

    state.grids.set_amount(
        faction0.id(),
        TilePosition::new(2, 4),
        SlimeAmount::from_integer(50000),
    );

    state.grids.set_amount(
        faction1.id(),
        TilePosition::new(8, 5),
        SlimeAmount::from_integer(60000),
    );

    // println!("{:?}", std::env::current_dir().unwrap());
    let texture = load_texture("assets/lucy.png").await.unwrap();

    // let mut error = None;

    loop {
        // let result = std::panic::catch_unwind(|| {
        update(&mut state);
        render(&state, &texture);
        // });

        // draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}

fn update(state: &mut GameState) {
    let faction0 = Faction::new(0, liquislime_core::Color::new(255, 128, 0));
    let faction1 = Faction::new(1, liquislime_core::Color::new(0, 255, 64));

    state.update(TimeInterval::from_seconds(get_frame_time() as f64));

    if input::is_mouse_button_pressed(MouseButton::Left) {
        #[allow(unused_must_use)]
        state.grids.try_add_amount(
            faction0.id(),
            TilePosition::new(
                (input::mouse_position().0 as i32) / 10,
                (input::mouse_position().1 as i32) / 10,
            ),
            SlimeAmount::from_integer(1000000),
        );
    }

    if input::is_mouse_button_down(MouseButton::Right) {
        #[allow(unused_must_use)]
        state.grids.try_add_amount(
            faction1.id(),
            TilePosition::new(
                (input::mouse_position().0 as i32) / 10,
                (input::mouse_position().1 as i32) / 10,
            ),
            SlimeAmount::from_integer(10000),
        );

        // let mouse_pos = input::mouse_position();
        // hero_pos = Vec2::new(mouse_pos.0, mouse_pos.1);
    }
}

fn render(state: &GameState, texture: &Texture2D) {
    let faction0 = Faction::new(0, liquislime_core::Color::new(255, 128, 0));
    let faction1 = Faction::new(1, liquislime_core::Color::new(0, 255, 64));

    let hero_pos = Vec2::new(0.0, 0.0);

    clear_background(LIGHTGRAY);

    draw_slime_grid(
        &state.grids.grids[0],
        0.0,
        0.0,
        10.0,
        parse_color(faction0.color()),
    );
    draw_slime_grid(
        &state.grids.grids[1],
        0.0,
        0.0,
        10.0,
        parse_color(faction1.color()),
    );

    draw_texture_ex(
        &texture,
        hero_pos.x - 50.0,
        hero_pos.y - 50.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::new(100.0, 100.0)),
            ..Default::default()
        },
    );
}

fn parse_color(color: liquislime_core::Color) -> Color {
    Color::new(
        color.r as f32 / 255.0,
        color.g as f32 / 255.0,
        color.b as f32 / 255.0,
        1.0,
    )
}

fn draw_slime_grid(grid: &SlimeGrid, offset_x: f32, offset_y: f32, tile_size: f32, color: Color) {
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
