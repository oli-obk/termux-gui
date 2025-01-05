use crate::activity::{Activity, Flags};
use crate::event::{self, Event};
use crate::layouts::swipe_refresh_layout::SwipeRefreshLayout;
use crate::widgets::View;
use crate::TGui;
use std::collections::HashMap;

type HandlerList<'a, T, E> = Vec<Box<dyn FnMut(&T, &mut Handler<'a, E>) -> Result<(), E> + 'a>>;

struct ActivityInfo<'a, E> {
    widget: HashMap<i32, HandlerList<'a, event::Widget, E>>,
    handlers: HandlerList<'a, event::Activity, E>,
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
                    kind: event::Activity::Destroy,
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
        activity: Activity<'a>,
        eh: impl FnMut(&event::Activity, &mut Self) -> Result<(), E> + 'a,
    ) {
        let eh = Box::new(eh);
        self.activity
            .get_mut(&activity.aid())
            .unwrap()
            .handlers
            .push(eh);
    }

    pub fn add_widget(
        &mut self,
        widget: impl View,
        eh: impl FnMut(&event::Widget, &mut Self) -> Result<(), E> + 'a,
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

    /// Convenience helper for registering a `Click { .. }` widget event handler.
    pub fn on_click(
        &mut self,
        view: impl View,
        mut f: impl FnMut(&mut Self) -> Result<(), E> + 'a,
    ) {
        self.add_widget(view, move |w, ehs| {
            Ok(if let event::Widget::Click { .. } = w {
                f(ehs)?
            })
        })
    }

    /// Convenience helper for registering a `LongClick { .. }` widget event handler.
    /// Automatically enables long click events for this widget.
    pub fn on_long_click(
        &mut self,
        view: impl View,
        mut f: impl FnMut(&mut Self) -> Result<(), E> + 'a,
    ) {
        view.send_long_click_event(true);
        self.add_widget(view, move |w, ehs| {
            Ok(if let event::Widget::LongClick { .. } = w {
                f(ehs)?
            })
        })
    }

    /// Convenience helper for registering a `Refresh` widget event handler.
    /// Automatically clears the refreshing spinner when the handler finishes.
    pub fn on_refresh(
        &mut self,
        view: SwipeRefreshLayout<'a>,
        mut f: impl FnMut(&mut Self) -> Result<(), E> + 'a,
    ) {
        self.add_widget(view, move |w, ehs| {
            Ok(if let event::Widget::Refresh = w {
                f(ehs)?;
                view.set_refreshing(false);
            })
        })
    }

    pub fn new(tgui: &'a TGui) -> Self {
        Self {
            tgui,
            activity: Default::default(),
        }
    }

    /// Creates a new activity to have its events tracked.
    /// The closure is invoked whenever the activity is (re)created by android.
    /// This happens once at the start and whenever android moves the app far into the background.
    pub fn new_activity(
        &mut self,
        flags: Flags,
        mut f: impl FnMut(Activity<'a>, &mut Self) -> Result<(), E> + 'a,
    ) {
        let activity = self.tgui.new_activity(flags);

        self.activity.insert(
            activity.aid(),
            ActivityInfo {
                handlers: vec![],
                widget: HashMap::new(),
            },
        );
        self.add_activity(activity, move |a, ehs| {
            Ok(if let event::Activity::Create = a {
                f(activity, ehs)?
            })
        });
    }
}
