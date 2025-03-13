// pub mod button;
// pub mod counter;
pub mod coloration;
pub mod dsquare;
pub mod map;
pub mod perlin;
pub mod previewer;
pub mod widget_chooser;
pub mod widget_io;

// use crate::ieef64::Vec2f64;
use crate::maths::{Circle, Vec2u};
use crate::ui::ui::Globals;
use map::*;
// use raylib::core::collision::check_collision_circles;
use raylib::prelude::*;
use std::collections::HashMap;

use super::ui::{
    Capture, LINK_COLOR, LINK_THICK, PLUG_RADIUS, SELECTION_OFFSET,
};
use super::voxels::VoxelMap;

#[derive(Clone)]
pub enum WidgetSignal {
    None,
    Bool(bool),
    I32(i32),
    Vector2(Vector2),
    Vec(Vec<WidgetSignal>),
    //IoSwap(InputToAdd,OutputToAdd, ToDelete)
    IoSwap(Vec<Vector2>, Vec<Vector2>, Vec<Vector2>),
}

#[derive(PartialEq)]
pub enum WidgetType {
    None,
    Io,
    Perlin,
    Dsquare,
    Coloration,
}

#[derive(Clone, Default)]
pub struct PlugInterface {
    pub wpos: Vector2,
    // inputs data can be seen as Option<Vector2>
    pub inputs: HashMap<Vec2u, Vec<Vector2>>,
    pub outputs: HashMap<Vec2u, Vec<Vector2>>,
}

impl PlugInterface {
    pub fn new(wpos: Vector2) -> Self {
        Self {
            wpos,
            ..Default::default()
        }
    }
}

pub trait WidgetRectangle: Widget {
    fn get_rect(&self) -> Rectangle;
    fn set_rect(&mut self, rect: Rectangle);

    fn get_pos(&self) -> Vector2 {
        let rect = self.get_rect();
        Vector2::new(rect.x, rect.y)
    }

    fn set_pos(&mut self, pos: Vector2) {
        let rect = self.get_pos();
        self.translate(pos - Vector2::new(rect.x, rect.y));
    }

    fn translate(&mut self, offset: Vector2) {
        let mut rect = self.get_rect();
        rect.x += offset.x;
        rect.y += offset.y;
        self.set_rect(rect);
    }
}

pub trait WidgetCollidable: WidgetRectangle {
    fn check_rect_collision(&self, rect: Rectangle) -> bool {
        self.get_rect().check_collision_recs(&rect)
    }

    fn check_circ_collision(&self, circle: &Circle) -> bool {
        self.get_rect()
            .check_collision_circle_rec(circle.pos, circle.rad)
    }

    fn check_point_collision(&self, point: Vector2) -> bool {
        self.get_rect().check_collision_point_rec(point)
    }
}

pub trait WidgetPlugable: WidgetCollidable {
    // fn check_inplug_collisions(&self, circle: &Circle) -> Option<Vector2>;
    // fn check_outplug_collisions(&self, circle: &Circle) -> Option<Vector2>;
    fn check_plug_collisions(&self, circle: &Circle) -> Option<Vector2> {
        self.plug_pos()
            .copied()
            .find(|x| circle.check_plug_collision(Vec2u::into(*x)))
            .map(|x| x.into())
    }

    fn is_input_plug(&self, pos: &Vector2) -> bool {
        self.get_iplug().inputs.contains_key(&Vec2u::from(*pos))
    }

    fn is_output_plug(&self, pos: &Vector2) -> bool {
        self.get_iplug().outputs.contains_key(&Vec2u::from(*pos))
    }

