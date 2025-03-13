use raylib::prelude::*;

const WINDOW_MIN_WIDTH: i32 = 640;
const WINDOW_MIN_HEIGHT: i32 = 480;
const BG_COLOR: Color = Color::WHITE;

const ZOOM_INCREMENT: f32 = 0.125;

pub fn main() {
    let (mut rl, rl_thread) = init()
        .size(WINDOW_MIN_WIDTH, WINDOW_MIN_HEIGHT)
        .resizable()
        .title("Raylib Camera")
        .build();

    rl.set_window_min_size(WINDOW_MIN_WIDTH, WINDOW_MIN_HEIGHT);
    rl.set_target_fps(60);

    let mut mouse = rl.begin_drawing(&rl_thread).get_mouse_position();
    let mut origin = Vector2::new(
        (WINDOW_MIN_WIDTH / 2) as f32,
        (WINDOW_MIN_HEIGHT / 2) as f32,
    );

    let mut camera = Camera2D::default();
    camera.zoom = 1.0;

    while !rl.window_should_close() {
        let mut dhandle = rl.begin_drawing(&rl_thread);
        let n_mouse = dhandle.get_mouse_position();

        if dhandle.is_window_resized() {
            origin.x = (dhandle.get_screen_width() / 2) as f32;
            origin.y = (dhandle.get_screen_height() / 2) as f32;
        }

        // if dhandle.is_mouse_button_pressed(MouseButton::MOUSE_RIGHT_BUTTON) {
        //     dbg!("{}", camera);
        // }

        if dhandle.is_mouse_button_down(MouseButton::MOUSE_RIGHT_BUTTON) {
            let mut delta = n_mouse - mouse;
            delta.scale(-1.0 / camera.zoom);
            camera.target += delta;
        }

        if dhandle.is_key_down(KeyboardKey::KEY_LEFT) {
            let mut delta = Vector2::new(10.0, 0.0);
            delta.scale(-1.0 / camera.zoom);
            camera.target += delta;
        }

        if dhandle.is_key_down(KeyboardKey::KEY_UP) {
            let mut delta = Vector2::new(0.0, 10.0);
            delta.scale(-1.0 / camera.zoom);
            camera.target += delta;
        }

        if dhandle.is_key_down(KeyboardKey::KEY_RIGHT) {
            let mut delta = Vector2::new(-10.0, 0.0);
            delta.scale(-1.0 / camera.zoom);
            camera.target += delta;
        }

        if dhandle.is_key_down(KeyboardKey::KEY_DOWN) {
            let mut delta = Vector2::new(0.0, -10.0);
            delta.scale(-1.0 / camera.zoom);
            camera.target += delta;
        }

        let wheel = dhandle.get_mouse_wheel_move();

        if wheel != 0.0 {
            camera.zoom += wheel * ZOOM_INCREMENT;
            camera.zoom = camera.zoom.clamp(ZOOM_INCREMENT, 2.0);
        }

        let mouse_world_pos = dhandle.get_screen_to_world2D(origin, camera);
        camera.target = mouse_world_pos;
        camera.offset = origin;

        camera.target = camera.target.clamp(0.0, 2000.0);
        dhandle.clear_background(BG_COLOR);

        let mut d = dhandle.begin_mode2D(&camera);

        d.draw_circle(100, 100, 50.0, Color::YELLOW);
        d.draw_rectangle_lines_ex(Rectangle::new(0.0, 0.0, 2000.0, 2000.0), 10, Color::GRAY);

        drop(d);

        dhandle.draw_circle_v(origin, 5.0, Color::RED);
        dhandle.gui_window_box(
            Rectangle::new(0.0, 0.0, 250.0, dhandle.get_screen_height() as f32 + 5.0),
            None,
        );

        mouse = n_mouse;
    }
}
