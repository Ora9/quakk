use gpui::{Action, App, Entity, FocusHandle, Length, SharedString, prelude::*, rems};
use gpui::{Window, div};
use gpui_component::input::InputState;
use gpui_component::v_flex;

pub struct PickerItem {
    text: SharedString,
    action: Option<Box<dyn Action>>,
}

impl PickerItem {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string().into(),
            action: None,
        }
    }
}

pub struct Picker {
    items: Vec<PickerItem>,
    selected_index: usize,
    input_state: Entity<InputState>,

    width: Option<Length>,
    max_height: Option<Length>,

    focus_handle: FocusHandle,
}

impl Picker {
    pub fn new(items: Vec<PickerItem>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            items,

            input_state: cx.new(|cx| InputState::new(window, cx)),
            selected_index: 0,

            width: None,
            max_height: None,

            focus_handle: cx.focus_handle(),
        }
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn max_height(mut self, max_height: impl Into<Length>) -> Self {
        self.max_height = Some(max_height.into());
        self
    }

    pub fn focus(&self, window: &mut Window, cx: &mut App) {
        self.focus_handle.focus(window, cx);
    }
}

impl Render for Picker {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex().w(rems(34.)).child(self.input_state.clone())
    }
}
