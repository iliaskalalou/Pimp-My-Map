// use crate::diamond_square::diamond_square::{self, *};
use crate::display::build_2D::{
    apply_biomes, diamond_square, generate_perlin_noise_matrix, normalize,
};
use crate::ui::ui::WINDOW_BOX_TITLE_SIZE;
use crate::widgets::*;

use noise::{NoiseFn, Perlin};
use raylib::prelude::*;
use std::ffi::CString;

const COLORATION_BLOCK_COLOR1: Color = Color::PINK;
const COLORATION_BLOCK_COLOR2: Color = Color::PURPLE;
const COLORATION_BLOCK_SIZE: f32 = 100.0;

const I_PLUG_OFFSET: Vector2 = Vector2 {
    x: 0.0,
    y: COLORATION_BLOCK_SIZE / 2.0,
};

pub struct Coloration {
    // pub opts: ColorationOpts,
    pub i_plug: PlugInterface,

    pub rect: Rectangle,
    pub id: String,
    pub ready: bool,
    pub visible: bool,
}

impl Coloration {
    pub fn create(objmap: &ObjMap, globals: &mut Globals) -> Self {
        let mut res = Coloration::default();
        let mouse = globals.r_mouse;

        res.rect = Rectangle::new(
            mouse.x,
            mouse.y,
            COLORATION_BLOCK_SIZE,
            COLORATION_BLOCK_SIZE,
        );
        res.id = format!("{}_{}", res.id, objmap.objs.unique_keys_count());

        let i = mouse + I_PLUG_OFFSET;

        res.i_plug = PlugInterface::new(mouse);

        res.i_plug.inputs.insert(i.into(), Vec::new());

        res
    }
}

impl Default for Coloration {
    fn default() -> Self {
        Self {
            // opts: ColorationOpts::default(),
            i_plug: PlugInterface::default(),

            rect: Rectangle::EMPTY,
            id: String::from("Coloration"),
            ready: false,
            visible: true,
        }
    }
}

impl WidgetRectangle for Coloration {
    fn get_rect(&self) -> Rectangle {
        self.rect
    }

    fn set_rect(&mut self, rect: Rectangle) {
        self.rect = rect;
    }
}

impl WidgetCollidable for Coloration {}

impl WidgetPlugable for Coloration {
    fn get_iplug(&self) -> &PlugInterface {
        &self.i_plug
    }

    fn get_iplug_mut(&mut self) -> &mut PlugInterface {
        &mut self.i_plug
    }

    fn replace_plug_pos(
        &mut self,
        pos: Vector2,
        new: Vector2,
    ) -> Option<Vec<Vector2>> {
        if let Some(i) = self.i_plug.inputs.remove(&pos.into()) {
            if let Some(c) = self.i_plug.inputs.insert(new.into(), i) {
                return Some(c);
            }

            return None;
        }

        if let Some(i) = self.i_plug.outputs.remove(&pos.into()) {
            if let Some(c) = self.i_plug.outputs.insert(new.into(), i) {
                return Some(c);
            }

            return None;
        }

        None
    }

    fn translate_wplugs(&mut self, offset: Vector2) {
        let old_pos = self.get_pos();
        self.translate(offset);
        let new_pos = self.get_pos();
        self.replace_plug_pos(old_pos + I_PLUG_OFFSET, new_pos + I_PLUG_OFFSET);
    }

    fn apply(&mut self, voxmap: &mut VoxelMap, globals: &mut Globals) {
        let mut base =
            vec![vec![0.0; voxmap.dims.x as usize]; voxmap.dims.y as usize];

        voxmap.voxels.iter().for_each(|x| {
            base[x.coords.x as usize][x.coords.y as usize] = x.value;
        });

        globals.altmap = generate_perlin_noise_matrix(
            voxmap.dims.x as usize,
            voxmap.dims.y as usize,
            90.0,
        );

        globals.altmap = diamond_square(9);
        normalize(&mut globals.altmap);
        globals.altmap.pop();
        globals.altmap.iter_mut().for_each(|x| {
            x.pop();
        });

        let tmp_map = generate_perlin_noise_matrix(
            voxmap.dims.x as usize,
            voxmap.dims.y as usize,
            90.0,
        );

        let hum_map = generate_perlin_noise_matrix(
            voxmap.dims.x as usize,
            voxmap.dims.y as usize,
            90.0,
        );

        apply_biomes(&mut base, &globals.altmap, &tmp_map, &hum_map);

        voxmap.voxels.iter_mut().for_each(|x| {
            let cx = x.coords.x as usize;
            let cy = x.coords.y as usize;
            x.value = base[cx][cy];
        });
    }
}

