use super::{Activity, Event, Widget};
use crate::activity::Flags;
use crate::ui::Ui;
use crate::widgets::View;
use crate::TGui;
use std::collections::HashMap;

type HandlerList<'a, T, E> = Vec<Box<dyn FnMut(&T, &mut Handler<'a, E>) -> Result<(), E> + 'a>>;

struct ActivityInfo<'a, E> {
    widget: HashMap<i32, HandlerList<'a, Widget, E>>,
    handlers: HandlerList<'a, Activity, E>,
    ui: Ui<'a>,
}

#[derive(Debug)]
pub enum NoErrors {}

pub struct Handler<'a, E = NoErrors> {
    tgui: &'a TGui,
    activity: HashMap<i32, ActivityInfo<'a, E>>,
}

impl<'a, E> Handler<'a, E> {
    fn handle(&mut self, event: &Event) -> Result<(), E> {
        match event {
            Event::System(_) => {}
            Event::Activity { aid, kind, .. } => {
                let n = self.activity[aid].handlers.len();

                for i in 0..n {
                    // Ensure that we don't call `handle` recursively
                    let mut eh = std::mem::replace(
                        &mut self.activity.get_mut(aid).unwrap().handlers[i],
                        Box::new(|_, _| unreachable!()),
                    );
                    eh(kind, self)?;
                    self.activity.get_mut(aid).unwrap().handlers[i] = eh;
                }
            }
            Event::Widget { aid, id, kind } => {
                let n = self.activity[aid].widget.get(id).map(|ehs| ehs.len());
                if let Some(n) = n {
                    for i in 0..n {
                        // Ensure that we don't call `handle` recursively
                        let mut eh = std::mem::replace(
                            &mut self
                                .activity
                                .get_mut(aid)
                                .unwrap()
                                .widget
                                .get_mut(id)
                                .unwrap()[i],
                            Box::new(|_, _| unreachable!()),
                        );
                        eh(kind, self)?;
                        self.activity
                            .get_mut(aid)
                            .unwrap()
                            .widget
                            .get_mut(id)
                            .unwrap()[i] = eh;
                    }
                }
            }
            Event::Other { .. } => {}
        }
        Ok(())
    }

    pub fn handle_all_events(mut self) -> Result<(), E> {
        loop {
            let event = self.tgui.event().unwrap();

            self.handle(&event)?;

            match &event {
                Event::Activity {
                    finishing: true,
                    kind: Activity::Destroy,
                    aid,
                } => {
                    self.activity.remove(aid).unwrap();
                    if self.activity.is_empty() {
                        return Ok(());
                    }
                }
                _ => {
                    if self.tgui.debug {
                        eprintln!("{event:?}")
                    }
                }
            }
        }
    }

    pub fn add_activity(
        &mut self,
        aid: i32,
        eh: impl FnMut(&Activity, &mut Self) -> Result<(), E> + 'a,
    ) {
        let eh = Box::new(eh);
        self.activity.get_mut(&aid).unwrap().handlers.push(eh);
    }

    pub fn add_widget(
        &mut self,
        widget: &impl View,
        eh: impl FnMut(&Widget, &mut Self) -> Result<(), E> + 'a,
    ) {
        let aid = widget.get_activity().aid();
        let eh = Box::new(eh);
        self.activity
            .get_mut(&aid)
            .unwrap()
            .widget
            .entry(widget.get_id())
            .or_default()
            .push(eh);
    }

    pub fn new(tgui: &'a TGui) -> Self {
        Self {
            tgui,
            activity: Default::default(),
        }
    }

    /// Creates a new activity to have its events tracked.
    pub fn new_activity(&mut self, flags: Flags) -> i32 {
        let ui = self.tgui.ui(flags);
        let aid = ui.activity().aid();
        self.activity.insert(
            aid,
            ActivityInfo {
                ui,
                handlers: vec![],
                widget: HashMap::new(),
            },
        );
        aid
    }
}

impl<'a, E> std::ops::Index<i32> for Handler<'a, E> {
    type Output = Ui<'a>;
    fn index(&self, aid: i32) -> &Self::Output {
        &self.activity[&aid].ui
    }
}
