// use crate::voxels::*;
use crate::ui::ui::{WINDOW_BOX_TITLE_SIZE, WINDOW_MIN_HEIGHT};
use crate::widgets::*;

use raylib::prelude::*;
use std::ffi::CString;

const IO_BLOCK_COLOR: Color = Color::GRAY;
const IO_BLOCK_BORDER_COLOR: Color = Color::DARKGRAY;
const IO_BLOCK_SIZE: f32 = 50.0;
const I_PLUG_OFFSET: Vector2 = Vector2 {
    x: 0.0,
    y: IO_BLOCK_SIZE / 2.0,
};
const O_PLUG_OFFSET: Vector2 = Vector2 {
    x: IO_BLOCK_SIZE,
    y: IO_BLOCK_SIZE / 2.0,
};

pub struct WidgetIO {
    pub i_plug: PlugInterface,
    pub output_mode: bool,

    pub rect: Rectangle,
    pub id: String,
    pub ready: bool,
    pub visible: bool,
}

impl WidgetIO {
    pub fn create(globals: &mut Globals) -> Self {
        let mut res = WidgetIO::default();
        let mouse = globals.r_mouse;

        res.rect =
            Rectangle::new(mouse.x, mouse.y, IO_BLOCK_SIZE, IO_BLOCK_SIZE);
        res.i_plug = PlugInterface::new(mouse);

        let output_plug_pos: Vec2u = (mouse + O_PLUG_OFFSET).into();
        res.i_plug.outputs.insert(output_plug_pos, Vec::new());

        res
    }

    pub fn io_swap_signal(&mut self) -> WidgetSignal {
        let pos = self.get_pos();
        let in_plug = pos + I_PLUG_OFFSET;
        let out_plug = pos + O_PLUG_OFFSET;

        if self.output_mode {
            WidgetSignal::IoSwap(vec![in_plug], vec![], vec![out_plug])
        } else {
            WidgetSignal::IoSwap(vec![], vec![out_plug], vec![in_plug])
        }
    }
}

impl Default for WidgetIO {
    fn default() -> Self {
        Self {
            i_plug: PlugInterface::default(),
            output_mode: true,

            rect: Rectangle::EMPTY,
            id: String::from("IO"),
            ready: false,
            visible: true,
        }
    }
}

impl WidgetRectangle for WidgetIO {
    fn get_rect(&self) -> Rectangle {
        self.rect
    }

    fn set_rect(&mut self, rect: Rectangle) {
        self.rect = rect;
    }
}

impl WidgetCollidable for WidgetIO {}

impl WidgetPlugable for WidgetIO {
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
            return self.i_plug.inputs.insert(new.into(), i);
        }

        if let Some(i) = self.i_plug.outputs.remove(&pos.into()) {
            return self.i_plug.outputs.insert(new.into(), i);
        }

        None
    }

    fn translate_wplugs(&mut self, offset: Vector2) {
        let old_pos = self.get_pos();
        self.translate(offset);
        let new_pos = self.get_pos();

        if self.output_mode {
            self.replace_plug_pos(
                old_pos + O_PLUG_OFFSET,
                new_pos + O_PLUG_OFFSET,
            );
        } else {
            self.replace_plug_pos(
                old_pos + I_PLUG_OFFSET,
                new_pos + I_PLUG_OFFSET,
            );
        }
    }

    fn apply(&mut self, _: &mut VoxelMap, _: &mut Globals) {}
}

impl WidgetConfigurable for WidgetIO {
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

        let new_output_mode = handle.gui_toggle(
            Rectangle::new(5.0, WINDOW_BOX_TITLE_SIZE + 5.0, 140.0, 30.0),
            Some(&CString::new("output mode").expect("CString::new failed")),
            self.output_mode,
        );

        let res = if new_output_mode != self.output_mode {
            self.io_swap_signal()
        } else {
            WidgetSignal::None
        };

        self.output_mode = new_output_mode;

        res
    }
}

impl Widget for WidgetIO {
    fn render(
        &mut self,
        handle: &mut RaylibMode2D<'_, RaylibDrawHandle>,
        globals: &mut Globals,
    ) -> WidgetSignal {
        self.draw_plug_links(handle);

        handle.draw_rectangle_rec(self.rect, IO_BLOCK_COLOR);
        handle.draw_rectangle_lines_ex(self.rect, 2, IO_BLOCK_BORDER_COLOR);

        handle.draw_text(
            &self.id,
            self.rect.x as i32 + 2,
            self.rect.y as i32 + 5,
            15,
            Color::WHITE,
        );

        if self.output_mode {
            self.draw_output_plugs(handle, globals);
        } else {
            self.draw_input_plugs(handle, globals);
        }

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
        WidgetType::Io
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
