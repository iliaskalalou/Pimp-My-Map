use crate::diamond_square::diamond_square::{self, *};
use crate::ui::ui::WINDOW_BOX_TITLE_SIZE;
// use crate::voxels::*;
use crate::widgets::*;

use raylib::prelude::*;
use std::ffi::CString;

const DSQUARE_BLOCK_COLOR: Color = Color::BLUE;
const DSQUARE_BLOCK_BORDER_COLOR: Color = Color::DARKBLUE;
const DSQUARE_BLOCK_SIZE: f32 = 65.0;
const I_PLUG_OFFSET: Vector2 = Vector2 {
    x: 0.0,
    y: DSQUARE_BLOCK_SIZE / 2.0,
};
const O_PLUG_OFFSET: Vector2 = Vector2 {
    x: DSQUARE_BLOCK_SIZE,
    y: DSQUARE_BLOCK_SIZE / 2.0,
};

pub struct Dsquare {
    pub opts: DsquareOpts,

    pub i_plug: PlugInterface,

    pub rect: Rectangle,
    pub id: String,
    pub ready: bool,
    pub visible: bool,
}

impl Dsquare {
    pub fn create(objmap: &ObjMap, globals: &mut Globals) -> Self {
        let mut res = Dsquare::default();
        let mouse = globals.r_mouse;

        res.rect = Rectangle::new(
            mouse.x,
            mouse.y,
            DSQUARE_BLOCK_SIZE,
            DSQUARE_BLOCK_SIZE,
        );
        res.id = format!("{}_{}", res.id, objmap.objs.unique_keys_count());

        let input = mouse + I_PLUG_OFFSET;
        let output = mouse + O_PLUG_OFFSET;

        res.i_plug = PlugInterface::new(mouse);

        res.i_plug.inputs.insert(input.into(), Vec::new());
        res.i_plug.outputs.insert(output.into(), Vec::new());

        res
    }
}

impl Default for Dsquare {
    fn default() -> Self {
        Self {
            opts: DsquareOpts::default(),
            i_plug: PlugInterface::default(),

            rect: Rectangle::EMPTY,
            id: String::from("Dsquare"),
            ready: false,
            visible: true,
        }
    }
}

impl WidgetRectangle for Dsquare {
    fn get_rect(&self) -> Rectangle {
        self.rect
    }

    fn set_rect(&mut self, rect: Rectangle) {
        self.rect = rect;
    }
}

impl WidgetCollidable for Dsquare {}

impl WidgetPlugable for Dsquare {
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
        self.replace_plug_pos(old_pos + O_PLUG_OFFSET, new_pos + O_PLUG_OFFSET);
    }

    fn apply(&mut self, voxmap: &mut VoxelMap, _: &mut Globals) {
        let mut base =
            vec![vec![0.0; voxmap.dims.x as usize]; voxmap.dims.y as usize];

        voxmap.voxels.iter().for_each(|x| {
            base[x.coords.x as usize][x.coords.y as usize] = x.value;
        });

        let res = diamond_square::diamond_square(&self.opts, &base);

        voxmap.voxels.iter_mut().for_each(|x| {
            let cx = x.coords.x as usize;
            let cy = x.coords.y as usize;
            x.value = res[cx][cy];
        });
    }
}

impl WidgetConfigurable for Dsquare {
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

        handle.gui_group_box(
            Rectangle::new(15.0, WINDOW_BOX_TITLE_SIZE + 10.0, 120.0, 20.0),
            Some(
                &CString::new(format!("Roughness: {}", self.opts.roughness))
                    .expect("CString::new failed"),
            ),
        );

        self.opts.roughness = ((handle.gui_slider_bar(
            Rectangle::new(15.0, WINDOW_BOX_TITLE_SIZE + 17.0, 120.0, 20.0),
            Some(&CString::new("0").expect("CString::new failed")),
            Some(&CString::new("1").expect("CString::new failed")),
            self.opts.roughness as f32,
            0.0,
            1.0,
        ) * 1000.0)
            .ceil()
            / 1000.0) as f64;

        if handle.gui_button(
            Rectangle::new(5.0, globals.dimensions.y - 35.0, 140.0, 30.0),
            Some(&CString::new("Reset").expect("CString::new failed")),
        ) {
            self.opts = DsquareOpts::default();
        }

        WidgetSignal::None
    }
}

impl Widget for Dsquare {
    fn render(
        &mut self,
        handle: &mut RaylibMode2D<'_, RaylibDrawHandle>,
        globals: &mut Globals,
    ) -> WidgetSignal {
        self.draw_plug_links(handle);

        handle.draw_rectangle_rec(self.rect, DSQUARE_BLOCK_COLOR);
        handle.draw_rectangle_lines_ex(
            self.rect,
            2,
            DSQUARE_BLOCK_BORDER_COLOR,
        );

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
        WidgetType::Dsquare
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
