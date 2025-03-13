use crate::ui::ui::SELECTION_OFFSET;
use crate::voxels::*;
use crate::widgets::*;
use raylib::prelude::*;

use std::ffi::CString;

pub struct Previewer {
    pub voxmap: VoxelMap,
    pub scroll: Vector2,
    pub callback: fn(&mut Self, &WidgetSignal),
    pub texture: Option<Texture2D>,

    pub rect: Rectangle,
    pub id: String,
    pub ready: bool,
    pub visible: bool,
}

impl Previewer {
    pub fn render_to_texture(
        &mut self,
        handle: &mut RaylibHandle,
        rl_thread: &RaylibThread,
    ) {
        let img = self.voxmap.render_to_img();
        let tex = handle.load_texture_from_image(rl_thread, &img);

        self.texture = match tex {
            Ok(t) => Some(t),
            _ => None,
        }
    }
}

impl Default for Previewer {
    fn default() -> Self {
        Self {
            voxmap: VoxelMap::default(),
            scroll: Vector2::new(-256.0, -256.0),
            callback: |obj, res| {
                if let WidgetSignal::Vec(v) = res {
                    if let [WidgetSignal::Bool(b), WidgetSignal::Vector2(v2)] =
                        v[..]
                    {
                        if b {
                            obj.visible = false;
                        }

                        obj.scroll = v2;
                    }
                }
            },
            texture: None,

            rect: Rectangle::new(250.0, 40.0, 524.0, 547.0),
            id: String::from("Preview Window"),
            ready: true,
            visible: false,
        }
    }
}

impl WidgetRectangle for Previewer {
    fn get_rect(&self) -> Rectangle {
        self.rect
    }

    fn set_rect(&mut self, rect: Rectangle) {
        self.rect = rect;
    }
}

impl WidgetCollidable for Previewer {}

impl Widget for Previewer {
    fn render(
        &mut self,
        handle: &mut RaylibMode2D<'_, RaylibDrawHandle>,
        globals: &mut Globals,
    ) -> WidgetSignal {
        if self.visible {
            globals.workspace_locked = self
                .rect
                .check_collision_circle_rec(globals.r_mouse, SELECTION_OFFSET);

            let mut res = vec![WidgetSignal::None, WidgetSignal::None];

            let scroll_panel_rect = Rectangle::new(
                self.rect.x,
                self.rect.y + 23.0,
                self.rect.width - 1.0,
                self.rect.height - 24.0,
            );

            let img_rect = Rectangle::new(
                self.rect.x,
                self.rect.y,
                self.voxmap.dims.x,
                self.voxmap.dims.y,
            );

            if img_rect.width < scroll_panel_rect.width {
                self.rect.width = img_rect.width;
            }

            if img_rect.height < scroll_panel_rect.height {
                self.rect.height = img_rect.height;
            }

            res[0] = WidgetSignal::Bool(handle.gui_window_box(
                self.rect,
                Some(&CString::new(&*self.id).expect("CString::new failed")),
            ));

            let (bounds, offrect, scroll) = if img_rect.width
                < scroll_panel_rect.width
                || img_rect.height < scroll_panel_rect.height
            {
                let (bounds, scroll) = handle.gui_scroll_panel(
                    scroll_panel_rect,
                    img_rect,
                    self.scroll,
                );

                let offrect = Rectangle::new(
                    -scroll.x + 1.0,
                    -scroll.y + 1.0,
                    bounds.width,
                    bounds.height,
                );

                (bounds, offrect, scroll)
            } else {
                (
                    scroll_panel_rect,
                    Rectangle::new(0.0, 0.0, self.rect.width, self.rect.height),
                    Vector2::default(),
                )
            };

            res[1] = WidgetSignal::Vector2(scroll);

            if let Some(tex) = &self.texture {
                handle.draw_texture_rec(
                    tex,
                    offrect,
                    Vector2::new(bounds.x, bounds.y),
                    Color::WHITE,
                );
            }

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
