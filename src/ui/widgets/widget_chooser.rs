use crate::ui::ui::{SELECTION_OFFSET, WINDOW_MIN_HEIGHT};
use crate::widgets::*;
use raylib::prelude::*;

use std::ffi::CString;

pub struct WidgetChooser {
    pub values: String,
    pub scroll_index: i32,
    pub selected_index: i32,
    pub callback: fn(&mut Self, &WidgetSignal),

    pub rect: Rectangle,
    pub id: String,
    pub ready: bool,
    pub visible: bool,
}

impl WidgetChooser {
    pub fn new(
        rect: Rectangle,
        values: String,
        callback: fn(&mut Self, &WidgetSignal),
        id: String,
    ) -> Self {
        Self {
            rect,
            values,
            visible: true,
            scroll_index: 0,
            selected_index: -1,
            callback,
            ready: true,
            id,
        }
    }

    pub fn reset_index(&mut self) {
        self.scroll_index = 0;
        self.selected_index = 0;
    }
}

impl Default for WidgetChooser {
    fn default() -> Self {
        Self::new(
            Rectangle::new(0.0, 0.0, 150.0, WINDOW_MIN_HEIGHT),
            String::from("None\nIO\nPerlin\nDsquare\nColoration"),
            |obj, res| {
                if let WidgetSignal::Vec(v) = res {
                    if let [WidgetSignal::Bool(b), WidgetSignal::I32(i)] = v[..]
                    {
                        if obj.is_ready() {
                            if b {
                                obj.visible = false;
                            }
                            obj.selected_index = i;
                        }
                    }
                }
            },
            String::from("Selection Menu"),
        )
    }
}

impl WidgetChooser {
    pub fn get_widgettype(&self) -> WidgetType {
        match self.selected_index {
            1 => WidgetType::Io,
            2 => WidgetType::Perlin,
            3 => WidgetType::Dsquare,
            4 => WidgetType::Coloration,
            _ => WidgetType::None,
        }
    }

    pub fn reset_sel_index(&mut self) {
        self.selected_index = 0;
    }
}

impl WidgetRectangle for WidgetChooser {
    fn get_rect(&self) -> Rectangle {
        self.rect
    }

    fn set_rect(&mut self, rect: Rectangle) {
        self.rect = rect
    }
}

impl WidgetCollidable for WidgetChooser {
    fn check_rect_collision(&self, rect: Rectangle) -> bool {
        self.visible && self.get_rect().check_collision_recs(&rect)
    }

    fn check_circ_collision(&self, circle: &Circle) -> bool {
        self.visible
            && self
                .get_rect()
                .check_collision_circle_rec(circle.pos, circle.rad)
    }

    fn check_point_collision(&self, point: Vector2) -> bool {
        self.visible && self.get_rect().check_collision_point_rec(point)
    }
}

impl Widget for WidgetChooser {
    fn render(
        &mut self,
        handle: &mut RaylibMode2D<'_, RaylibDrawHandle>,
        globals: &mut Globals,
    ) -> WidgetSignal {
        // res = [gui_window_box(), gui_list_view()]
        if self.visible {
            globals.workspace_locked = self
                .rect
                .check_collision_circle_rec(globals.s_mouse, SELECTION_OFFSET);

            let mut res = vec![WidgetSignal::None, WidgetSignal::None];

            res[0] = WidgetSignal::Bool(handle.gui_window_box(
                self.rect,
                Some(&CString::new(&*self.id).expect("CString::new failed")),
            ));

            res[1] = WidgetSignal::I32(handle.gui_list_view(
                Rectangle::new(
                    self.rect.x + 0.0,
                    self.rect.y + 23.0,
                    self.rect.width,
                    self.rect.height,
                ),
                Some(
                    &CString::new(&*self.values).expect("CString::new failed"),
                ),
                &mut self.scroll_index,
                self.selected_index,
            ));

            WidgetSignal::Vec(res)
        } else {
            WidgetSignal::None
        }
    }

    fn call(
        &mut self,
        handle: &mut RaylibMode2D<'_, RaylibDrawHandle>,
        globals: &mut Globals,
    ) -> WidgetSignal {
        let res = self.render(handle, globals);
        if self.ready {
            (self.callback)(self, &res);
        }
        res
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
        self.visible = state;
    }
    fn show(&mut self) {
        self.visible = true;
    }
    fn hide(&mut self) {
        self.visible = false;
    }
    fn toggle_visible(&mut self) {
        self.visible ^= true;
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
}