impl WidgetConfigurable for Coloration {
    fn configure(
        &mut self,
        handle: &mut RaylibMode2D<'_, RaylibDrawHandle>,
        globals: &mut Globals,
    ) -> WidgetSignal {
        let panel_rect =
            Rectangle::new(0.0, 0.0, 150.0, globals.dimensions.y + 1.0);

        globals.workspace_locked = panel_rect
            .check_collision_circle_rec(globals.s_mouse, SELECTION_OFFSET);

        handle.gui_window_box(
            panel_rect,
            Some(&CString::new(&*self.id).expect("CString::new failed")),
        );

        WidgetSignal::None
    }
}

impl Widget for Coloration {
    fn render(
        &mut self,
        handle: &mut RaylibMode2D<'_, RaylibDrawHandle>,
        globals: &mut Globals,
    ) -> WidgetSignal {
        self.draw_plug_links(handle);

        handle.draw_rectangle_rec(self.rect, COLORATION_BLOCK_COLOR1);
        handle.draw_rectangle_lines_ex(self.rect, 2, COLORATION_BLOCK_COLOR2);

        handle.draw_text(
            &self.id,
            self.rect.x as i32 + 2,
            self.rect.y as i32 + 5,
            15,
            Color::WHITE,
        );

        self.draw_plugs(handle, globals);

        WidgetSignal::None
    }

    fn call(
        &mut self,
        handle: &mut RaylibMode2D<'_, RaylibDrawHandle>,
        globals: &mut Globals,
    ) -> WidgetSignal {
        self.render(handle, globals)
    }

    fn ready(&mut self) {
        self.ready = true
    }
    fn unready(&mut self) {
        self.ready = false
    }
    fn is_ready(&self) -> bool {
        self.ready
    }

    fn get_type(&self) -> WidgetType {
        WidgetType::Coloration
    }

    fn get_id(&self) -> String {
        self.id.clone()
    }
    fn set_id(&mut self, id: String) {
        self.id = id
    }

    fn is_visible(&self) -> bool {
        self.visible
    }
    fn set_visible(&mut self, state: bool) {
        self.visible = state
    }
    fn show(&mut self) {
        self.visible = true
    }
    fn hide(&mut self) {
        self.visible = false
    }
    fn toggle_visible(&mut self) {
        self.visible ^= true
    }

    fn as_widget_rectangle(&self) -> Option<&dyn WidgetRectangle> {
        Some(self as _)
    }

    fn as_widget_rectangle_mut(&mut self) -> Option<&mut dyn WidgetRectangle> {
        Some(self as _)
    }

    fn as_widget_collidable(&self) -> Option<&dyn WidgetCollidable> {
        Some(self as _)
    }

    fn as_widget_collidable_mut(
        &mut self,
    ) -> Option<&mut dyn WidgetCollidable> {
        Some(self as _)
    }

    fn as_widget_plugable(&self) -> Option<&dyn WidgetPlugable> {
        Some(self as _)
    }

    fn as_widget_plugable_mut(&mut self) -> Option<&mut dyn WidgetPlugable> {
        Some(self as _)
    }

    fn as_widget_configurable(&self) -> Option<&dyn WidgetConfigurable> {
        Some(self as _)
    }

    fn as_widget_configurable_mut(
        &mut self,
    ) -> Option<&mut dyn WidgetConfigurable> {
        Some(self as _)
    }
}
