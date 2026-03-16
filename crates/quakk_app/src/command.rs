use std::sync::{Arc, Mutex};

use crate::AppState;

pub trait Command {
    fn title() -> &'static str;
    fn description() -> Option<&'static str> {
        None
    }

    fn action(&self, app_state: &mut Arc<Mutex<AppState>>);
}

pub struct Termout {
    pub string: String,
}

impl Command for Termout {
    fn title() -> &'static str {
        "Termout"
    }

    fn action(&self, app_state: &mut Arc<Mutex<AppState>>) {
        dbg!(&self.string);
        dbg!(app_state.lock());
    }
}
