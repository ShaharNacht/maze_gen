use std::collections::BTreeMap;

use crate::point::WindowPoint;
use crate::{GFX_UI_HEIGHT, GFX_UI_PADDING, GFX_UI_WIDTH, GFX_UI_X, GFX_UI_Y};

pub struct Ui {
    buttons: BTreeMap<ButtonId, Button>,
    pressed_button_id: Option<ButtonId>,
}

impl Ui {
    pub fn new() -> Self {
        let button_count = ButtonId::all().len() as i64;
        let button_width =
            (GFX_UI_WIDTH - GFX_UI_PADDING - button_count * GFX_UI_PADDING) / button_count;
        let button_height = GFX_UI_HEIGHT - GFX_UI_PADDING * 2;

        let mut buttons = BTreeMap::new();

        for (i, button_id) in ButtonId::all().iter().copied().enumerate() {
            let x = GFX_UI_X + GFX_UI_PADDING + ((button_width + GFX_UI_PADDING) * i as i64);
            let y = GFX_UI_Y + GFX_UI_PADDING;

            let button = Button::new(
                button_id,
                WindowPoint::new(x, y),
                button_width,
                button_height,
            );

            buttons.insert(button_id, button);
        }

        Self {
            buttons,
            pressed_button_id: None,
        }
    }

    pub fn update(&mut self) {
        for button in self.buttons.values_mut() {
            button.update();
        }
    }

    pub fn on_mouse_move(&mut self, mouse: WindowPoint) {
        if self.pressed_button_id.is_none() {
            for button in self.buttons.values_mut() {
                button.on_mouse_move(mouse);
            }
        }
    }

    pub fn on_mouse_press(&mut self, mouse: WindowPoint) {
        self.on_mouse_move(mouse);

        if let Some(pressed_button) = self
            .buttons
            .values_mut()
            .find(|button| button.is_point_inside(mouse))
        {
            pressed_button.on_mouse_press();

            self.pressed_button_id = Some(pressed_button.button_id);
        }

        self.on_mouse_move(mouse);
    }

    pub fn on_mouse_release(&mut self, mouse: WindowPoint) -> Option<ButtonId> {
        let mut clicked_button = None;

        self.on_mouse_move(mouse);

        if let Some(pressed_button_id) = self.pressed_button_id {
            let button = self.buttons.get_mut(&pressed_button_id).unwrap();

            button.on_mouse_release(mouse);

            if button.is_point_inside(mouse) {
                clicked_button = Some(pressed_button_id);
            }

            self.pressed_button_id = None;
        }

        self.on_mouse_move(mouse);

        clicked_button
    }

    pub fn buttons(&self) -> impl Iterator<Item = &Button> {
        self.buttons.values()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ButtonId {
    Step,
    AutoStepOrPause,
    Finish,
    Reset,
}

impl ButtonId {
    fn all() -> &'static [ButtonId] {
        &[
            ButtonId::Step,
            ButtonId::AutoStepOrPause,
            ButtonId::Finish,
            ButtonId::Reset,
        ]
    }

    fn text(&self) -> (&str, Option<&str>) {
        match self {
            ButtonId::Step => ("Step", None),
            ButtonId::AutoStepOrPause => ("Auto-Step", Some("Pause")),
            ButtonId::Finish => ("Finish", None),
            ButtonId::Reset => ("Reset", None),
        }
    }
}

pub struct Button {
    pub button_id: ButtonId,

    pub position: WindowPoint,
    pub width: i64,
    pub height: i64,

    pub state: ButtonState,
}

impl Button {
    fn new(button_id: ButtonId, position: WindowPoint, width: i64, height: i64) -> Self {
        let state = ButtonState::Normal {
            is_mouse_over: false,
            highlight: 0.0,
        };

        Self {
            button_id,
            position,
            width,
            height,
            state,
        }
    }

    pub fn text(&self) -> (&str, Option<&str>) {
        self.button_id.text()
    }

    fn update(&mut self) {
        if let ButtonState::Normal {
            is_mouse_over,
            highlight,
        } = &mut self.state
        {
            if *is_mouse_over {
                *highlight += 0.1;
            } else {
                *highlight -= 0.1;
            }

            *highlight = highlight.clamp(0.0, 1.0);
        }
    }

    fn on_mouse_move(&mut self, mouse: WindowPoint) {
        let mouse_over = self.is_point_inside(mouse);

        if let ButtonState::Normal { is_mouse_over, .. } = &mut self.state {
            *is_mouse_over = mouse_over;
        }
    }

    fn on_mouse_press(&mut self) {
        self.state = ButtonState::Pressed;
    }

    fn on_mouse_release(&mut self, mouse: WindowPoint) {
        self.state = ButtonState::Normal {
            is_mouse_over: self.is_point_inside(mouse),
            highlight: 0.0,
        };
    }

    fn is_point_inside(&self, point: WindowPoint) -> bool {
        point.x >= self.position.x
            && point.x < (self.position.x + self.width)
            && point.y >= self.position.y
            && point.y < (self.position.y + self.height)
    }
}

pub enum ButtonState {
    Normal { is_mouse_over: bool, highlight: f64 },
    Pressed,
}
