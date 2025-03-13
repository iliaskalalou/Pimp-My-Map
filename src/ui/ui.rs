use crate::display::build_2D::*;
use crate::maths::*;
use crate::multimap::*;
// use crate::perlin_noise;
// use crate::voxels::*;
use crate::obj_export::export_obj::create_3d_terrain;
use crate::widgets::coloration::*;
use crate::widgets::dsquare::*;
use crate::widgets::map::*;
use crate::widgets::perlin::*;
use crate::widgets::previewer::*;
use crate::widgets::widget_chooser::*;
use crate::widgets::widget_io::*;
use crate::widgets::*;

use raylib::core::texture::*;
use raylib::math::Rectangle;
use raylib::math::Vector2;
use raylib::prelude::*;
use std::collections::HashMap;
use std::ffi::CString;

use super::voxels::VoxelMap;

pub const WINDOW_MIN_WIDTH: f32 = 940.0;
pub const WINDOW_MIN_HEIGHT: f32 = 680.0;
pub const BG_COLOR: Color = Color::WHITE;
pub const SELECTION_OFFSET: f32 = 5.0;
pub const ZOOM_INCREMENT: f32 = 0.125;
pub const WORK_ZONE_WIDTH: f32 = 2000.0;
pub const WORK_ZONE_HEIGHT: f32 = 2000.0;
pub const PLUG_RADIUS: f32 = 5.0;
pub const LINK_THICK: f32 = 2.5;
pub const LINK_COLOR: Color = Color::RED;
pub const SELECTION_COLOR: Color = Color::RED;
pub const MOVING_SELECTION_COLOR: Color = Color::ORANGE;
pub const MOUSE_OVERLAY: Color = Color {
    r: 102,
    g: 192,
    b: 255,
    a: 128,
};
pub const WINDOW_BOX_TITLE_SIZE: f32 = 23.0;

// s for static; r for relative
// by definition static's zoom should be 1.0

#[derive(Copy, Clone, PartialEq)]
pub enum Capture {
    Circle(Circle),
    Rectangle(Rectangle),
}

pub struct Globals {
    pub quit: bool,
    pub workspace_locked: bool,
    pub lclick_locked: bool,
    pub rclick_locked: bool,
    pub dimensions: Vector2,
    pub s_camera: Camera2D,
    pub s_mouse: Vector2,
    pub s_mouse_delta: Vector2,
    pub r_camera: Camera2D,
    pub r_mouse: Vector2,
    pub r_mouse_delta: Vector2,
    pub origin: Vector2,
    pub altmap: Vec<Vec<f64>>,
    pub selection: Vec<Rectangle>,
    pub capture: [Option<Capture>; 2],
    pub capture_anchor: Option<Vector2>,
    pub is_selection_moving: bool,
    pub configuring: Option<Vector2>,
}

impl Default for Globals {
    fn default() -> Self {
        Self {
            quit: false,
            workspace_locked: false,
            lclick_locked: false,
            rclick_locked: false,
            dimensions: Vector2::new(WINDOW_MIN_WIDTH, WINDOW_MIN_HEIGHT),
            s_camera: Camera2D::default(),
            s_mouse: Vector2::default(),
            s_mouse_delta: Vector2::default(),
            r_camera: Camera2D::default(),
            r_mouse: Vector2::default(),
            r_mouse_delta: Vector2::default(),
            origin: Vector2::default(),
            altmap: Vec::new(),
            selection: Vec::new(),
            capture_anchor: None,
            capture: [None, None],
            is_selection_moving: false,
            configuring: None,
        }
    }
}

pub struct StaticWidgets {
    pub objmap: ObjMap,
    pub chooser: WidgetChooser,
    pub previewer: Previewer,
}

impl Default for StaticWidgets {
    fn default() -> Self {
        Self {
            objmap: ObjMap::new(String::from("ui_map")),
            chooser: WidgetChooser::default(),
            previewer: Previewer::default(),
        }
    }
}

#[derive(Default)]
pub struct UI {
    globals: Globals,
    widgets: StaticWidgets,
}

impl UI {
    fn keycalls(&mut self, dhandle: &mut RaylibDrawHandle) {
        let wheel = dhandle.get_mouse_wheel_move();

        if dhandle.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
            self.set_exit_status();
        }

