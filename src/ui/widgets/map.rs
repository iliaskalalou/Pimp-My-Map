// use crate::ieef64::*;
use crate::multimap::*;
use crate::widgets::*;
use raylib::prelude::*;
// use std::collections::HashMap;

pub struct ObjMap {
    pub objs: MultiMap<Vec2u, Box<dyn Widget>>,
    pub id: String,
    pub locked: bool,
    visible: bool,
}

impl ObjMap {
    pub fn new(id: String) -> Self {
        Self {
            objs: MultiMap::with_capacity(16),
            id,
            locked: false,
            visible: true,
        }
    }

    pub fn lock(&mut self) {
        self.locked = true
    }
    pub fn unlock(&mut self) {
        self.locked = false
    }
    pub fn toggle_lock(&mut self) {
        self.locked ^= true
    }
    pub fn get_state(&self) -> bool {
        self.locked
    }
    pub fn set_state(&mut self, state: bool) {
        self.locked = state
    }

    pub fn any_rect_colliding(&self, rect: Rectangle) -> bool {
        self.objs
            .data_values()
            .filter_map(|obj| obj.as_widget_collidable())
            .any(|x| x.check_rect_collision(rect))
    }

    pub fn find_target(
        &mut self,
        circ: &Circle,
    ) -> Option<&mut dyn WidgetCollidable> {
        self.objs
            .data_values_mut()
            .filter_map(|x| x.as_widget_collidable_mut())
            .find(|x| x.check_circ_collision(circ))
    }

    pub fn dump(&mut self) {
        let mut entries = HashMap::<Index, Vec<Vector2>>::with_capacity(
            self.objs.unique_keys_count().into(),
        );

        self.objs.key_values().for_each(|(x, xidx)| {
            if let Some(act) = entries.get_mut(xidx) {
                act.push((*x).into());
            } else {
                entries.insert(*xidx, vec![(*x).into()]);
            }
        });

        println!("{:#?}", entries);
    }

    pub fn link_widgets(
        &mut self,
        from: Vector2,
        to: Vector2,
    ) -> Option<(Vector2, Vector2)> {
        if self.objs.keys_get(&from.into())?
            == self.objs.keys_get(&to.into())?
        {
            return None;
        }

        if self
            .objs
            .get(&from.into())?
            .as_widget_plugable()?
            .is_input_plug(&from)
        {
            let to_widget =
                self.objs.get_mut(&to.into())?.as_widget_plugable_mut()?;

            if to_widget.is_output_plug(&to) {
                let tmp =
                    to_widget.get_iplug_mut().outputs.get_mut(&to.into())?;

                if !tmp.contains(&from) {
                    tmp.push(from);
                }

                let res = self
                    .objs
                    .get_mut(&from.into())?
                    .as_widget_plugable_mut()?
                    .get_iplug_mut()
                    .inputs
                    .insert(from.into(), vec![to]);

                if let Some(v) = res {
                    if let Some(&target) = v.last() {
                        self.objs
                            .get_mut(&target.into())
                            .unwrap()
                            .as_widget_plugable_mut()
                            .unwrap()
                            .get_iplug_mut()
                            .outputs
                            .get_mut(&target.into())
                            .unwrap()
                            .retain(|x| *x != from)
                    }
                }

                Some((from, to))
            } else {
                None
            }
        } else {
            let to_widget =
                self.objs.get_mut(&to.into())?.as_widget_plugable_mut()?;

            if to_widget.is_input_plug(&to) {
                let res = to_widget
                    .get_iplug_mut()
                    .inputs
                    .insert(to.into(), vec![from]);

                if let Some(v) = res {
                    if let Some(&target) = v.last() {
                        self.objs
                            .get_mut(&target.into())
                            .unwrap()
                            .as_widget_plugable_mut()
                            .unwrap()
                            .get_iplug_mut()
                            .outputs
                            .get_mut(&target.into())
                            .unwrap()
                            .retain(|x| *x != to)
                    }
                }

                let tmp = self
                    .objs
                    .get_mut(&from.into())?
                    .as_widget_plugable_mut()?
                    .get_iplug_mut()
                    .outputs
                    .get_mut(&from.into())?;

                if !tmp.contains(&to) {
                    tmp.push(to);
                }

                Some((from, to))
            } else {
                None
            }
        }
    }

