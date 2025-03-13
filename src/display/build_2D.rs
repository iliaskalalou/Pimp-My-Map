use raylib::prelude::*;

extern crate noise;

use noise::{Seedable, Perlin, NoiseFn};
use rand::Rng; 
use rand::seq::SliceRandom; 


const DEEP_WATER_THRESHOLD: f64 = 0.3;
const DEEP_THRESHOLD: f64 = 0.2;
const MOUNTAIN_THRESHOLD: f64 = 0.7;
pub const DARK_FOREST_THRESHOLD: f64 = 0.45;
const SNOW_THRESHOLD: f64 = 0.82; 
const HUMIDITY_THRESHOLD: f64 = 0.7;
const GLACEIL_THRESHOLD: f64 = 0.25; 
const DESSERT_THRESHOLD: f64 = 0.75;


const FOREST_AP: f64 = 0.3;
const DARK_FOREST_AP: f64 = 0.6;

const DEEP_WATER: f64 = -2.0;
const WATER: f64 = -1.0;
const DESSERT: f64 = 2.0;
const MOUTAIN: f64 = 3.0;
const MOUNTAIN_SNOW: f64 = 4.0;
const GLACIAL: f64 = 5.0;
const DESSERT_SHARA: f64 = 6.0;
const DARK_FOREST: f64 = 7.0;
const MARSH: f64 = 8.0;
const CITY: f64 = 10.0;
const VILLAGE: f64 = 11.0;
const ROUTE: f64 = 12.0;


const VILLAGE_VALUE: f64 = 10.0; 
const VILLE_VALUE: f64 = 11.0; 
const PIXELS_PAR_VILLAGE: usize = 8000;
const PIXELS_PAR_VILLE: usize = 16000;

const VILLAGE_DISTANCE_MAX_CARREE: usize = 100 * 100; 
const VILLE_DISTANCE_MAX_CARREE: usize = 200 * 200;  
const VILLAGE_MAX_ROUTES: usize = 3;
const CITY_MAX_ROUTES: usize = 5;

const MIN_DESERT_SIZE: usize = 40;
const MIN_GLACIAL_SIZE: usize = 40;


pub fn generate_perlin_noise_matrix(width: usize, height: usize, scale: f64) -> Vec<Vec<f64>> {
    let seed = rand::random::<u32>();
    let perlin = Perlin::new().set_seed(seed);

    let mut matrix = vec![vec![0.0; width]; height];

    for y in 0..height {
        for x in 0..width {
            let nx = x as f64 / scale; 
            let ny = y as f64 / scale;
            let noise_val = perlin.get([nx, ny, 0.0]);
            matrix[y][x] = (noise_val + 1.0) / 2.0; 

        }
    }
    
    matrix
}




use std::usize;

use rand::{rngs::ThreadRng};

fn __diamond_square(mut grid: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let h = grid.len();
    let mut rng = rand::thread_rng();

    let mut i = h - 1;
    while i > 1 {
        let id = i / 2;
        for x in (id..h).step_by(i) {
            for y in (id..h).step_by(i) {
                let moyenne = (grid[x - id][y - id] + grid[x - id][y + id] +
                               grid[x + id][y + id] + grid[x + id][y - id]) / 4.0;
                grid[x][y] = moyenne + rng.gen_range(-(id as f64)..=id as f64);
            }
        }
        let mut decalage = 0;
        for x in (0..h).step_by(id) {
            if decalage == 0 {
                decalage = id;
            } else {
                decalage = 0;
            }
            for y in (decalage..h).step_by(i) {
                let mut somme = 0.0;
                let mut n = 0;
                if x >= id {
                    somme += grid[x - id][y];
                    n += 1;
                }
                if x + id < h {
                    somme += grid[x + id][y];
                    n += 1;
                }
                if y >= id {
                    somme += grid[x][y - id];
                    n += 1;
                }
                if y + id < h {
                    somme += grid[x][y + id];
                    n += 1;
                }
                grid[x][y] = somme / n as f64 + rng.gen_range(-(id as f64)..=id as f64);
            }
        }
        i = id;
    }
    grid
}

pub fn diamond_square(n: u32) -> Vec<Vec<f64>>
{
    let l = (2_i32.pow(n) + 1) as usize;
    let grid = vec![vec![0.0;l]; l];
    let grid = __diamond_square(grid);
    grid
}


fn combine_matrices
(
    matrix1: Vec<Vec<f64>>, 
    matrix2: Vec<Vec<f64>>
) -> Vec<Vec<f64>> 
{
    let rows = matrix1.len();
    let cols = matrix1[0].len();
    let mut combined_matrix = vec![vec![0.0; cols]; rows];

    for i in 0..rows 
    {
        for j in 0..cols 
        {
            combined_matrix[i][j] = (matrix1[i][j] + matrix2[i][j]) / 2.0;
        }
    }

    combined_matrix
}


fn apply_deep_water
(
    base_matrix: &mut Vec<Vec<f64>>, 
    altitude_matrix: &Vec<Vec<f64>>, 
    threshold: f64
) 
{
    let size = base_matrix.len();
    for i in 0..size 
    {
        for j in 0..size 
        {
            if altitude_matrix[i][j] < threshold 
            {
                base_matrix[i][j] = DEEP_WATER;
            }
        }
    }
}

fn apply_water
(
    base_matrix: &mut Vec<Vec<f64>>, 
    altitude_matrix: &Vec<Vec<f64>>, 
    threshold: f64
) 
{
    let size = base_matrix.len();
    for i in 0..size 
    {
        for j in 0..size 
        {
            if base_matrix[i][j] == DEEP_WATER && altitude_matrix[i][j] < threshold 
            {
                base_matrix[i][j] = WATER;
            }
        }
    }
}

fn apply_sand_biome
(
    base_matrix: &mut Vec<Vec<f64>>, 
    altitude_matrix: &Vec<Vec<f64>>, 
    SAND: usize
) 
{
    let size = base_matrix.len();
    for i in 0..size 
    {
        for j in 0..size 
        {
            if base_matrix[i][j] == WATER || base_matrix[i][j] == DEEP_WATER
            { 
                for di in i.saturating_sub(SAND)..=(i + SAND).min(size - 1) 
                {
                    for dj in j.saturating_sub(SAND)..=(j + SAND).min(size - 1) 
                    {
                        if base_matrix[di][dj] != WATER && base_matrix[di][dj] != DEEP_WATER && altitude_matrix[di][dj] < 0.7 
                        {
                            base_matrix[di][dj] = DESSERT;
                        }
                    }
                }
            }
        }
    }
}



fn apply_sand_to_shallow_water_transformation
(
    base_matrix: &mut Vec<Vec<f64>>,
    water_zones: &Vec<Vec<bool>>,
    size: usize,
    shallow_water_value: f64,
    sand_value: f64,
) 
{
    for i in 0..size 
    {
        for j in 0..size 
        {
            if !water_zones[i][j] && base_matrix[i][j] == DESSERT
            {
                base_matrix[i][j] = DEEP_WATER; 
            }
        }
    }
}