        if dhandle.is_mouse_button_down(MouseButton::MOUSE_RIGHT_BUTTON) {
            if !self.globals.rclick_locked {
                let mouse_c = self.get_mouse_circle();
                let plug = if let Some(w) = self
                    .widgets
                    .objmap
                    .find_target(&mouse_c)
                    .map(|x| x.as_widget_plugable().unwrap())
                {
                    w.check_plug_collisions(&mouse_c)
                } else {
                    None
                };

                if let Some(plug_pos) = plug {
                    self.globals.rclick_locked = true;
                    self.widgets.objmap.remove_links(plug_pos)
                } else if self.globals.capture[0].is_none() {
                    self.globals.configuring = None;
                    let mut delta = self.globals.s_mouse_delta;
                    delta.scale(-1.0 / self.globals.r_camera.zoom);
                    self.globals.r_camera.target += delta;
                }
            }
        } else if dhandle.is_mouse_button_up(MouseButton::MOUSE_RIGHT_BUTTON) {
            self.globals.rclick_locked = false;
        }

        if dhandle.is_key_down(KeyboardKey::KEY_LEFT) {
            let mut delta = Vector2::new(10.0, 0.0);
            delta.scale(-1.0 / self.globals.r_camera.zoom);
            self.globals.r_camera.target += delta;
        } else if dhandle.is_key_down(KeyboardKey::KEY_UP) {
            let mut delta = Vector2::new(0.0, 10.0);
            delta.scale(-1.0 / self.globals.r_camera.zoom);
            self.globals.r_camera.target += delta;
        } else if dhandle.is_key_down(KeyboardKey::KEY_RIGHT) {
            let mut delta = Vector2::new(-10.0, 0.0);
            delta.scale(-1.0 / self.globals.r_camera.zoom);
            self.globals.r_camera.target += delta;
        } else if dhandle.is_key_down(KeyboardKey::KEY_DOWN) {
            let mut delta = Vector2::new(0.0, -10.0);
            delta.scale(-1.0 / self.globals.r_camera.zoom);
            self.globals.r_camera.target += delta;
        } else if dhandle.is_key_pressed(KeyboardKey::KEY_F1) {
            self.reset_configuring();
            self.widgets.chooser.visible ^= true;
        } else if dhandle.is_key_pressed(KeyboardKey::KEY_F2) {
            self.widgets.previewer.visible ^= true;
        } else if dhandle.is_key_pressed(KeyboardKey::KEY_F3) {
            self.export_image();
        } else if dhandle.is_key_pressed(KeyboardKey::KEY_F4) {
            self.export_object();
        } else if dhandle.is_key_pressed(KeyboardKey::KEY_ONE) {
            self.widgets.chooser.selected_index = 1;
        } else if dhandle.is_key_pressed(KeyboardKey::KEY_TWO) {
            self.widgets.chooser.selected_index = 2;
        } else if dhandle.is_key_pressed(KeyboardKey::KEY_THREE) {
            self.widgets.chooser.selected_index = 3;
        } else if dhandle.is_key_pressed(KeyboardKey::KEY_FOUR) {
            self.widgets.chooser.selected_index = 4;
        } else if dhandle.is_key_pressed(KeyboardKey::KEY_F10) {
            self.widgets.objmap.dump();
        } else if dhandle.is_key_pressed(KeyboardKey::KEY_F11) {
            if let Some(rect) = self.globals.selection.last() {
                println!(
                    "{}",
                    self.widgets
                        .objmap
                        .objs
                        .get(
                            &Vector2::new(
                                rect.x + SELECTION_OFFSET,
                                rect.y + SELECTION_OFFSET
                            )
                            .into()
                        )
                        .unwrap()
                        .as_widget_plugable()
                        .unwrap()
                );
            }
        } else if dhandle.is_key_pressed(KeyboardKey::KEY_C) {
            if let Some(rect) = self.globals.selection.last() {
                let pos = Vector2::new(
                    rect.x + SELECTION_OFFSET,
                    rect.y + SELECTION_OFFSET,
                );
                self.widgets.objmap.remove_widget_links(pos)
            }
        } else if dhandle.is_key_pressed(KeyboardKey::KEY_F5) {
            self.run_generation()
        }

        if wheel != 0.0 {
            self.scale_camera_zoom(wheel);
        }