    fn update_prev_widgets_plugs(
        &mut self,
        prev: Vec<Vector2>,
        org: Vector2,
        offset: Vector2,
    ) {
        prev.into_iter().for_each(|x| {
            let prev_outputs: Vec<Vec<Vector2>> = self
                .objs
                .get_mut(&x.into())
                .unwrap()
                .as_widget_plugable_mut()
                .unwrap()
                .get_iplug_mut()
                .outputs
                .values()
                .cloned()
                .collect();

            let to_treat: Vec<Vector2> = prev_outputs
                .into_iter()
                .flatten()
                .filter(|x| {
                    self.objs.is_alias_of(&org.into(), &Vector2::into(*x))
                })
                .collect();

            to_treat.into_iter().for_each(|x1| {
                self.objs
                    .get_mut(&x.into())
                    .unwrap()
                    .as_widget_plugable_mut()
                    .unwrap()
                    .get_iplug_mut()
                    .outputs
                    .values_mut()
                    .for_each(|x2| {
                        x2.iter_mut().for_each(|x2| {
                            if *x2 == x1 {
                                *x2 += offset;
                            }
                        })
                    })
            })
        });
    }

    fn update_next_widgets_plugs(
        &mut self,
        next: Vec<Vector2>,
        org: Vector2,
        offset: Vector2,
    ) {
        next.into_iter().for_each(|x| {
            let next_inputs: Vec<Vec<Vector2>> = self
                .objs
                .get_mut(&x.into())
                .unwrap()
                .as_widget_plugable_mut()
                .unwrap()
                .get_iplug_mut()
                .inputs
                .values()
                .cloned()
                .collect();

            let to_treat: Vec<Vector2> = next_inputs
                .into_iter()
                .flatten()
                .filter(|x| {
                    self.objs.is_alias_of(&org.into(), &Vector2::into(*x))
                })
                .collect();

            to_treat.into_iter().for_each(|x1| {
                self.objs
                    .get_mut(&x.into())
                    .unwrap()
                    .as_widget_plugable_mut()
                    .unwrap()
                    .get_iplug_mut()
                    .inputs
                    .values_mut()
                    .for_each(|x2| {
                        x2.iter_mut().for_each(|x2| {
                            if *x2 == x1 {
                                *x2 += offset;
                            }
                        })
                    })
            })
        });
    }

    fn update_key_aliases(&mut self, org: Vector2, offset: Vector2) {
        let new_pos = org + offset;
        let old_aliases = self.objs.unalias(&org.into());
        self.objs.replace_key(&org.into(), new_pos.into());

        old_aliases.into_iter().for_each(|x| {
            self.objs.alias(&new_pos.into(), x + offset.into());
        });
    }

    pub fn move_widget_by(&mut self, org: Vector2, offset: Vector2) {
        let mut prev: Vec<Vector2> = Vec::new();
        let mut next: Vec<Vector2> = Vec::new();

        self.objs.entry(org.into()).and_modify(|f| {
            match f.as_widget_plugable_mut() {
                Some(w) => {
                    w.translate_wplugs(offset);
                    let i_plug = w.get_iplug_mut();

                    prev = i_plug
                        .inputs
                        .values()
                        .flatten()
                        .copied()
                        .collect::<Vec<Vector2>>();

                    next = i_plug
                        .outputs
                        .values()
                        .flatten()
                        .copied()
                        .collect::<Vec<Vector2>>();
                }
                _ => f.as_widget_collidable_mut().unwrap().translate(offset),
            }
        });

        self.update_prev_widgets_plugs(prev, org, offset);
        self.update_next_widgets_plugs(next, org, offset);
        self.update_key_aliases(org, offset);
    }

    // delete all widget's link that are pointing to "ghost" plugs
    pub fn plug_ghost_buster(&mut self, pos: Vector2) {
        let iplug = self
            .objs
            .get_mut(&pos.into())
            .unwrap()
            .as_widget_plugable_mut()
            .unwrap()
            .get_iplug_mut();

        let in_to_bust =
            iplug.inputs.values().flatten().copied().collect::<Vec<_>>();

        let out_to_bust = iplug
            .outputs
            .values()
            .flatten()
            .copied()
            .collect::<Vec<_>>();

        let in_to_bust = in_to_bust
            .into_iter()
            .filter(|x| !self.objs.contains_key(&Vector2::into(*x)))
            .collect::<Vec<_>>();

        let out_to_bust = out_to_bust
            .into_iter()
            .filter(|x| !self.objs.contains_key(&Vector2::into(*x)))
            .collect::<Vec<_>>();

        let iplug = self
            .objs
            .get_mut(&pos.into())
            .unwrap()
            .as_widget_plugable_mut()
            .unwrap()
            .get_iplug_mut();

        iplug
            .inputs
            .values_mut()
            .for_each(|x| x.retain(|x| !in_to_bust.contains(x)));

        iplug
            .outputs
            .values_mut()
            .for_each(|x| x.retain(|x| !out_to_bust.contains(x)))
    }

