use raylib::prelude::*;

pub fn display3D(img: &Image, water: &Image) {
    let (mut rl, thread): (RaylibHandle, RaylibThread) = raylib::init()
        .size(800, 600)
        .title("3D display")
        .vsync()
        .build();

    let mut camera: Camera3D = Camera3D::perspective(
        Vector3::new(410.0, 10.0, 310.0),
        Vector3::forward(),
        Vector3::up(),
        60.0,
    );

    let mut mesh = unsafe {
        Mesh::gen_mesh_plane(&thread, 400.0, 300.0, 1, 1).make_weak()
    };

    let mut mesh_water = unsafe {
        Mesh::gen_mesh_plane(&thread, 400.0, 300.0, 1, 1).make_weak()
    };

    let mut model_water: &mut Model =
        &mut rl.load_model_from_mesh(&thread, mesh_water).unwrap();
    {
        let materials = model_water.materials_mut();
        let mat = &mut materials[0];
        let mats = mat.maps_mut();
        let texture = unsafe {
            let mut t = rl.load_texture_from_image(&thread, water).unwrap();
            t.gen_texture_mipmaps();
            t.unwrap()
        };
        mats[MaterialMapIndex::MATERIAL_MAP_ALBEDO as usize].texture = texture;
    }

    let mut model: &mut Model =
        &mut rl.load_model_from_mesh(&thread, mesh).unwrap();
    {
        let materials = model.materials_mut();
        let mat = &mut materials[0];
        let mats = mat.maps_mut();
        let texture = unsafe {
            let mut t = rl.load_texture_from_image(&thread, img).unwrap();
            t.gen_texture_mipmaps();
            t.unwrap()
        };
        mats[MaterialMapIndex::MATERIAL_MAP_ALBEDO as usize].texture = texture;
    }

    rl.set_camera_mode(&camera, CameraMode::CAMERA_THIRD_PERSON);
    rl.set_target_fps(60);

    while !rl.window_should_close() {
        rl.update_camera(&mut camera);

        let mut d: RaylibDrawHandle<'_> = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        {
            let mut d3 = d.begin_mode3D(&camera);
            d3.draw_model(
                &mut *model,
                Vector3::new(0.0, 1.0, 0.0),
                1.0,
                Color::WHITE,
            );
            let mut color_ocean = Color::BLUE;
            color_ocean.a = 128;
            d3.draw_model(
                &mut *model_water,
                Vector3::new(0.0, 10.0, 0.0),
                1.0,
                color_ocean,
            );
        }
    }
}
