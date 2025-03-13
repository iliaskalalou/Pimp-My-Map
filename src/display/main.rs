use noise::{NoiseFn, Perlin, Seedable};
use raylib::prelude::*;

mod build_2D;
mod display_3D;

fn main()
{
    let mut img: Image = Image::gen_image_color(800, 600, Color::WHITE);
    let mut water: Image = Image::gen_image_color(800, 600, Color::WHITE);
    let mut altitude1 = build_2D::test_2D(&mut img, &mut water);
    //altitude1 = build_2D::test_2D(&mut img, &mut water);
    //altitude1 = build_2D::test_2D(&mut img, &mut water);
    //altitude1 = build_2D::test_2D(&mut img, &mut water);
    println!("TAILLE {}", altitude1.len());
    //display_3D::display3D(&img, &water);

    let mut altitude2 = build_2D::test_2D_Diamond(&mut img, &mut water);
    println!("TAILLE {}", altitude2.len());

    let mut altitude3 = build_2D::test_2D_MIX_Diamond_Perlin(&mut img, &mut water);
    println!("TAILLE {}", altitude3.len());


    //let mut altitude4 = build_2D::test_2D_WFC(&mut img, &mut water);
    //println!("TAILLE {}", altitude4.len()); 
}