fn mark_water_zones
(
    base_matrix: &Vec<Vec<f64>>,
    size: usize,
    shallow_water_value: f64,
    deep_water_value: f64
) -> Vec<Vec<bool>> 
{

    let mut water_zones = vec![vec![false; size]; size];
    let mut deep_water_zones = vec![vec![false; size]; size];

    for i in 0..size 
    {
        for j in 0..size 
        {
            if base_matrix[i][j] == shallow_water_value 
            {
                deep_water_zones[i][j] = true;
            }
        }
    }

    test(&mut deep_water_zones, &base_matrix, size);
    // println!("PPPPPPPPP");
    mark_sand_near_ocean(&mut deep_water_zones, &base_matrix, size, DESSERT);
    enlarge_deep_water_zones(&mut deep_water_zones, size);
    deep_water_zones
}

fn enlarge_deep_water_zones
(
    deep_water_zones: &mut Vec<Vec<bool>>, 
    size: usize
) 
{
    let mut to_mark = vec![vec![false; size]; size];

    for i in 0..size 
    {
        for j in 0..size 
        {
            if deep_water_zones[i][j] 
            {
                for di in -2..=2 
                {
                    for dj in -2..=2 
                    {
                        let new_i = i as isize + di;
                        let new_j = j as isize + dj;

                        if new_i >= 0 && new_i < size as isize && new_j >= 0 && new_j < size as isize 
                        {
                            to_mark[new_i as usize][new_j as usize] = true;
                        }
                    }
                }
            }
        }
    }

    for i in 0..size 
    {
        for j in 0..size 
        {
            if to_mark[i][j] 
            {
                deep_water_zones[i][j] = true;
            }
        }
    }
}


fn test
(
    deep_water_zones: &mut Vec<Vec<bool>>,
    base_matrix: &Vec<Vec<f64>>,
    size: usize,
)
{
    for i in 0..size 
    {
        for j in 0..size 
        {
            if deep_water_zones[i][j] 
            {
                extend_zone_to_shallow_water(i, j, deep_water_zones, base_matrix, size);
            }
        }
    }
}

fn mark_sand_near_ocean
(
    deep_water_zones: &mut Vec<Vec<bool>>,
    base_matrix: &Vec<Vec<f64>>,
    size: usize,
    sand_value: f64,
) 
{
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    for i in 0..size 
    {
        for j in 0..size 
        {
            if base_matrix[i][j] == sand_value 
            {
                for (dx, dy) in &directions 
                {
                    let new_i = (i as isize) + dx;
                    let new_j = (j as isize) + dy;

                    if new_i >= 0 && new_i < size as isize && new_j >= 0 && new_j < size as isize 
                    {
                        let new_i = new_i as usize;
                        let new_j = new_j as usize;

                        if deep_water_zones[new_i][new_j] 
                        {
                            deep_water_zones[i][j] = true;
                            break;  
                        }
                    }
                }
            }
        }
    }
}



fn extend_zone_to_shallow_water
(
    i: usize,
    j: usize,
    deep_water_zones: &mut Vec<Vec<bool>>,
    base_matrix: &Vec<Vec<f64>>,
    size: usize,
) 
{
    let mut stack = Vec::new();
    stack.push((i, j));

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    while let Some((current_i, current_j)) = stack.pop() {
        for (di, dj) in directions.iter() {
            let new_i = current_i as isize + *di;
            let new_j = current_j as isize + *dj;

            if new_i >= 0 && new_i < size as isize && new_j >= 0 && new_j < size as isize {
                let new_i = new_i as usize;
                let new_j = new_j as usize;

                if base_matrix[new_i][new_j] == DEEP_WATER && !deep_water_zones[new_i][new_j] {
                    deep_water_zones[new_i][new_j] = true;
                    stack.push((new_i, new_j));
                }
            }
        }
    }
}









fn apply_mountain_biome
(
    base_matrix: &mut Vec<Vec<f64>>, 
    altitude_matrix: &Vec<Vec<f64>>, 
    mountain_threshold: f64, 
    snow_threshold: f64
) 
{
    let size = base_matrix.len();
    for i in 0..size 
    {
        for j in 0..size 
        {
            if altitude_matrix[i][j] > mountain_threshold 
            {
                if altitude_matrix[i][j] > snow_threshold 
                {
                    base_matrix[i][j] = MOUNTAIN_SNOW; 
                } 
                else 
                {
                    base_matrix[i][j] = MOUTAIN; 
                }
            }
        }
    }
}


fn apply_dark_forest
(
    base_matrix: &mut Vec<Vec<f64>>, 
    altitude_matrix: &Vec<Vec<f64>>, 
    humidity_matrix: &Vec<Vec<f64>>, 
    forest: usize, 
    humidity_threshold: f64
) 
{
    let size = base_matrix.len();
    for i in 0..size 
    {
        for j in 0..size 
        {
            if base_matrix[i][j] == FOREST_AP
            { 
                let mut is_near_mountain = false;
                for di in i.saturating_sub(forest)..=(i + forest).min(size - 1) 
                {
                    for dj in j.saturating_sub(forest)..=(j + forest).min(size - 1) 
                    {
                        if base_matrix[di][dj] == MOUTAIN
                        {
                            is_near_mountain = true;
                            break;
                        }
                    }
                    if is_near_mountain { break; }
                }

                if is_near_mountain || humidity_matrix[i][j] > humidity_threshold 
                {
                    base_matrix[i][j] = MARSH; 
                }
            }
        }
    }
}



fn apply_desert_biome
(
    base_matrix: &mut Vec<Vec<f64>>,
    temperature_matrix: &Vec<Vec<f64>>,
    temperature_threshold: f64,
) 
{
    let size = base_matrix.len();
    for i in 0..size 
    {
        for j in 0..size 
        {
            if temperature_matrix[i][j] > temperature_threshold && 
            base_matrix[i][j] != DEEP_WATER && base_matrix[i][j] != WATER &&
            base_matrix[i][j] != MOUNTAIN_SNOW && /*base_matrix[i][j] != GLACIAL && */
            base_matrix[i][j] != MOUTAIN //&& base_matrix[i][j] != MARSH
            {
                base_matrix[i][j] = DESSERT_SHARA; 
            }
        }
    }
    delete_small_deserts(base_matrix,DESSERT_SHARA,FOREST_AP,MIN_DESERT_SIZE);
}

