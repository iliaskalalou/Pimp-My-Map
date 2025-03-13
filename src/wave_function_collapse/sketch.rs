use crate::tile::Tile;
use crate::Cell;
use ::core::panic;
use rand::seq::SliceRandom;
use raylib::prelude::*;
use serde_json::*;
use std::{
    fs::{self, ReadDir},
    usize,
};

fn get_images(
    paths: ReadDir,
    thread: &RaylibThread,
    handle: &mut RaylibHandle,
) -> Vec<prelude::Texture2D>
{
    let mut image_array: Vec<prelude::Texture2D> = Vec::new();
    for p in paths {
        match p {
            Ok(p) => {
                let filename = p.path();
                if filename.ends_with(".png") {
                    let texture: prelude::Texture2D = handle
                        .load_texture(thread, filename.to_str().unwrap())
                        .unwrap();
                    image_array.push(texture);
                }
            }
            Err(_) => panic!("error with entry"),
        }
    }
    image_array
}

fn extract_config_data(dirname: &str) -> Vec<Value>
{
    let configfile_content = json!(fs::read_to_string(format!("{dirname}/config.json"))
        .expect("config file \"config.json\" not found in directory {dirname}"));

    let object_array = match configfile_content {
        Value::Object(obj) => {
            let binding = obj
                .get("files")
                .expect("couldn't access attribute \"files\" of config.json")
                .clone();
            binding
        }
        _ => panic!("json format not valid."),
    };

    match object_array {
        Value::Array(obj_arr) => obj_arr.to_vec(),
        _ => panic!("Wrong value in field \"files\""),
    }
}

fn extract_object_data(obj: &Map<String, Value>) -> (usize, [String; 4])
{
    let id = match obj.get("id").unwrap() {
        Value::Number(num) => num,
        _ => panic!("Wrond value in field id"),
    };
    let index = id.as_u64().unwrap() as usize;

    let adjs = match obj.get("adjency").unwrap() {
        Value::Array(arr) => arr,
        _ => panic!("Wrond value in field adjency"),
    };

    match adjs.as_slice() {
        [Value::String(n), Value::String(e), Value::String(s), Value::String(o)] => (
            index,
            [n.to_string(), e.to_string(), s.to_string(), o.to_string()],
        ),
        _ => panic!("one adjency is missing or not of the right type"),
    }
}

pub fn preload(dirname: &str, thread: &RaylibThread, handle: &mut RaylibHandle) -> Vec<Tile>
{
    let mut tiles = Vec::new();
    let mut edges_copy: Vec<[String; 4]> = Vec::new();
    let paths = fs::read_dir(dirname).expect("couldn't read into provided directory");

    let image_array: Vec<prelude::Texture2D> = get_images(paths, &thread, handle);
    let object_array = extract_config_data(dirname);

    assert_eq!(object_array.len(), image_array.len());
    for (image, obj) in image_array.into_iter().zip(object_array.iter()) {
        match obj {
            Value::Object(obj) => {
                let (i, edges) = extract_object_data(obj);
                tiles.push(Tile::new(image, &edges, i));
                edges_copy.push(edges.clone());
            }
            _ => panic!("error in json format"),
        }
    }

    tiles.iter_mut().for_each(|t| t.analyze(&edges_copy));
    tiles
}

pub fn init_grid(dimensions: usize, number_options: usize) -> Vec<&Cell>
{
    vec![&Cell::new(Vec::from_iter(0..number_options)); dimensions * dimensions]
}

pub fn check_valid(arr: Vec<usize>, valid: Vec<usize>) -> Vec<usize>
{
    arr.into_iter().filter(|e| valid.contains(e)).collect()
}

pub fn draw_image(
    canvas: &mut RaylibDrawHandle,
    cells: Vec<Cell>,
    tiles: Vec<Tile>,
    dimensions: usize,
)
{
    if cells.iter().all(|c| c.collapsed) {
        for i in 0..dimensions {
            for j in 0..dimensions {
                let cell = &cells[i + j * dimensions];
                let index = cell.options[0];
                let texture: &prelude::Texture2D = &tiles[index].img;
                canvas.draw_texture(&texture, i as i32 * 50, j as i32 * 50, Color::WHITE);
            }
        }
    }
}

pub fn wave_function_collapse(cells: Vec<&Cell>, tiles: Vec<Tile>, dimensions: usize)
    -> Vec<&Cell>
{
    let min_options = cells.iter().map(|e| e.options.len()).min().unwrap();
    let grid_copy = cells.clone();
    let minimal_entropy: Vec<&Cell> = grid_copy
        .into_iter()
        .filter(|e| !e.collapsed && e.options.len() == min_options)
        .collect();

    let mut random_cell = minimal_entropy.choose(&mut rand::thread_rng()).unwrap();
    random_cell.collapsed = true;
    let pick = *random_cell.options.choose(&mut rand::thread_rng()).unwrap();
    random_cell.options = vec![pick];

    let mut next_grid = Vec::with_capacity(dimensions * dimensions);
    for i in 0..dimensions {
        for j in 0..dimensions {
            let index = i + j * dimensions;
            if cells[index].collapsed {
                next_grid[index] = cells[index];
            } else {
                let mut options = Vec::from_iter(0..tiles.len());

                // LOOk UP
                if j > 0 {
                    let up = cells[i + (j - 1) * dimensions];
                    let mut valid_options: Vec<usize> = Vec::new();
                    for option in up.options {
                        let valid = tiles[option].down;
                        valid_options.extend(valid);
                    }
                    options = check_valid(options, valid_options);
                }

                // LOOk RIGHT
                if i < dimensions - 1 {
                    let right = cells[i + 1 + j * dimensions];
                    let mut valid_options: Vec<usize> = Vec::new();
                    for option in right.options {
                        let valid = tiles[option].left;
                        valid_options.extend(valid);
                    }
                    options = check_valid(options, valid_options);
                }

                // LOOk DOWN
                if j < dimensions - 1 {
                    let down = cells[i + (j + 1) * dimensions];
                    let mut valid_options: Vec<usize> = Vec::new();
                    for option in down.options {
                        let valid = tiles[option].up;
                        valid_options.extend(valid);
                    }
                    options = check_valid(options, valid_options);
                }

                // LOOk UP
                if i > 0 {
                    let left = cells[i - 1 + j * dimensions];
                    let mut valid_options: Vec<usize> = Vec::new();
                    for option in left.options {
                        let valid = tiles[option].right;
                        valid_options.extend(valid);
                    }
                    options = check_valid(options, valid_options);
                }
                next_grid[index] = &Cell::new(options);
            }
        }
    }

    next_grid
}