    pub fn remove_widget_links(&mut self, pos: Vector2) {
        let mut plugs: Vec<(Vec2u, Vec<Vector2>)> = Vec::new();

        if let Some(plugable) = self
            .objs
            .get_mut(&pos.into())
            .unwrap()
            .as_widget_plugable_mut()
        {
            plugs = plugable
                .links()
                .map(|(x, xvec)| (*x, xvec.clone()))
                .collect();

            let iplug = plugable.get_iplug_mut();
            iplug.inputs.values_mut().for_each(|x| x.clear());
            iplug.outputs.values_mut().for_each(|x| x.clear());
        }

        plugs.into_iter().for_each(|(x, xvec)| {
            self.objs.remove_alias(&x);
            xvec.into_iter().for_each(|x| self.plug_ghost_buster(x));
            self.objs.alias(&pos.into(), x);
        });
    }

    pub fn remove_links(&mut self, plug_pos: Vector2) {
        let widget = self
            .objs
            .get_mut(&plug_pos.into())
            .map(|x| x.as_widget_plugable_mut());

        if widget.is_none() {
            return;
        }

        let widget = widget.unwrap().unwrap();

        // let mut others: Vec<Vector2> = Vec::new();

        if widget.is_input_plug(&plug_pos) {
            let to_treat: Vec<Vector2> = widget
                .in_links()
                .filter(|(x, _)| **x == plug_pos.into())
                .flat_map(|(_, xvec)| xvec.clone())
                .collect();

            widget
                .get_iplug_mut()
                .inputs
                .get_mut(&plug_pos.into())
                .unwrap()
                .clear();

            to_treat.into_iter().for_each(|x| {
                self.objs
                    .get_mut(&x.into())
                    .unwrap()
                    .as_widget_plugable_mut()
                    .unwrap()
                    .out_links_values_mut()
                    .for_each(|x| x.retain(|elt| *elt != plug_pos))
            })
        } else if widget.is_output_plug(&plug_pos) {
            let to_treat: Vec<Vector2> = widget
                .out_links()
                .filter(|(x, _)| **x == plug_pos.into())
                .flat_map(|(_, xvec)| xvec.clone())
                .collect();

            widget
                .get_iplug_mut()
                .outputs
                .get_mut(&plug_pos.into())
                .unwrap()
                .clear();

            to_treat.into_iter().for_each(|x| {
                self.objs
                    .get_mut(&x.into())
                    .unwrap()
                    .as_widget_plugable_mut()
                    .unwrap()
                    .in_links_values_mut()
                    .for_each(|x| x.retain(|elt| *elt != plug_pos))
            })
        }
    }

    pub fn get_prev_widgets(&self, pos: Index) -> Option<Vec<Index>> {
        Some(
            self.objs
                .data_get(&pos)?
                .as_widget_plugable()?
                .in_links_values()
                .flatten()
                .map(|x| *self.objs.keys_get(&Vector2::into(*x)).unwrap())
                .collect::<Vec<Index>>(),
        )
    }

    pub fn get_next_widgets(&self, pos: Index) -> Option<Vec<Index>> {
        Some(
            self.objs
                .data_get(&pos)?
                .as_widget_plugable()?
                .out_links_values()
                .flatten()
                .map(|x| *self.objs.keys_get(&Vector2::into(*x)).unwrap())
                .collect::<Vec<Index>>(),
        )
    }

    pub fn remove_widget(&mut self, pos: Vector2) -> Option<Box<dyn Widget>> {
        let target = self.objs.remove(&Vector2::into(pos));

        if let Some(w) = target {
            if let Some(plugable) = w.as_widget_plugable() {
                plugable.plug_pos().for_each(|x| {
                    self.objs.remove_alias(x);
                });

                plugable
                    .links_values()
                    .flatten()
                    .for_each(|x| self.plug_ghost_buster(*x));

                Some(w)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl Widget for ObjMap {
    fn render(
        &mut self,
        handle: &mut RaylibMode2D<'_, RaylibDrawHandle>,
        globals: &mut Globals,
    ) -> WidgetSignal {
        WidgetSignal::Vec(
            self.objs
                .data_values_mut()
                .map(|x| x.render(handle, globals))
                .collect::<Vec<WidgetSignal>>(),
        )
    }

    fn call(
        &mut self,
        handle: &mut RaylibMode2D<'_, RaylibDrawHandle>,
        globals: &mut Globals,
    ) -> WidgetSignal {
        WidgetSignal::Vec(
            self.objs
                .data_values_mut()
                .map(|x| x.call(handle, globals))
                .collect::<Vec<WidgetSignal>>(),
        )
    }

    fn ready(&mut self) {
        self.objs.data_values_mut().for_each(|x| x.ready())
    }
    fn unready(&mut self) {
        self.objs.data_values_mut().for_each(|x| x.unready())
    }
    fn is_ready(&self) -> bool {
        self.objs.data_values().all(|x| x.is_ready())
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
}