fn delete_small_deserts
(
    base_matrix: &mut Vec<Vec<f64>>,
    desert_value: f64,
    plain_value: f64,
    min_desert_size: usize,
) 
{
    let size = base_matrix.len();
    let mut visited = vec![vec![false; size]; size];

    for i in 0..size 
    {
        for j in 0..size 
        {
            if base_matrix[i][j] == desert_value && !visited[i][j] 
            {
                let mut current_size = 0;
                let mut stack = vec![(i, j)];
                let mut desert_cells = Vec::new();

                while let Some((x, y)) = stack.pop() 
                {
                    if x < size && y < size && base_matrix[x][y] == desert_value && !visited[x][y] 
                    {
                        visited[x][y] = true;
                        current_size += 1;
                        desert_cells.push((x, y));

                        if x > 0 { stack.push((x - 1, y)); }
                        if y > 0 { stack.push((x, y - 1)); }
                        if x + 1 < size { stack.push((x + 1, y)); }
                        if y + 1 < size { stack.push((x, y + 1)); }
                    }
                }

                if current_size < min_desert_size 
                {
                    for (x, y) in desert_cells 
                    {
                        base_matrix[x][y] = plain_value;
                    }
                }
            }
        }
    }
}


fn apply_glacial_biome
(
    base_matrix: &mut Vec<Vec<f64>>,
    temperature_matrix: &Vec<Vec<f64>>,
    temperature_threshold: f64,
) 
{
    let size = base_matrix.len();
    for i in 0..size 
    {
        for j in 0..size 
        {
            if temperature_matrix[i][j] < temperature_threshold && 
            base_matrix[i][j] != DEEP_WATER && base_matrix[i][j] != WATER &&
            base_matrix[i][j] != MOUNTAIN_SNOW && 
            base_matrix[i][j] != MOUTAIN && base_matrix[i][j] != MARSH
            {
                base_matrix[i][j] = GLACIAL; 
            }
        }
    }
    delete_small_glacial(base_matrix,GLACIAL,FOREST_AP,MIN_GLACIAL_SIZE);

}

fn delete_small_glacial
(
    base_matrix: &mut Vec<Vec<f64>>,
    desert_value: f64,
    plain_value: f64,
    min_desert_size: usize,
) 
{
    let size = base_matrix.len();
    let mut visited = vec![vec![false; size]; size];

    for i in 0..size 
    {
        for j in 0..size 
        {
            if base_matrix[i][j] == desert_value && !visited[i][j] 
            {
                let mut current_size = 0;
                let mut stack = vec![(i, j)];
                let mut desert_cells = Vec::new();

                while let Some((x, y)) = stack.pop() 
                {
                    if x < size && y < size && base_matrix[x][y] == desert_value && !visited[x][y] 
                    {
                        visited[x][y] = true;
                        current_size += 1;
                        desert_cells.push((x, y));

                        if x > 0 { stack.push((x - 1, y)); }
                        if y > 0 { stack.push((x, y - 1)); }
                        if x + 1 < size { stack.push((x + 1, y)); }
                        if y + 1 < size { stack.push((x, y + 1)); }
                    }
                }

                if current_size < min_desert_size 
                {
                    for (x, y) in desert_cells 
                    {
                        base_matrix[x][y] = plain_value;
                    }
                }
            }
        }
    }
}



