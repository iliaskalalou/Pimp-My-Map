mod cell;
mod sketch;
mod tile;
use cell::Cell;
use std::env::args;

fn main()
{
    let arg_list: Vec<String> = args().collect();
    match arg_list.as_slice() {
        [dir_path, dim] => {
            let (mut rl, thread) = raylib::init()
                .title("Wave function Collapse")
                .vsync()
                .build();
            let mut tiles_images = sketch::preload(dir_path, &thread, &mut rl);
            while !rl.window_should_close() {}
        }
        _ => panic!("Usage:\n\t./wfc <dir_path> <grid_dimension>"),
    }
}
