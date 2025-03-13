pub mod perlin;

use perlin::{perlin2d, perlin3d, random_permutations, PerlinOpts};
extern crate rand;

use raylib::{core::math::Rectangle, prelude::*};
use std::{time::Duration, usize};

const WINDOW_HEIGHT: usize = 640;
const WINDOW_WIDTH: usize = 480;
// const CHUNK_COUNT: usize = 8;

const OCTAVES: usize = 8;
const FALLOUT: f32 = 0.5;
const LACUNARITY: f32 = 2.0;
const FREQ: f32 = 0.005;

#[derive(Clone)]
struct Chunk {
    rect: Rectangle,
    coords: Vector3,
    color: Color,
}

fn build_chunks(dims: (usize, usize), chunk_size: usize) -> Vec<Chunk> {
    let mut chunks: Vec<Chunk> = Vec::with_capacity((dims.0) * (dims.1));

    for i in 0..dims.0 / chunk_size {
        for j in 0..dims.1 / chunk_size {
            chunks.push(Chunk {
                rect: Rectangle::new(
                    (i * chunk_size as usize) as f32,
                    (j * chunk_size as usize) as f32,
                    chunk_size as f32,
                    chunk_size as f32,
                ),
                color: Color::BLACK,
                coords: Vector3::new(i as f32, j as f32, 0.0),
            });
        }
    }

    return chunks;
}

fn color_chunks(chunks: &mut Vec<Chunk>, permuts: &[usize; 512]) {
    for chunk in chunks.iter_mut() {
        chunk.color = {
            let gray: u8 = (perlin3d(&PerlinOpts {
                x: chunk.coords.x * chunk.rect.width * FREQ,
                y: chunk.coords.y * chunk.rect.width * FREQ,
                z: 1.0,
                base: 0.0,
                permutations: *permuts,
                octaves: OCTAVES,
                fallout: FALLOUT,
                lacunarity: LACUNARITY,
            }) * 255.0) as u8;

            rcolor(gray, gray, gray, 255)
        }
    }
}

fn draw_chunks(canvas: &mut RaylibDrawHandle, chunks: &Vec<Chunk>) {
    for chunk in chunks {
        let (x, y, h, w) = (
            chunk.rect.x as i32,
            chunk.rect.y as i32,
            chunk.rect.height as i32,
            chunk.rect.width as i32,
        );

        canvas.draw_rectangle(x, y, h, w, chunk.color);
    }
}

pub fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("raylib_plain_test")
        .build();

    let mut chunk_size: usize = 8;
    let dims: (usize, usize) = (WINDOW_HEIGHT as usize, WINDOW_WIDTH as usize);
    let mut chunks = build_chunks(dims, chunk_size);

    let mut permuts: [usize; 512] = random_permutations();
    rl.set_target_fps(60);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        if d.is_key_pressed(KeyboardKey::KEY_SPACE) {
            color_chunks(&mut chunks, &permuts);
            permuts = random_permutations();
        }

        if d.is_window_resized() {}

        if d.is_key_pressed(KeyboardKey::KEY_UP) {
            if chunk_size < std::usize::MAX / 2 + 1 {
                chunk_size *= 2;
            }
            chunks = build_chunks(dims, chunk_size);
        }

        if d.is_key_pressed(KeyboardKey::KEY_DOWN) {
            if chunk_size > 1 {
                chunk_size /= 2;
            }
            chunks = build_chunks(dims, chunk_size);
        }

        d.clear_background(Color::WHITE);
        draw_chunks(&mut d, &chunks);
    }
}