    fn input_pos(&self) -> Box<dyn Iterator<Item = &Vec2u> + '_> {
        Box::new(self.get_iplug().inputs.keys())
    }

    fn output_pos(&self) -> Box<dyn Iterator<Item = &Vec2u> + '_> {
        Box::new(self.get_iplug().outputs.keys())
    }

    fn plug_pos(&self) -> Box<dyn Iterator<Item = &Vec2u> + '_> {
        Box::new(self.input_pos().chain(self.output_pos()))
    }

    fn inputs(&self) -> Box<dyn Iterator<Item = (&Vec2u, &Vec<Vector2>)> + '_> {
        let iplug = self.get_iplug();
        Box::new(iplug.inputs.iter())
    }

    fn outputs(
        &self,
    ) -> Box<dyn Iterator<Item = (&Vec2u, &Vec<Vector2>)> + '_> {
        let iplug = self.get_iplug();
        Box::new(iplug.outputs.iter())
    }

    fn plugs(&self) -> Box<dyn Iterator<Item = (&Vec2u, &Vec<Vector2>)> + '_> {
        Box::new(self.inputs().chain(self.outputs()))
    }

    fn in_links(
        &self,
    ) -> Box<dyn Iterator<Item = (&Vec2u, &Vec<Vector2>)> + '_> {
        Box::new(self.inputs().filter(|(_, y)| !y.is_empty()))
    }

    fn out_links(
        &self,
    ) -> Box<dyn Iterator<Item = (&Vec2u, &Vec<Vector2>)> + '_> {
        Box::new(self.outputs().filter(|(_, y)| !y.is_empty()))
    }

    fn links(&self) -> Box<dyn Iterator<Item = (&Vec2u, &Vec<Vector2>)> + '_> {
        Box::new(self.in_links().chain(self.out_links()))
    }

    fn in_links_mut(
        &mut self,
    ) -> Box<dyn Iterator<Item = (&Vec2u, &mut Vec<Vector2>)> + '_> {
        let iplug = self.get_iplug_mut();
        Box::new(iplug.inputs.iter_mut().filter(|(_, y)| !y.is_empty()))
    }

    fn out_links_mut(
        &mut self,
    ) -> Box<dyn Iterator<Item = (&Vec2u, &mut Vec<Vector2>)> + '_> {
        let iplug = self.get_iplug_mut();
        Box::new(iplug.outputs.iter_mut().filter(|(_, y)| !y.is_empty()))
    }

    fn in_links_values(&self) -> Box<dyn Iterator<Item = &Vec<Vector2>> + '_> {
        Box::new(self.in_links().map(|(_, x)| x))
    }

    fn out_links_values(&self) -> Box<dyn Iterator<Item = &Vec<Vector2>> + '_> {
        Box::new(self.out_links().map(|(_, x)| x))
    }

    fn links_values(&self) -> Box<dyn Iterator<Item = &Vec<Vector2>> + '_> {
        Box::new(self.in_links_values().chain(self.out_links_values()))
    }

    fn in_links_values_mut(
        &mut self,
    ) -> Box<dyn Iterator<Item = &mut Vec<Vector2>> + '_> {
        Box::new(self.in_links_mut().map(|(_, x)| x))
    }

    fn out_links_values_mut(
        &mut self,
    ) -> Box<dyn Iterator<Item = &mut Vec<Vector2>> + '_> {
        Box::new(self.out_links_mut().map(|(_, x)| x))
    }

    fn get_iplug(&self) -> &PlugInterface;
    fn get_iplug_mut(&mut self) -> &mut PlugInterface;
    fn replace_plug_pos(
        &mut self,
        old: Vector2,
        new: Vector2,
    ) -> Option<Vec<Vector2>>;
    fn translate_wplugs(&mut self, offset: Vector2);

    fn draw_plug_links(&self, handle: &mut RaylibMode2D<'_, RaylibDrawHandle>) {
        self.links().for_each(|(x, vecy)| {
            vecy.iter().for_each(|y| {
                handle.draw_line_ex(
                    std::convert::Into::<Vector2>::into(*x),
                    y,
                    LINK_THICK,
                    LINK_COLOR,
                )
            })
        })
    }

    fn draw_input_plugs(
        &self,
        handle: &mut RaylibMode2D<'_, RaylibDrawHandle>,
        globals: &Globals,
    ) {
        let mouse_c = Circle::new(globals.r_mouse, SELECTION_OFFSET);
        let captured = match globals.capture[1] {
            Some(Capture::Circle(c)) => c,
            _ => Circle::default(),
        };

        self.inputs().for_each(|(x, xvec)| {
            let plug_pos: Vector2 = Vec2u::into(*x);

            let color = if mouse_c.check_plug_collision(plug_pos) {
                Color::SKYBLUE
            } else if captured.pos == plug_pos {
                Color::BLUE
            } else if !xvec.is_empty() {
                Color::RED
            } else {
                Color::LIGHTGRAY
            };

            handle.draw_circle_v(plug_pos, PLUG_RADIUS, color);
            handle.draw_ring(plug_pos, 4.0, 6.0, 0.0, 360.0, 1, Color::GRAY);
        })
    }

    fn draw_output_plugs(
        &self,
        handle: &mut RaylibMode2D<'_, RaylibDrawHandle>,
        globals: &Globals,
    ) {
        let mouse_c = Circle::new(globals.r_mouse, SELECTION_OFFSET);
        let captured = match globals.capture[1] {
            Some(Capture::Circle(c)) => c,
            _ => Circle::default(),
        };

        self.outputs().for_each(|(x, xvec)| {
            let plug_pos: Vector2 = Vec2u::into(*x);

            let color = if mouse_c.check_plug_collision(plug_pos) {
                Color::SKYBLUE
            } else if captured.pos == plug_pos {
                Color::BLUE
            } else if !xvec.is_empty() {
                Color::RED
            } else {
                Color::LIGHTGRAY
            };

            handle.draw_circle_v(plug_pos, PLUG_RADIUS, color);
            handle.draw_ring(plug_pos, 4.0, 6.0, 0.0, 360.0, 1, Color::GRAY);
        })
    }

    fn draw_plugs(
        &self,
        handle: &mut RaylibMode2D<'_, RaylibDrawHandle>,
        globals: &Globals,
    ) {
        self.draw_input_plugs(handle, globals);
        self.draw_output_plugs(handle, globals);
    }

    fn apply(&mut self, voxmap: &mut VoxelMap, globals: &mut Globals);
}