        if dhandle.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
            if !self.globals.lclick_locked {
                self.lock_click();

                if !self.globals.workspace_locked {
                    if dhandle.is_key_up(KeyboardKey::KEY_LEFT_SHIFT) {
                        self.reset_selection();
                    }

                    if self.select_mouse_target().is_none() {
                        self.reset_capture();
                        self.insert_new_element();
                    } else {
                        self.capture_mouse_target();

                        if let Some(Capture::Rectangle(rect)) =
                            self.globals.capture[0]
                        {
                            if self.globals.capture[1].is_none() {
                                let pos = Vector2::new(
                                    rect.x + SELECTION_OFFSET,
                                    rect.y + SELECTION_OFFSET,
                                );

                                self.globals.configuring = Some(pos);
                            } else {
                                self.reset_configuring();
                            }
                        } else {
                            self.reset_configuring();
                        }

                        ()
                    }

                    ()
                }

                if dhandle.is_key_up(KeyboardKey::KEY_LEFT_CONTROL) {
                    self.reset_chooser_idx();
                }
            } else if self.globals.capture[0].is_some()
                && self.globals.capture[1].is_none()
            {
                let rect = if let Some(Capture::Rectangle(rect)) =
                    self.globals.capture[0]
                {
                    rect
                } else {
                    unreachable!()
                };

                let pos = Vector2::new(rect.x, rect.y);

                if self.globals.is_selection_moving {
                    self.widgets
                        .objmap
                        .move_widget_by(pos, self.globals.r_mouse_delta);

                    self.globals.capture[0] =
                        Some(Capture::Rectangle(Rectangle::new(
                            pos.x + self.globals.r_mouse_delta.x,
                            pos.y + self.globals.r_mouse_delta.y,
                            rect.width,
                            rect.height,
                        )));

                    let rect = self.globals.selection.last_mut().unwrap();
                    rect.x += self.globals.r_mouse_delta.x;
                    rect.y += self.globals.r_mouse_delta.y;

                    self.globals.configuring = Some(self.globals.r_mouse);
                } else {
                    self.globals.is_selection_moving = true;
                }
            }
        } else if dhandle.is_mouse_button_up(MouseButton::MOUSE_LEFT_BUTTON) {
            if self.globals.lclick_locked {
                self.unlock_click();
                self.globals.is_selection_moving = false;
                self.link_mouse_target();
                self.reset_capture();
            }

            ()
        }

        if dhandle.is_key_down(KeyboardKey::KEY_LEFT_CONTROL)
            && dhandle.is_key_pressed(KeyboardKey::KEY_R)
        {
            self.reset_workspace();
        }

        if dhandle.is_key_pressed(KeyboardKey::KEY_BACKSPACE) {
            self.delete_selection();
        }
    }

    fn export_object(&self) {
        let dims = self.widgets.previewer.voxmap.dims;
        let _ = create_3d_terrain(
            dims.x as usize,
            dims.y as usize,
            self.globals.altmap.clone(),
            "./output_3d.obj",
            256.0,
        );

        println!("Exporting to 3D... DONE");
    }

    fn export_image(&self) {
        let dims = self.widgets.previewer.voxmap.dims;

        let mut image =
            Image::gen_image_color(dims.x as i32, dims.y as i32, Color::WHITE);

        self.widgets.previewer.voxmap.voxels.iter().for_each(|x| {
            let color = match x.value {
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
                _ if x.value >= 0.0 && x.value < DARK_FOREST_THRESHOLD => {
                    Color::GREEN
                }
                _ if x.value >= DARK_FOREST_THRESHOLD => Color::DARKGREEN,
                _ => Color::DARKGREEN,
            };
            image.draw_pixel(x.coords.x as i32, x.coords.y as i32, color);
        });

        // export_image(&image, "carte.png");
        image.export_image("output.png");
        println!("Exporting to PNG... DONE");
    }

    fn reset_chooser_idx(&mut self) {
        self.widgets.chooser.reset_index();
    }

    fn reset_configuring(&mut self) {
        self.globals.configuring = None;
        self.widgets.chooser.ready();
    }

    fn lock_click(&mut self) {
        self.globals.lclick_locked = true;
    }

    fn unlock_click(&mut self) {
        self.globals.lclick_locked = false;
    }

    fn set_exit_status(&mut self) {
        if !self.globals.selection.is_empty() {
            self.globals.selection = Vec::new();
        }

        if self.widgets.previewer.is_visible() {
            self.widgets.previewer.hide();
        } else if self.globals.configuring.is_some() {
            self.reset_configuring();
        } else if self.widgets.chooser.is_visible() {
            self.widgets.chooser.hide();
        } else {
            self.globals.quit = true;
        }
    }

    fn scale_camera_zoom(&mut self, wheel: f32) {
        self.globals.r_camera.zoom += wheel * ZOOM_INCREMENT;
        self.globals.r_camera.zoom =
            self.globals.r_camera.zoom.clamp(ZOOM_INCREMENT, 2.0);
    }

    fn reset_workspace(&mut self) {
        if self.widgets.objmap.locked {
            return;
        }

        // self.globals.plugmap.clear();
        self.widgets.objmap.objs.clear();
        self.reset_selection();
        self.reset_configuring();
        self.widgets.previewer.voxmap = VoxelMap::default();
    }

    fn reset_selection(&mut self) {
        self.globals.selection.clear();
        self.globals.is_selection_moving = false;
    }

    fn reset_capture(&mut self) {
        self.globals.capture[0] = None;
        self.globals.capture[1] = None;
    }

    fn add_to_selection(&mut self, target: Rectangle) {
        self.globals.selection.push(target);
    }

    fn get_mouse_circle(&self) -> Circle {
        Circle::new(
            self.globals.r_mouse,
            SELECTION_OFFSET * self.globals.r_camera.zoom,
        )
    }

    fn delete_selection(&mut self) {
        if self.widgets.objmap.locked {
            return;
        }

        while let Some(rect) = self.globals.selection.pop() {
            let pos = Vector2::new(
                rect.x + SELECTION_OFFSET,
                rect.y + SELECTION_OFFSET,
            );

            self.widgets.objmap.remove_widget(pos);
        }

        self.globals.is_selection_moving = false;
        self.globals.configuring = None;
    }

    fn link_mouse_target(&mut self) {
        let mouse_c = self.get_mouse_circle();

        if let Some(v) = self.widgets.objmap.find_target(&mouse_c) {
            let p = v.as_widget_plugable().unwrap();
            let col = p.check_plug_collisions(&mouse_c);

            if let Some(Capture::Circle(from)) = self.globals.capture[1] {
                self.reset_capture();
                self.reset_configuring();

                if let Some(to) = col {
                    self.widgets.objmap.link_widgets(from.pos, to);
                }
            }
        }
    }

    fn configure_target(
        &mut self,
        pos: Vector2,
        handle: &mut RaylibMode2D<'_, RaylibDrawHandle>,
    ) {
        let circle = Circle::new(pos, SELECTION_OFFSET);
        let widget = self.widgets.objmap.find_target(&circle).unwrap();
        let mut sig = WidgetSignal::None;
        let pos = widget.get_pos();

        if let Some(widget) = widget.as_widget_configurable_mut() {
            self.widgets.chooser.unready();
            sig = widget.configure(handle, &mut self.globals).clone();
        }

        match sig {
            WidgetSignal::IoSwap(itoadd, otoadd, todelete) => {
                todelete.into_iter().for_each(|x| {
                    self.widgets.objmap.remove_links(x);
                    self.widgets.objmap.objs.remove_alias(&x.into());
                });

                self.widgets
                    .objmap
                    .objs
                    .get_mut(&pos.into())
                    .unwrap()
                    .as_widget_plugable_mut()
                    .unwrap()
                    .get_iplug_mut()
                    .inputs
                    .clear();

                self.widgets
                    .objmap
                    .objs
                    .get_mut(&pos.into())
                    .unwrap()
                    .as_widget_plugable_mut()
                    .unwrap()
                    .get_iplug_mut()
                    .outputs
                    .clear();

                itoadd.into_iter().for_each(|x| {
                    self.widgets.objmap.objs.alias(&pos.into(), x.into());
                    self.widgets
                        .objmap
                        .objs
                        .get_mut(&pos.into())
                        .unwrap()
                        .as_widget_plugable_mut()
                        .unwrap()
                        .get_iplug_mut()
                        .inputs
                        .insert(x.into(), Vec::new());
                });

                otoadd.into_iter().for_each(|x| {
                    self.widgets.objmap.objs.alias(&pos.into(), x.into());
                    self.widgets
                        .objmap
                        .objs
                        .get_mut(&pos.into())
                        .unwrap()
                        .as_widget_plugable_mut()
                        .unwrap()
                        .get_iplug_mut()
                        .outputs
                        .insert(x.into(), Vec::new());
                })
            }
            WidgetSignal::None => {}
            _ => {}
        }
    }

    fn capture_mouse_target(&mut self) {
        let mouse_c = self.get_mouse_circle();
        let rect = self.globals.selection.last().unwrap();
        let pos =
            Vector2::new(rect.x + SELECTION_OFFSET, rect.y + SELECTION_OFFSET);

        if let Some(v) = self
            .widgets
            .objmap
            .objs
            .get(&pos.into())
            .unwrap()
            .as_widget_collidable()
        {
            let rect = v.get_rect();
            let p = v.as_widget_plugable().unwrap();

            if let Some(c) = p.check_plug_collisions(&mouse_c) {
                self.globals.capture[1] =
                    Some(Capture::Circle(Circle::new(c, PLUG_RADIUS)));
            }

            self.globals.capture[0] = Some(Capture::Rectangle(rect));
        }
    }

    fn select_mouse_target(&mut self) -> Option<Vector2> {
        let mouse_c = self.get_mouse_circle();

        match self.widgets.objmap.find_target(&mouse_c) {
            Some(v) => {
                let rect = v.get_rect();
                self.widgets.chooser.reset_index();

                self.add_to_selection(Rectangle::new(
                    rect.x - SELECTION_OFFSET,
                    rect.y - SELECTION_OFFSET,
                    rect.width + SELECTION_OFFSET * 2.0,
                    rect.height + SELECTION_OFFSET * 2.0,
                ));

                Some(Vector2::new(rect.x, rect.y))
            }
            _ => None,
        }
    }

    fn create_new_object(&mut self) -> Option<Box<dyn Widget>> {
        match self.widgets.chooser.get_widgettype() {
            WidgetType::Io => {
                Some(Box::new(WidgetIO::create(&mut self.globals)))
            }
            WidgetType::Perlin => {
                let perlin =
                    Perlin::create(&self.widgets.objmap, &mut self.globals);

                Some(Box::new(perlin))
            }
            WidgetType::Dsquare => Some(Box::new(Dsquare::create(
                &self.widgets.objmap,
                &mut self.globals,
            ))),
            WidgetType::Coloration => Some(Box::new(Coloration::create(
                &self.widgets.objmap,
                &mut self.globals,
            ))),
            WidgetType::None => None,
        }
    }

    fn insert_new_element(&mut self) {
        if self.widgets.objmap.locked {
            return;
        }

        let new_element = self.create_new_object();

        match new_element {
            Some(widget) => {
                if let Some(obj) = widget.as_widget_collidable() {
                    let rect = obj.get_rect();
                    let rect = Rectangle::new(
                        rect.x - SELECTION_OFFSET,
                        rect.y - SELECTION_OFFSET,
                        rect.width + SELECTION_OFFSET * 2.0,
                        rect.height + SELECTION_OFFSET * 2.0,
                    );

                    if !self.widgets.objmap.any_rect_colliding(rect) {
                        let pos: Vec2u = self.globals.r_mouse.into();

                        let plugs_to_alias: Option<Vec<Vec2u>> = widget
                            .as_widget_plugable()
                            .map(|pl| pl.plug_pos().copied().collect());

                        self.widgets.objmap.objs.insert(pos, widget);

                        if let Some(vec) = plugs_to_alias {
                            vec.into_iter().for_each(|x| {
                                self.widgets.objmap.objs.alias(&pos, x);
                            });
                        }

                        self.globals.selection.push(rect);
                        self.globals.configuring = Some(pos.into());
                    }
                }
            }
            _ => {
                self.globals.selection = Vec::new();
                self.reset_configuring();
            }
        }
    }

    pub fn dfs_traversal(&mut self, start: Index) -> Option<String> {
        let mut stack: Vec<Index> = Vec::new();
        let order = self.widgets.objmap.objs.order();

        if order == 1 {
            return Some(String::from("Missing Output"));
        }

        let mut mark = HashMap::<Index, bool>::with_capacity(order);

        mark.insert(start, true);
        stack.push(start);

        'dfs: while let Some(elt) = stack.pop() {
            if self
                .widgets
                .objmap
                .get_prev_widgets(elt)
                .unwrap()
                .iter()
                .all(|x| {
                    let t = mark.get(x);
                    t.is_some() && *t.unwrap()
                })
            {
                for adj in self.widgets.objmap.get_next_widgets(elt).unwrap() {
                    let b = mark.get(&adj);
                    if b.is_none() || !*b.unwrap() {
                        stack.push(adj);
                    }
                }

                let wigdet =
                    self.widgets.objmap.objs.data_get_mut(&elt).unwrap();

                match wigdet.get_type() {
                    WidgetType::Perlin | WidgetType::Dsquare => {
                        let widget = wigdet.as_widget_plugable_mut().unwrap();
                        widget.apply(
                            &mut self.widgets.previewer.voxmap,
                            &mut self.globals,
                        );
                        mark.insert(elt, true);
                    }
                    WidgetType::Coloration => {
                        if let Some(v) =
                            self.widgets.objmap.get_prev_widgets(elt)
                        {
                            if v.len() == 1 {
                                mark.insert(elt, true);

                                let widget = self
                                    .widgets
                                    .objmap
                                    .objs
                                    .data_get_mut(&elt)
                                    .unwrap()
                                    .as_widget_plugable_mut()
                                    .unwrap();

                                widget.apply(
                                    &mut self.widgets.previewer.voxmap,
                                    &mut self.globals,
                                );

                                break 'dfs;
                            }
                        }
                    }
                    WidgetType::Io => {
                        if let Some(v) =
                            self.widgets.objmap.get_prev_widgets(elt)
                        {
                            if !v.is_empty() {
                                mark.insert(elt, true);
                                break 'dfs;
                            }
                        }
                    }
                    WidgetType::None => {}
                }
            }
        }

        if mark.len() < order || !mark.values().all(|x| *x) {
            return Some(String::from("Some plugs aren't connected"));
        }

        None
    }

    pub fn run_generation(&mut self) {
        let blocks = self.widgets.objmap.objs.data();

        let start: Vec<Index> = blocks
            .filter(|(_, widget)| widget.as_widget_plugable().is_some())
            .filter(|(_, widget)| {
                widget.get_type() == WidgetType::Io
                    && widget.as_widget_plugable().unwrap().outputs().count()
                        != 0
                    && widget.as_widget_plugable().unwrap().inputs().count()
                        == 0
            })
            .map(|(x, _)| x)
            .copied()
            .collect();

        if start.is_empty() {
            eprintln!("Couldn't find an entry point");
            return;
        } else if start.len() != 1 {
            eprintln!("Cannot handle multiple entry point");
            return;
        }

        eprint!("Generating... ");

        if let Some(msg) = self.dfs_traversal(*start.first().unwrap()) {
            eprintln!("FAILED");
            eprintln!("ERROR: {}", msg);
            return;
        }

        eprintln!("DONE");
    }
}

