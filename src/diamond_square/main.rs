pub mod diamond_square;
use diamond_square::*;

use raylib::prelude::*;

fn main() {
    let opts = DsquareOpts::default();
    let base = vec![vec![0.0; 512]; 512];

    let mut grid = diamond_square(&opts, &base);

    let l = grid.len() as i32;
    let (mut rl, thread) =
        raylib::init().size(l, l).title("diamond_square").build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        if d.is_key_pressed(KeyboardKey::KEY_SPACE) {
            grid = diamond_square(&opts, &base);
        }
        for (x, row) in grid.iter().enumerate() {
            for (y, val) in row.iter().enumerate() {
                let gray = val * 255.0;
                let gray = gray as u8;
                let gray = Color {
                    r: gray,
                    g: gray,
                    b: gray,
                    a: 255,
                };
                d.draw_rectangle(x as i32, y as i32, 1, 1, gray);
            }
        }
    }
}