impl std::fmt::Display for &dyn WidgetPlugable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}",
            self.links()
                .map(|(x, xvec)| { (Vec2u::into(*x), xvec.clone()) })
                .collect::<Vec<(Vector2, Vec<Vector2>)>>()
        )
    }
}

pub trait WidgetConfigurable: WidgetCollidable {
    fn configure(
        &mut self,
        handle: &mut RaylibMode2D<'_, RaylibDrawHandle>,
        globals: &mut Globals,
    ) -> WidgetSignal;
}

pub trait Widget {
    fn render(
        &mut self,
        handle: &mut RaylibMode2D<'_, RaylibDrawHandle>,
        globals: &mut Globals,
    ) -> WidgetSignal;

    fn call(
        &mut self,
        handle: &mut RaylibMode2D<'_, RaylibDrawHandle>,
        globals: &mut Globals,
    ) -> WidgetSignal;

    fn ready(&mut self);
    fn unready(&mut self);
    fn is_ready(&self) -> bool;

    fn get_type(&self) -> WidgetType {
        WidgetType::None
    }

    fn get_id(&self) -> String;
    fn set_id(&mut self, id: String);

    fn is_visible(&self) -> bool;
    fn set_visible(&mut self, state: bool);
    fn show(&mut self);
    fn hide(&mut self);
    fn toggle_visible(&mut self);

    fn as_widget_rectangle(&self) -> Option<&dyn WidgetRectangle> {
        None
    }

    fn as_widget_rectangle_mut(&mut self) -> Option<&mut dyn WidgetRectangle> {
        None
    }

    fn as_widget_collidable(&self) -> Option<&dyn WidgetCollidable> {
        None
    }

    fn as_widget_collidable_mut(
        &mut self,
    ) -> Option<&mut dyn WidgetCollidable> {
        None
    }

    fn as_widget_plugable(&self) -> Option<&dyn WidgetPlugable> {
        None
    }

    fn as_widget_plugable_mut(&mut self) -> Option<&mut dyn WidgetPlugable> {
        None
    }

    fn as_widget_configurable(&self) -> Option<&dyn WidgetConfigurable> {
        None
    }

    fn as_widget_configurable_mut(
        &mut self,
    ) -> Option<&mut dyn WidgetConfigurable> {
        None
    }
}