pub fn main() {
    // core::logging::set_trace_log(ffi::TraceLogLevel::LOG_WARNING);
    set_trace_log(ffi::TraceLogLevel::LOG_WARNING);

    let (mut rl, rl_thread) = init()
        .size(WINDOW_MIN_WIDTH as i32, WINDOW_MIN_HEIGHT as i32)
        .resizable()
        .title("Raylib Widgets")
        .build();

    rl.set_window_min_size(WINDOW_MIN_WIDTH as i32, WINDOW_MIN_HEIGHT as i32);
    rl.set_target_fps(120);
    rl.set_exit_key(None);

    let mut ui = UI::default();

    // let mut globals.selection = Rectangle::EMPTY;

    // let mut mouse = rl.begin_drawing(&rl_thread).get_mouse_position();
    ui.globals.origin = ui.globals.dimensions / 2.0;

    // let mut camera = Camera2D::default();
    ui.globals.s_camera.zoom = 1.0;
    ui.globals.r_camera.zoom = 1.0;
    // let mut state = 0;
    // let mut scroll_idx = 1;

    // let mut scroll = Vector2::zero();

    while !rl.window_should_close() && !ui.globals.quit {
        ui.widgets.previewer.render_to_texture(&mut rl, &rl_thread);

        let mut dhandle = rl.begin_drawing(&rl_thread);

        if dhandle.is_window_resized() {
            ui.globals.dimensions = Vector2::new(
                dhandle.get_screen_width() as f32,
                dhandle.get_screen_height() as f32,
            );

            ui.globals.origin = ui.globals.dimensions / 2.0;
            ui.widgets.chooser.rect.height = ui.globals.dimensions.y;
        }

        ui.keycalls(&mut dhandle);

        let ns_mouse = dhandle.get_mouse_position();
        let nr_mouse =
            dhandle.get_screen_to_world2D(ns_mouse, ui.globals.r_camera);

        ui.globals.s_mouse_delta = ns_mouse - ui.globals.s_mouse;
        ui.globals.r_mouse_delta = nr_mouse - ui.globals.r_mouse;

        let mouse_world_pos = dhandle
            .get_screen_to_world2D(ui.globals.origin, ui.globals.r_camera);

        ui.globals.r_camera.target = mouse_world_pos;
        ui.globals.r_camera.offset = ui.globals.origin;
        ui.globals.r_camera.target =
            ui.globals.r_camera.target.clamp(0.0, 2000.0);

        // ui.widgets.objmap.set_state(ui.widgets.chooser.check_point_collision(ui.globals.r_mouse));

        //////////////////////////////// DRAWING PHASE ////////////////////////////////

        dhandle.clear_background(BG_COLOR);
        let window_dims = Vector2::new(
            dhandle.get_screen_width() as f32,
            dhandle.get_screen_height() as f32,
        );

        let mut r_dhandle = dhandle.begin_mode2D(&ui.globals.r_camera);

        r_dhandle.draw_rectangle_lines_ex(
            Rectangle::new(-10.0, -10.0, WORK_ZONE_WIDTH, WORK_ZONE_HEIGHT),
            10,
            Color::GRAY,
        );

        let _ = r_dhandle.gui_grid(
            Rectangle::new(
                0.0,
                0.0,
                WORK_ZONE_WIDTH - 20.0,
                WORK_ZONE_HEIGHT - 20.0,
            ),
            30.0,
            5,
        );

        r_dhandle.draw_circle(100, 100, 50.0, Color::YELLOW);

        let _ = ui.widgets.objmap.call(&mut r_dhandle, &mut ui.globals);

        let selection_color = if ui.globals.is_selection_moving {
            MOVING_SELECTION_COLOR
        } else {
            SELECTION_COLOR
        };

        ui.globals.selection.iter().for_each(|x| {
            r_dhandle.draw_rectangle_lines_ex(x, 2, selection_color);
        });

        if let Some(Capture::Circle(c)) = ui.globals.capture[1] {
            r_dhandle.draw_line_ex(
                ui.globals.r_mouse,
                c.pos,
                LINK_THICK,
                Color::BLUE,
            );
        }

        drop(r_dhandle);

        dhandle.gui_status_bar(
            Rectangle::new(
                window_dims.x - 80.0,
                window_dims.y - 20.0,
                80.0,
                20.0,
            ),
            Some(
                &CString::new(format!(
                    "x:{} y:{}",
                    ui.globals.s_mouse.x, ui.globals.s_mouse.y
                ))
                .expect("CString::new failed"),
            ),
        );

        dhandle.gui_status_bar(
            Rectangle::new(
                window_dims.x - 230.0,
                window_dims.y - 20.0,
                150.0,
                20.0,
            ),
            Some(
                &CString::new(format!(
                    "rx:{} ry:{}",
                    ui.globals.r_mouse.x, ui.globals.r_mouse.y
                ))
                .expect("CString::new failed"),
            ),
        );

        let s_camera = Camera2D {
            zoom: 1.0,
            ..Default::default()
        };
        let mut s_dhandle = dhandle.begin_mode2D(s_camera);

        ui.widgets.chooser.call(&mut s_dhandle, &mut ui.globals);
        ui.widgets.previewer.call(&mut s_dhandle, &mut ui.globals);

        if let Some(pos) = ui.globals.configuring {
            ui.configure_target(pos, &mut s_dhandle);
        }

        drop(s_dhandle);

        ///////////////////////////////////////////////////////////////////////////////

        // ui.mouse = n_mouse;
        ui.globals.s_mouse += ui.globals.s_mouse_delta;
        ui.globals.r_mouse += ui.globals.r_mouse_delta;
    }
}