fn apply_marsh_biome
(
    base_matrix: &mut Vec<Vec<f64>>,
    humidity_matrix: &Vec<Vec<f64>>,
    temperature_matrix: &Vec<Vec<f64>>,
    humidity_threshold: f64,
    marsh_value: f64, 
    sand_value: f64,
    water_value: f64, 
    water_deep_value: f64, 
) 
{
    let size = base_matrix.len();
    for i in 0..size 
    {
        for j in 0..size 
        {
            if humidity_matrix[i][j] > humidity_threshold && (base_matrix[i][j] == water_value || base_matrix[i][j] == water_deep_value || base_matrix[i][j] == sand_value || base_matrix[i][j] >= 0.0 && base_matrix[i][j] <= 0.35) 
            {
                let mut temp = vec![];                
                for di in -1..=1 
                {
                    for dj in -1..=1 
                    {
                        let adj_i = i as isize + di;
                        let adj_j = j as isize + dj;
                        
                        if adj_i >= 0 && adj_i < size as isize && adj_j >= 0 && adj_j < size as isize 
                        {
                            let adj_i = adj_i as usize;
                            let adj_j = adj_j as usize;
                            
                            if base_matrix[adj_i][adj_j] == water_value || base_matrix[adj_i][adj_j] == water_deep_value 
                            {
                                temp.push((i, j));
                                break;
                            }
                        }
                    }
                }

                for (marsh_i, marsh_j) in temp 
                {
                    base_matrix[marsh_i][marsh_j] = marsh_value;
                    for di in -3..=3 
                    {
                        for dj in -3..=3 
                        {
                            let adj_i = marsh_i as isize + di;
                            let adj_j = marsh_j as isize + dj;
                            
                            if adj_i >= 0 && adj_i < size as isize && adj_j >= 0 && adj_j < size as isize 
                            {
                                let adj_i = adj_i as usize;
                                let adj_j = adj_j as usize;
                                if (base_matrix[adj_i][adj_j] == water_value || base_matrix[adj_i][adj_j] == water_deep_value || base_matrix[adj_i][adj_j] == sand_value || base_matrix[adj_i][adj_j] >= 0.0 && base_matrix[adj_i][adj_j] <= 0.35) && humidity_matrix[adj_i][adj_j] > humidity_threshold 
                                {
                                    base_matrix[adj_i][adj_j] = marsh_value;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}



fn identifier_et_mesurer_les_plaines
(
    base_matrix: &Vec<Vec<f64>>
) -> Vec<(usize, usize, usize)> 
{
    let size = base_matrix.len();
    let mut visited = vec![vec![false; size]; size];
    let mut zones_de_plaines = Vec::new();

    for i in 0..size 
    {
        for j in 0..size 
        {
            if base_matrix[i][j] < DARK_FOREST_THRESHOLD && base_matrix[i][j] > 0.0 && !visited[i][j] 
            {
                let taille = mesurer_zone_de_plaine(i, j, base_matrix, &mut visited, size);
                zones_de_plaines.push((i, j, taille));
            }
        }
    }

    zones_de_plaines
}

fn mesurer_zone_de_plaine(
    i: usize, 
    j: usize, 
    base_matrix: &Vec<Vec<f64>>, 
    visited: &mut Vec<Vec<bool>>, 
    size: usize
) -> usize 
{
    let mut stack = Vec::new();
    stack.push((i, j));

    let mut area_size = 0;

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    while let Some((current_i, current_j)) = stack.pop() 
    {
        if current_i >= size || current_j >= size || visited[current_i][current_j] || (base_matrix[current_i][current_j] > DARK_FOREST_THRESHOLD && base_matrix[current_i][current_j] > 0.0) 
        {
            continue;
        }

        visited[current_i][current_j] = true;
        area_size += 1;

        for (di, dj) in directions.iter() 
        {
            let new_i = current_i.wrapping_add(*di as usize);
            let new_j = current_j.wrapping_add(*dj as usize);

            if new_i < size && new_j < size && !visited[new_i][new_j] && base_matrix[new_i][new_j] < DARK_FOREST_THRESHOLD && base_matrix[new_i][new_j] > 0.0 
            {
                stack.push((new_i, new_j));
            }
        }
    }

    area_size
}


use rand::thread_rng;

fn generer_villages_et_villes
(
    base_matrix: &mut Vec<Vec<f64>>, 
    zones_de_plaines: Vec<(usize, usize, usize)>
) 
{
    let mut rng = thread_rng();

    for (_, _, taille) in zones_de_plaines.iter() 
    {
        let mut village_positions = Vec::new();
        let mut city_positions = Vec::new();
        
        let mut compteur_pixels = 0;
        let mut villages_a_placer = 0;

        for i in 0..base_matrix.len() 
        {
            for j in 0..base_matrix[0].len() 
            {
                if base_matrix[i][j] < DARK_FOREST_THRESHOLD && base_matrix[i][j] > 0.0 
                {
                    village_positions.push((i, j));
                    compteur_pixels += 1;
                    if compteur_pixels % PIXELS_PAR_VILLE == 0 
                    {
                        city_positions.push((i, j));
                        villages_a_placer = 0; 
                    } 
                    else if compteur_pixels % PIXELS_PAR_VILLAGE == 0 
                    {
                        villages_a_placer += 1;
                        if villages_a_placer < 5 
                        {
                            village_positions.push((i, j));
                        }
                    }
                }
            }
        }

        let villes_a_placer = taille / PIXELS_PAR_VILLE;
        for _ in 0..villes_a_placer 
        {
            if let Some(&(i, j)) = city_positions.choose(&mut rng) 
            {
                placer_construction(base_matrix, i, j, VILLE_VALUE);
            }
        }

        let villages_a_placer = taille / PIXELS_PAR_VILLAGE;
        for _ in 0..villages_a_placer 
        {
            if let Some(&(i, j)) = village_positions.choose(&mut rng) 
            {
                placer_construction(base_matrix, i, j, VILLAGE_VALUE);
            }
        }
    }
}

fn placer_construction
(
    base_matrix: &mut Vec<Vec<f64>>, 
    i: usize, 
    j: usize, 
    valeur: f64
) 
{
    match valeur 
    {
        11.0 => 
        {
            if i > 2 && j > 2 && (i + 2) < base_matrix.len() && (j + 2) < base_matrix[0].len() 
            {
                base_matrix[i - 1][j] = valeur;
                base_matrix[i + 1][j] = valeur;
                base_matrix[i][j - 1] = valeur;
                base_matrix[i][j + 1] = valeur;
                base_matrix[i - 2][j] = valeur;
                base_matrix[i + 2][j] = valeur;
                base_matrix[i][j - 2] = valeur;
                base_matrix[i][j + 2] = valeur;
                base_matrix[i][j] = valeur;
            }
        },
        10.0 => 
        {
            if i > 2 && j > 2  && (i + 3) < base_matrix.len() && (j + 3) < base_matrix[0].len() 
            {
                base_matrix[i - 1][j] = valeur;
                base_matrix[i + 1][j] = valeur;
                base_matrix[i][j - 1] = valeur;
                base_matrix[i][j + 1] = valeur;
                base_matrix[i - 2][j] = valeur;
                base_matrix[i + 2][j] = valeur;
                base_matrix[i][j - 2] = valeur;
                base_matrix[i][j + 2] = valeur;
                base_matrix[i][j] = valeur;
            }
        },
        _ => {}
    }
}



use std::collections::HashSet;
use std::cmp::Ordering;

struct UnionFind 
{
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind 
{
    fn new(n: usize) -> Self 
    {
        UnionFind 
        {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    fn find(&mut self, x: usize) -> usize 
    {
        if self.parent[x] != x 
        {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) 
    {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x != root_y 
        {
            if self.rank[root_x] > self.rank[root_y] 
            {
                self.parent[root_y] = root_x;
            } 
            else if self.rank[root_x] < self.rank[root_y] 
            {
                self.parent[root_x] = root_y;
            } 
            else 
            {
                self.parent[root_y] = root_x;
                self.rank[root_x] += 1;
            }
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Edge 
{
    cost: usize,
    start: usize,
    end: usize,
}

impl Ord for Edge 
{
    fn cmp(&self, other: &Self) -> Ordering 
    {
        other.cost.cmp(&self.cost).reverse()
    }
}

impl PartialOrd for Edge 
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> 
    {
        Some(self.cmp(other))
    }
}

fn kruskal
(
    points: Vec<(usize, usize)>, 
    base_matrix: &mut Vec<Vec<f64>>, 
    route_value: f64, 
    restricted_values: &HashSet<i64>
) 
{
    let mut edges = Vec::new();
    let n = points.len();

    for i in 0..n 
    {
        for j in i + 1..n 
        {
            let start = points[i];
            let end = points[j];
            let distance = distance_carree(start, end);
            let max_distance = if base_matrix[start.0][start.1] == VILLAGE || base_matrix[end.0][end.1] == VILLAGE 
            {
                VILLAGE_DISTANCE_MAX_CARREE
            } 
            else 
            {
                VILLE_DISTANCE_MAX_CARREE
            };
            if distance <= max_distance 
            {
                edges.push(Edge { cost: distance, start: i, end: j });
            }
        }
    }

    edges.sort_by(|a, b| a.cost.cmp(&b.cost));

    let mut uf = UnionFind::new(n);
    let mut mst = Vec::new();

    let mut connections = vec![0; n];

    for edge in edges 
    {
        if uf.find(edge.start) != uf.find(edge.end) 
        {
            let start_value = base_matrix[points[edge.start].0][points[edge.start].1];
            let end_value = base_matrix[points[edge.end].0][points[edge.end].1];
            let max_connections = if start_value == VILLAGE || end_value == VILLAGE 
            {
                VILLAGE_MAX_ROUTES
            } 
            else 
            {
                CITY_MAX_ROUTES
            };

            if connections[edge.start] < max_connections && connections[edge.end] < max_connections 
            {
                if tracer_route(base_matrix, points[edge.start], points[edge.end], route_value, restricted_values) 
                {
                    uf.union(edge.start, edge.end);
                    mst.push(edge);
                    connections[edge.start] += 1;
                    connections[edge.end] += 1;
                }
            }
        }
    }
}

fn bezier_curve
(
    start: (usize, usize), 
    control: (usize, usize), 
    end: (usize, usize)
) -> Vec<(usize, usize)> 
{
    let mut path = Vec::new();
    for t in 0..=100 
    {
        let t = t as f64 / 100.0;
        let x = ((1.0 - t) * (1.0 - t) * start.0 as f64 + 2.0 * (1.0 - t) * t * control.0 as f64 + t * t * end.0 as f64) as usize;
        let y = ((1.0 - t) * (1.0 - t) * start.1 as f64 + 2.0 * (1.0 - t) * t * control.1 as f64 + t * t * end.1 as f64) as usize;
        path.push((x, y));
    }
    path
}

fn tracer_route
(
    base_matrix: &mut Vec<Vec<f64>>,
    start: (usize, usize),
    end: (usize, usize),
    route_value: f64,
    restricted_values: &HashSet<i64>,
) -> bool 
{
    let control = ((start.0 + end.0) / 2, (start.1 + end.1) / 2); 
    let path = bezier_curve(start, control, end);

    for &(px, py) in &path 
    {
        if restricted_values.contains(&f64_to_i64(base_matrix[px][py])) 
        {
            return false;
        }
    }

    for &(px, py) in &path 
    {
        if base_matrix[px][py] != route_value 
        {
            base_matrix[px][py] = route_value;
        }
    }

    true
}

fn f64_to_i64(value: f64) -> i64 
{
    value.to_bits() as i64
}

fn distance_carree((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> usize 
{
    let dx = (x2 as isize - x1 as isize).abs() as usize;
    let dy = (y2 as isize - y1 as isize).abs() as usize;
    dx * dx + dy * dy
}

fn collecter_villes_et_villages
(
    base_matrix: &Vec<Vec<f64>>,
    ville_value: f64,
    village_value: f64
) -> Vec<(usize, usize)> 
{
    let mut points = Vec::new();
    let size = base_matrix.len();

    for i in 2..(size - 2) 
    {
        for j in 2..(size - 2) 
        {
            if base_matrix[i][j] == ville_value || base_matrix[i][j] == village_value 
            {
                if base_matrix[i - 1][j] == base_matrix[i][j] &&
                   base_matrix[i + 1][j] == base_matrix[i][j] &&
                   base_matrix[i][j - 1] == base_matrix[i][j] &&
                   base_matrix[i][j + 1] == base_matrix[i][j] &&
                   base_matrix[i - 2][j] == base_matrix[i][j] &&
                   base_matrix[i + 2][j] == base_matrix[i][j] &&
                   base_matrix[i][j - 2] == base_matrix[i][j] &&
                   base_matrix[i][j + 2] == base_matrix[i][j] 
                {
                    points.push((i, j));
                }
            }
        }
    }

    // println!("Nombre de points de villes et villages collectés : {}", points.len());
    points
}



















fn apply_convolution(matrix: &mut Vec<Vec<f64>>) 
{
    let size = matrix.len();
    let mut new_matrix = matrix.clone();

    for _ in 0..5 
    {  
        for i in 1..size-1 
        {
            for j in 1..size-1 
            {
                let mut sum = 0.0;
                for di in -1..=1 
                {
                    for dj in -1..=1 
                    {
                        let new_i = (i as isize + di) as usize;
                        let new_j = (j as isize + dj) as usize;
                        sum += matrix[new_i][new_j];
                    }
                }
                new_matrix[i][j] = sum / 9.0; 
            }
        }
        *matrix = new_matrix.clone();  
    }
}

pub fn normalize(grid: &mut Vec<Vec<f64>>)
{
    let mut min_val = f64::INFINITY;
    let mut max_val = f64::NEG_INFINITY;
    for row in grid.iter() 
    {
        for &val in row.iter() 
        {
            if val < min_val 
            {
                min_val = val;
            }
            if val > max_val 
            {
                max_val = val;
            }
        }
    }

    if (min_val - max_val).abs() > f64::EPSILON 
    {
        for row in grid.iter_mut() 
        {
            for val in row.iter_mut() 
            {
                *val = (*val - min_val) / (max_val - min_val);
            }
        }
    }
}

fn calculer_moyenne_matrices(mat1: Vec<Vec<f64>>, mat2: Vec<Vec<f64>>) -> Vec<Vec<f64>> 
{
    let max_rows = std::cmp::max(mat1.len(), mat2.len());
    let max_cols = std::cmp::max(
        mat1.iter().map(|row| row.len()).max().unwrap_or(0),
        mat2.iter().map(|row| row.len()).max().unwrap_or(0),
    );

    let mut result = vec![vec![0.0; max_cols]; max_rows];


    for i in 0..max_rows 
    {
        for j in 0..max_cols 
        {
            result[i][j] = ((mat1[i][j] + mat2[i][j]) / 2.0).abs();
        }
    }

    result
}

fn convert_matrix_f32_to_f64(matrix_f32: Vec<Vec<f32>>) -> Vec<Vec<f64>> 
{
    matrix_f32.into_iter()
        .map(|row| row.into_iter()
            .map(|elem| elem as f64)
            .collect::<Vec<f64>>())
        .collect()
}

use std::thread;
use std::time::Duration;
use raylib::prelude::*;
use std::path::Path;
use raylib::ffi;



pub fn apply_biomes
(
    base_matrix: &mut Vec<Vec<f64>>, 
    altitude_matrix: &Vec<Vec<f64>>,
    temperature_matrix: &Vec<Vec<f64>>,
    humidity_matrix: &Vec<Vec<f64>>
) 
{

    apply_deep_water(base_matrix, altitude_matrix, DEEP_WATER_THRESHOLD);
    apply_water(base_matrix, altitude_matrix, DEEP_THRESHOLD);
    apply_sand_biome(base_matrix, altitude_matrix, DESSERT as usize);

    let size = base_matrix.len(); 
    let water_zones = mark_water_zones(&base_matrix, size, WATER, DEEP_WATER);
    apply_sand_to_shallow_water_transformation(base_matrix,&water_zones,base_matrix.len(),DEEP_WATER,DESSERT);
    

    apply_mountain_biome(base_matrix, altitude_matrix, MOUNTAIN_THRESHOLD,SNOW_THRESHOLD);
    //apply_dark_forest(base_matrix, altitude_matrix, humidity_matrix, MOUTAIN, DARK_FOREST_THRESHOLD);
    apply_marsh_biome(base_matrix,humidity_matrix,temperature_matrix,MOUNTAIN_THRESHOLD,MARSH,DESSERT,WATER,DEEP_WATER);
    apply_desert_biome(base_matrix, temperature_matrix, DESSERT_THRESHOLD);
    apply_glacial_biome(base_matrix, temperature_matrix, GLACEIL_THRESHOLD);

    let zones_de_plaines = identifier_et_mesurer_les_plaines(base_matrix);
    generer_villages_et_villes(base_matrix, zones_de_plaines);


    //let points = collecter_villes_et_villages(base_matrix, CITY, VILLAGE);
    //generer_routes_simples(base_matrix, points, ROUTE, CITY, VILLAGE);

    let points = collecter_villes_et_villages(base_matrix, CITY, VILLAGE);
    let restricted_values = HashSet::from([
        f64_to_i64(DEEP_WATER),
        f64_to_i64(WATER),
        f64_to_i64(DESSERT),
        f64_to_i64(MOUTAIN),
        f64_to_i64(MOUNTAIN_SNOW),
        f64_to_i64(GLACIAL),
        f64_to_i64(DESSERT_SHARA),
        f64_to_i64(MARSH),
    ]);
    kruskal(points, base_matrix, ROUTE, &restricted_values);


}



pub fn test_2D(img_texture: &mut Image, img_water: &mut Image) -> Vec<Vec<f64>> 
{
    let (w, h) = (2000, 2000); 
    let (mut rl, thread) = raylib::init()
        .size(w, h)
        .title("Exemple de Carte")
        .build();

    rl.set_target_fps(60);



    let mut perlin_matrix = generate_perlin_noise_matrix(1000,1000,90.0);
    let mut perlin_matrix2 = generate_perlin_noise_matrix(1000,1000,90.0);
    let altitude_matrix = generate_perlin_noise_matrix(1000,1000,90.0);
    let temperature_matrix = generate_perlin_noise_matrix(1000,1000,90.0);
    let humidity_matrix = generate_perlin_noise_matrix(1000,1000,90.0);


    perlin_matrix = combine_matrices(perlin_matrix, perlin_matrix2);
    
    apply_biomes(&mut perlin_matrix,&altitude_matrix,&temperature_matrix,&humidity_matrix);
    
    let mut image = Image::gen_image_color(w as i32, h as i32, Color::WHITE);

    for (i, row) in perlin_matrix.iter().enumerate() 
    {
        for (j, &val) in row.iter().enumerate() 
        {
            let color = match val 
            {
                -2.0 => Color::BLUE,
                -1.0 => Color::DARKBLUE,
                2.0 => Color::GOLD,
                3.0 => Color::GRAY,
                4.0 => Color::LIGHTGRAY, 
                5.0 => Color::WHITE, 
                6.0 => Color::YELLOW,
                7.0 => Color::DARKGREEN,
                8.0 => Color::PURPLE,
                -5.0 => Color::BLACK,
                10.0 => Color::RED,
                11.0 => Color::ORANGE,
                12.0 => Color::BLACK,
                _ if val >= 0.0 && val < DARK_FOREST_THRESHOLD => Color::GREEN,
                _ if val >= DARK_FOREST_THRESHOLD => Color::DARKGREEN,
                _ => Color::DARKGREEN,
            };
            image.draw_pixel(j as i32, i as i32, color);
        }
    }
    export_image(&image, "carte.png");

    while !rl.window_should_close() 
    {
        let mut d = rl.begin_drawing(&thread);
        
        d.clear_background(Color::WHITE);
        

        for (i, row) in perlin_matrix.iter().enumerate() 
        {
            for (j, &val) in row.iter().enumerate() 
            {
                let color = match val {
                    -2.0 => Color::BLUE, 
                    -1.0 => Color::DARKBLUE,
                    2.0 => Color::GOLD,
                    3.0 => Color::GRAY, 
                    4.0 => Color::LIGHTGRAY, 
                    5.0 => Color::WHITE, 
                    6.0 => Color::YELLOW,
                    7.0 => Color::DARKGREEN,
                    8.0 => Color::PURPLE,
                    -5.0 => Color::BLACK,
                    10.0 => Color::RED,
                    11.0 => Color::ORANGE,
                    12.0 => Color::BLACK,
                    _ if val >= 0.0 && val < DARK_FOREST_THRESHOLD => Color::GREEN, 
                    _ if val >= DARK_FOREST_THRESHOLD => Color::DARKGREEN, 
                    _ => Color::DARKGREEN,
                };
                if color == Color::BLUE || color == Color::DARKBLUE 
                {
                    let mut color = color;
                    color.a = 200;
                    img_water.draw_rectangle(j as i32, i as i32, 1, 1, color);
                }
                img_texture.draw_rectangle(j as i32 * 1, i as i32 * 1, 1, 1, color);
                d.draw_rectangle(j as i32 * 1, i as i32 * 1, 1, 1, color);
            }
        }
    }
    altitude_matrix
}



pub fn test_2D_Diamond(img_texture: &mut Image, img_water: &mut Image) -> Vec<Vec<f64>> 
{
    let (w, h) = (2000, 2000); 
    let (mut rl, thread) = raylib::init()
        .size(w, h)
        .title("Exemple de Carte")
        .build();

    rl.set_target_fps(60);

// par 10 c 1025
// par 11 c'est 2049
    let mut perlin_matrix = diamond_square(10);
    let mut perlin_matrix2 = diamond_square(10);
    let mut altitude_matrix = diamond_square(10);
    let mut temperature_matrix = diamond_square(10);
    let mut humidity_matrix = diamond_square(10);

    normalize(&mut perlin_matrix);
    normalize(&mut perlin_matrix2);
    normalize(&mut altitude_matrix);
    normalize(&mut temperature_matrix);
    normalize(&mut humidity_matrix);

    apply_biomes(&mut perlin_matrix,&altitude_matrix,&temperature_matrix,&humidity_matrix);
    

    let mut image = Image::gen_image_color(w as i32, h as i32, Color::WHITE);

    for (i, row) in perlin_matrix.iter().enumerate() 
    {
        for (j, &val) in row.iter().enumerate() 
        {
            let color = match val 
            {
                -2.0 => Color::BLUE,
                -1.0 => Color::DARKBLUE,
                2.0 => Color::GOLD,
                3.0 => Color::GRAY,
                4.0 => Color::LIGHTGRAY, 
                5.0 => Color::WHITE, 
                6.0 => Color::YELLOW,
                7.0 => Color::DARKGREEN,
                8.0 => Color::PURPLE,
                -5.0 => Color::BLACK,
                10.0 => Color::RED,
                11.0 => Color::ORANGE,
                12.0 => Color::BLACK,
                _ if val >= 0.0 && val < DARK_FOREST_THRESHOLD => Color::GREEN,
                _ if val >= DARK_FOREST_THRESHOLD => Color::DARKGREEN,
                _ => Color::DARKGREEN,
            };
            image.draw_pixel(j as i32, i as i32, color);
        }
    }
    export_image(&image, "carte.png");


    while !rl.window_should_close() 
    {
        let mut d = rl.begin_drawing(&thread);
        
        d.clear_background(Color::WHITE);
        

        for (i, row) in perlin_matrix.iter().enumerate() 
        {
            for (j, &val) in row.iter().enumerate() 
            {
                let color = match val {
                    -2.0 => Color::BLUE, 
                    -1.0 => Color::DARKBLUE,
                    2.0 => Color::GOLD,
                    3.0 => Color::GRAY, 
                    4.0 => Color::LIGHTGRAY, 
                    5.0 => Color::WHITE, 
                    6.0 => Color::YELLOW,
                    7.0 => Color::DARKGREEN,
                    8.0 => Color::PURPLE,
                    -5.0 => Color::BLACK,
                    10.0 => Color::RED,
                    11.0 => Color::ORANGE,
                    12.0 => Color::BLACK,
                    _ if val >= 0.0 && val < DARK_FOREST_THRESHOLD => Color::GREEN, 
                    _ if val >= DARK_FOREST_THRESHOLD => Color::DARKGREEN, 
                    _ => Color::DARKGREEN,
                };
                if color == Color::BLUE || color == Color::DARKBLUE 
                {
                    let mut color = color;
                    color.a = 200;
                    img_water.draw_rectangle(j as i32, i as i32, 1, 1, color);
                }
                img_texture.draw_rectangle(j as i32 * 1, i as i32 * 1, 1, 1, color);
                d.draw_rectangle(j as i32 * 1, i as i32 * 1, 1, 1, color);
            }
        }
    }

    altitude_matrix
}

pub fn test_2D_MIX_Diamond_Perlin(img_texture: &mut Image, img_water: &mut Image) -> Vec<Vec<f64>> 
{
    let (w, h) = (2000, 2000); 
    let (mut rl, thread) = raylib::init()
        .size(w, h)
        .title("Exemple de Carte")
        .build();

    rl.set_target_fps(60);



    let mut t_perlin_matrix = diamond_square(10);
    let mut t_perlin_matrix2 = diamond_square(10);
    let mut t_altitude_matrix = diamond_square(10);
    let mut t_temperature_matrix = diamond_square(10);
    let mut t_humidity_matrix = diamond_square(10);

    normalize(&mut t_perlin_matrix);
    normalize(&mut t_perlin_matrix2);
    normalize(&mut t_altitude_matrix);
    normalize(&mut t_temperature_matrix);
    normalize(&mut t_humidity_matrix);

    let mut perlin_matrix = calculer_moyenne_matrices(t_perlin_matrix, generate_perlin_noise_matrix(1025,1025,90.0));
    let mut perlin_matrix2 = calculer_moyenne_matrices(t_perlin_matrix2, generate_perlin_noise_matrix(1025,1025,90.0));
    let altitude_matrix = calculer_moyenne_matrices(t_altitude_matrix,generate_perlin_noise_matrix(1025,1025,90.0));
    let temperature_matrix = calculer_moyenne_matrices(t_temperature_matrix,generate_perlin_noise_matrix(1025,1025,90.0));
    let humidity_matrix = calculer_moyenne_matrices(t_humidity_matrix,generate_perlin_noise_matrix(1025,1025,90.0));

    apply_biomes(&mut perlin_matrix,&altitude_matrix,&temperature_matrix,&humidity_matrix);
    

    let mut image = Image::gen_image_color(w as i32, h as i32, Color::WHITE);

    for (i, row) in perlin_matrix.iter().enumerate() 
    {
        for (j, &val) in row.iter().enumerate() 
        {
            let color = match val 
            {
                -2.0 => Color::BLUE,
                -1.0 => Color::DARKBLUE,
                2.0 => Color::GOLD,
                3.0 => Color::GRAY,
                4.0 => Color::LIGHTGRAY, 
                5.0 => Color::WHITE, 
                6.0 => Color::YELLOW,
                7.0 => Color::DARKGREEN,
                8.0 => Color::PURPLE,
                -5.0 => Color::BLACK,
                10.0 => Color::RED,
                11.0 => Color::ORANGE,
                12.0 => Color::BLACK,
                _ if val >= 0.0 && val < DARK_FOREST_THRESHOLD => Color::GREEN,
                _ if val >= DARK_FOREST_THRESHOLD => Color::DARKGREEN,
                _ => Color::DARKGREEN,
            };
            image.draw_pixel(j as i32, i as i32, color);
        }
    }
    export_image(&image, "carte.png");


    while !rl.window_should_close() 
    {
        let mut d = rl.begin_drawing(&thread);
        
        d.clear_background(Color::WHITE);
        

        for (i, row) in perlin_matrix.iter().enumerate() 
        {
            for (j, &val) in row.iter().enumerate() 
            {
                let color = match val {
                    -2.0 => Color::BLUE, 
                    -1.0 => Color::DARKBLUE,
                    2.0 => Color::GOLD,
                    3.0 => Color::GRAY, 
                    4.0 => Color::LIGHTGRAY, 
                    5.0 => Color::WHITE, 
                    6.0 => Color::YELLOW,
                    7.0 => Color::DARKGREEN,
                    8.0 => Color::PURPLE,
                    -5.0 => Color::BLACK,
                    10.0 => Color::RED,
                    11.0 => Color::ORANGE,
                    12.0 => Color::BLACK,
                    _ if val >= 0.0 && val < DARK_FOREST_THRESHOLD => Color::GREEN, 
                    _ if val >= DARK_FOREST_THRESHOLD => Color::DARKGREEN, 
                    _ => Color::DARKGREEN,
                };
                if color == Color::BLUE || color == Color::DARKBLUE 
                {
                    let mut color = color;
                    color.a = 200;
                    img_water.draw_rectangle(j as i32, i as i32, 1, 1, color);
                }
                img_texture.draw_rectangle(j as i32 * 1, i as i32 * 1, 1, 1, color);
                d.draw_rectangle(j as i32 * 1, i as i32 * 1, 1, 1, color);
            }
        }
    }

    altitude_matrix
}




use rand::prelude::*;

use rand::seq::IteratorRandom;

#[derive(Copy, Clone, Debug)]
struct Tile {
    value: f64,
}

impl Tile {
    fn new(value: f64) -> Self {
        Tile { value }
    }

    fn all() -> Vec<Self> {
        (0..=10).map(|i| Tile::new(i as f64 / 10.0)).collect()
    }

    fn to_f64(self) -> f64 {
        self.value
    }
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        (self.value - other.value).abs() < std::f64::EPSILON
    }
}

impl Eq for Tile {}

impl std::hash::Hash for Tile {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(self.value.to_bits());
    }
}

fn allowed_neighbors(tile: Tile) -> HashSet<Tile> {
    let mut neighbors = HashSet::new();
    for t in Tile::all() {
        neighbors.insert(t);
    }
    neighbors
}

struct WaveFunction {
    width: usize,
    height: usize,
    possibilities: Vec<Vec<HashSet<Tile>>>,
}

impl WaveFunction {
    fn new(width: usize, height: usize) -> Self {
        let all_tiles: HashSet<Tile> = Tile::all().into_iter().collect();
        let possibilities = vec![vec![all_tiles; height]; width];
        WaveFunction { width, height, possibilities }
    }

    fn collapse(&mut self) {
        let mut rng = thread_rng();
        let mut step = 0;
        while !self.is_collapsed() {
            step += 1;
            // println!("Step: {}", step);
            let (x, y) = self.find_lowest_entropy_cell();
            // println!("Collapsing cell: ({}, {})", x, y);
            let possibilities = &self.possibilities[x][y];
            if possibilities.is_empty() {
                // println!("No possibilities left for cell: ({}, {})", x, y);
                return;
            }
            let tile = *possibilities.iter().choose(&mut rng).unwrap();
            self.possibilities[x][y] = vec![tile].into_iter().collect();
            self.propagate_constraints(x, y);
            // println!("Collapsed cell: ({}, {}) to tile: {:?}", x, y, tile);
        }
        // println!("Collapse completed.");
    }

    fn is_collapsed(&self) -> bool {
        self.possibilities.iter().all(|row| row.iter().all(|cell| cell.len() == 1))
    }

    fn find_lowest_entropy_cell(&self) -> (usize, usize) {
        let mut min_entropy = usize::MAX;
        let mut min_pos = (0, 0);

        for x in 0..self.width {
            for y in 0..self.height {
                let entropy = self.possibilities[x][y].len();
                if entropy > 1 && entropy < min_entropy {
                    min_entropy = entropy;
                    min_pos = (x, y);
                }
            }
        }

        min_pos
    }

    fn propagate_constraints(&mut self, x: usize, y: usize) {
        // println!("Propagating constraints from cell: ({}, {})", x, y);
        let mut stack = vec![(x, y)];
        let mut visited = vec![vec![false; self.height]; self.width];

        while let Some((cx, cy)) = stack.pop() {
            if visited[cx][cy] {
                continue;
            }
            visited[cx][cy] = true;

            let tile = *self.possibilities[cx][cy].iter().next().unwrap();
            let neighbors = [
                (cx.wrapping_sub(1), cy),
                (cx + 1, cy),
                (cx, cy.wrapping_sub(1)),
                (cx, cy + 1),
            ];

            for &(nx, ny) in neighbors.iter() {
                if nx < self.width && ny < self.height {
                    let allowed = allowed_neighbors(tile);
                    let initial_len = self.possibilities[nx][ny].len();
                    self.possibilities[nx][ny].retain(|t| allowed.contains(t));
                    let new_len = self.possibilities[nx][ny].len();
                    
                    if new_len < initial_len {
                        stack.push((nx, ny));
                    }
                }
            }
        }
        // println!("Finished propagating constraints from cell: ({}, {})", x, y);
    }

    fn to_matrix(&self) -> Vec<Vec<f64>> {
        self.possibilities.iter().map(|row| {
            row.iter().map(|cell| {
                cell.iter().next().unwrap().to_f64()
            }).collect()
        }).collect()
    }
}








pub fn test_2D_WFC(img_texture: &mut Image, img_water: &mut Image) -> Vec<Vec<f64>> 
{
    let (w, h) = (2000, 2000); 
    let (mut rl, thread) = raylib::init()
        .size(w, h)
        .title("Exemple de Carte")
        .build();

    rl.set_target_fps(60);

    let width = 100;
    let height = 100;
    // println!("ERRRRRRRRRRRRRRR");
    let mut wf1 = WaveFunction::new(width, height);
    let mut wf2 = WaveFunction::new(width, height);
    let mut wf3 = WaveFunction::new(width, height);
    let mut wf4 = WaveFunction::new(width, height);
    let mut wf5 = WaveFunction::new(width, height);
    // println!("ERRRRRRRRRRRRRRR");
    wf1.collapse();
    // println!("ERRRRRRRRRRRRRRR");
    wf2.collapse();
    wf3.collapse();
    // println!("ERRRRRRRRRRRRRRR");
    wf4.collapse();
    wf5.collapse();
    // println!("ERRRRRRRRRRRRRRR");

    let mut perlin_matrix = wf1.to_matrix();
    let mut perlin_matrix2 = wf2.to_matrix();
    let mut altitude_matrix = wf3.to_matrix();
    let mut temperature_matrix = wf4.to_matrix();
    let mut humidity_matrix = wf5.to_matrix();

    for i in 0..perlin_matrix.len()
    { 
        for j in 0..perlin_matrix.len()
        {
           
            println!("{}",perlin_matrix[i][j]);
        }
    }

    apply_biomes(&mut perlin_matrix,&altitude_matrix,&temperature_matrix,&humidity_matrix);
    

    let mut image = Image::gen_image_color(w as i32, h as i32, Color::WHITE);

    for (i, row) in perlin_matrix.iter().enumerate() 
    {
        for (j, &val) in row.iter().enumerate() 
        {
            let color = match val 
            {
                -2.0 => Color::BLUE,
                -1.0 => Color::DARKBLUE,
                2.0 => Color::GOLD,
                3.0 => Color::GRAY,
                4.0 => Color::LIGHTGRAY, 
                5.0 => Color::WHITE, 
                6.0 => Color::YELLOW,
                7.0 => Color::DARKGREEN,
                8.0 => Color::PURPLE,
                -5.0 => Color::BLACK,
                10.0 => Color::RED,
                11.0 => Color::ORANGE,
                12.0 => Color::BLACK,
                _ if val >= 0.0 && val < DARK_FOREST_THRESHOLD => Color::GREEN,
                _ if val >= DARK_FOREST_THRESHOLD => Color::DARKGREEN,
                _ => Color::DARKGREEN,
            };
            image.draw_pixel(j as i32, i as i32, color);
        }
    }
    export_image(&image, "carte.png");


    while !rl.window_should_close() 
    {
        let mut d = rl.begin_drawing(&thread);
        
        d.clear_background(Color::WHITE);
        

        for (i, row) in perlin_matrix.iter().enumerate() 
        {
            for (j, &val) in row.iter().enumerate() 
            {
                let color = match val {
                    -2.0 => Color::BLUE, 
                    -1.0 => Color::DARKBLUE,
                    2.0 => Color::GOLD,
                    3.0 => Color::GRAY, 
                    4.0 => Color::LIGHTGRAY, 
                    5.0 => Color::WHITE, 
                    6.0 => Color::YELLOW,
                    7.0 => Color::DARKGREEN,
                    8.0 => Color::PURPLE,
                    -5.0 => Color::BLACK,
                    10.0 => Color::RED,
                    11.0 => Color::ORANGE,
                    12.0 => Color::BLACK,
                    _ if val >= 0.0 && val < DARK_FOREST_THRESHOLD => Color::GREEN, 
                    _ if val >= DARK_FOREST_THRESHOLD => Color::DARKGREEN, 
                    _ => Color::DARKGREEN,
                };
                if color == Color::BLUE || color == Color::DARKBLUE 
                {
                    let mut color = color;
                    color.a = 200;
                    img_water.draw_rectangle(j as i32, i as i32, 1, 1, color);
                }
                img_texture.draw_rectangle(j as i32 * 1, i as i32 * 1, 1, 1, color);
                d.draw_rectangle(j as i32 * 1, i as i32 * 1, 1, 1, color);
            }
        }
    }

    altitude_matrix
}



fn export_image(image: &Image, file_path: &str) 
{
    let c_file_path = std::ffi::CString::new(file_path).expect("CString::new failed");
    let ffi_image = raylib::ffi::Image 
    {
        data: image.data,
        width: image.width,
        height: image.height,
        mipmaps: image.mipmaps,
        format: image.format,
    };
    unsafe 
    {
        ffi::ExportImage(ffi_image, c_file_path.as_ptr());
    }
}

trait ExportImage 
{
    fn export(&self, path: &Path);
}

impl ExportImage for Image 
{
    fn export(&self, path: &Path) 
    {
        let c_file_path = std::ffi::CString::new(path.to_str().unwrap()).expect("CString::new failed");
        let ffi_image = raylib::ffi::Image 
        {
            data: self.data,
            width: self.width,
            height: self.height,
            mipmaps: self.mipmaps,
            format: self.format,
        };
        unsafe 
        {
            ffi::ExportImage(ffi_image, c_file_path.as_ptr());
        }
    }
}

