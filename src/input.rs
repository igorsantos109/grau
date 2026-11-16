//! Input management for keyboard, mouse, and touch

use web_sys::{Window, KeyboardEvent, TouchEvent, MouseEvent, HtmlCanvasElement, HtmlElement, Element};
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::Closure;

#[derive(Debug, Clone, Copy)]
pub struct TouchState {
    pub x: f32,      // Normalized 0.0 - 1.0
    pub y: f32,      // Normalized 0.0 - 1.0
    pub active: bool,
}

pub struct InputManager {
    window: Window,
    keys_pressed: Rc<RefCell<HashMap<String, bool>>>,
    keys_just_pressed: Rc<RefCell<HashMap<String, bool>>>,
    keys_just_released: Rc<RefCell<HashMap<String, bool>>>,
    touch_state: Rc<RefCell<Option<TouchState>>>,
    mouse_position: Rc<RefCell<(f32, f32)>>,
    mouse_buttons: Rc<RefCell<HashMap<i16, bool>>>,
}

impl InputManager {
    pub fn new(window: &Window) -> Self {
        let keys_pressed = Rc::new(RefCell::new(HashMap::new()));
        let keys_just_pressed = Rc::new(RefCell::new(HashMap::new()));
        let keys_just_released = Rc::new(RefCell::new(HashMap::new()));
        let touch_state = Rc::new(RefCell::new(None));
        let mouse_position = Rc::new(RefCell::new((0.0, 0.0)));
        let mouse_buttons = Rc::new(RefCell::new(HashMap::new()));

        let input = Self {
            window: window.clone(),
            keys_pressed: keys_pressed.clone(),
            keys_just_pressed: keys_just_pressed.clone(),
            keys_just_released: keys_just_released.clone(),
            touch_state: touch_state.clone(),
            mouse_position: mouse_position.clone(),
            mouse_buttons: mouse_buttons.clone(),
        };

        input.setup_event_listeners(
            keys_pressed,
            keys_just_pressed,
            keys_just_released,
            touch_state,
            mouse_position,
            mouse_buttons,
        );

        input
    }

    fn setup_event_listeners(
        &self,
        keys_pressed: Rc<RefCell<HashMap<String, bool>>>,
        keys_just_pressed: Rc<RefCell<HashMap<String, bool>>>,
        keys_just_released: Rc<RefCell<HashMap<String, bool>>>,
        touch_state: Rc<RefCell<Option<TouchState>>>,
        mouse_position: Rc<RefCell<(f32, f32)>>,
        mouse_buttons: Rc<RefCell<HashMap<i16, bool>>>,
    ) {
        // Keyboard events
        let keydown_keys = keys_pressed.clone();
        let keydown_just = keys_just_pressed.clone();
        let keydown_closure = Closure::wrap(Box::new(move |event: KeyboardEvent| {
            let code = event.code();
            let code_for_check = code.clone();
            keydown_keys.borrow_mut().insert(code.clone(), true);
            keydown_just.borrow_mut().insert(code, true);
            
            // Prevent default for game keys
            if matches!(code_for_check.as_str(), 
                "ArrowUp" | "ArrowDown" | "ArrowLeft" | "ArrowRight" |
                "KeyW" | "KeyA" | "KeyS" | "KeyD" |
                "Space" | "ShiftLeft" | "ShiftRight" |
                "ControlLeft" | "ControlRight" | "KeyC" | "KeyR"
            ) {
                event.prevent_default();
            }
        }) as Box<dyn FnMut(_)>);
        
        self.window.add_event_listener_with_callback("keydown", keydown_closure.as_ref().unchecked_ref()).unwrap();
        keydown_closure.forget();

        let keyup_keys = keys_pressed.clone();
        let keyup_just = keys_just_released.clone();
        let keyup_closure = Closure::wrap(Box::new(move |event: KeyboardEvent| {
            let code = event.code();
            keyup_keys.borrow_mut().insert(code.clone(), false);
            keyup_just.borrow_mut().insert(code, true);
        }) as Box<dyn FnMut(_)>);
        
        self.window.add_event_listener_with_callback("keyup", keyup_closure.as_ref().unchecked_ref()).unwrap();
        keyup_closure.forget();

        // Touch events
        let touch_start_state = touch_state.clone();
        let touch_start_closure = Closure::wrap(Box::new(move |event: TouchEvent| {
            event.prevent_default();
            if let Some(touch) = event.changed_touches().get(0) {
                let canvas = event.target().unwrap().dyn_into::<Element>().ok();
                if let Some(canvas) = canvas {
                    let rect = canvas.get_bounding_client_rect();
                    let x = (touch.client_x() as f32 - rect.left() as f32) / rect.width() as f32;
                    let y = (touch.client_y() as f32 - rect.top() as f32) / rect.height() as f32;
                    *touch_start_state.borrow_mut() = Some(TouchState { x, y, active: true });
                }
            }
        }) as Box<dyn FnMut(_)>);
        
        self.window.add_event_listener_with_callback("touchstart", touch_start_closure.as_ref().unchecked_ref()).unwrap();
        touch_start_closure.forget();

        let touch_move_state = touch_state.clone();
        let touch_move_closure = Closure::wrap(Box::new(move |event: TouchEvent| {
            event.prevent_default();
            if let Some(touch) = event.changed_touches().get(0) {
                let canvas = event.target().unwrap().dyn_into::<Element>().ok();
                if let Some(canvas) = canvas {
                    let rect = canvas.get_bounding_client_rect();
                    let x = (touch.client_x() as f32 - rect.left() as f32) / rect.width() as f32;
                    let y = (touch.client_y() as f32 - rect.top() as f32) / rect.height() as f32;
                    if let Some(state) = touch_move_state.borrow_mut().as_mut() {
                        state.x = x;
                        state.y = y;
                    }
                }
            }
        }) as Box<dyn FnMut(_)>);
        
        self.window.add_event_listener_with_callback("touchmove", touch_move_closure.as_ref().unchecked_ref()).unwrap();
        touch_move_closure.forget();

        let touch_end_state = touch_state.clone();
        let touch_end_closure = Closure::wrap(Box::new(move |_event: TouchEvent| {
            *touch_end_state.borrow_mut() = None;
        }) as Box<dyn FnMut(_)>);
        
        self.window.add_event_listener_with_callback("touchend", touch_end_closure.as_ref().unchecked_ref()).unwrap();
        self.window.add_event_listener_with_callback("touchcancel", touch_end_closure.as_ref().unchecked_ref()).unwrap();
        touch_end_closure.forget();

        // Mouse events
        let mouse_move_pos = mouse_position.clone();
        let mouse_move_closure = Closure::wrap(Box::new(move |event: MouseEvent| {
            let canvas = event.target().unwrap().dyn_into::<Element>().ok();
            if let Some(canvas) = canvas {
                let rect = canvas.get_bounding_client_rect();
                let x = (event.client_x() as f32 - rect.left() as f32) / rect.width() as f32;
                let y = (event.client_y() as f32 - rect.top() as f32) / rect.height() as f32;
                *mouse_move_pos.borrow_mut() = (x, y);
            }
        }) as Box<dyn FnMut(_)>);
        
        self.window.add_event_listener_with_callback("mousemove", mouse_move_closure.as_ref().unchecked_ref()).unwrap();
        mouse_move_closure.forget();

        let mouse_down_btns = mouse_buttons.clone();
        let mouse_down_closure = Closure::wrap(Box::new(move |event: MouseEvent| {
            mouse_down_btns.borrow_mut().insert(event.button(), true);
        }) as Box<dyn FnMut(_)>);
        
        self.window.add_event_listener_with_callback("mousedown", mouse_down_closure.as_ref().unchecked_ref()).unwrap();
        mouse_down_closure.forget();

        let mouse_up_btns = mouse_buttons.clone();
        let mouse_up_closure = Closure::wrap(Box::new(move |event: MouseEvent| {
            mouse_up_btns.borrow_mut().insert(event.button(), false);
        }) as Box<dyn FnMut(_)>);
        
        self.window.add_event_listener_with_callback("mouseup", mouse_up_closure.as_ref().unchecked_ref()).unwrap();
        mouse_up_closure.forget();
    }

    pub fn update(&mut self) {
        // Clear just-pressed/just-released
        self.keys_just_pressed.borrow_mut().clear();
        self.keys_just_released.borrow_mut().clear();
    }

    pub fn is_key_pressed(&self, code: &str) -> bool {
        *self.keys_pressed.borrow().get(code).unwrap_or(&false)
    }

    pub fn was_key_just_pressed(&self, code: &str) -> bool {
        *self.keys_just_pressed.borrow().get(code).unwrap_or(&false)
    }

    pub fn was_key_just_released(&self, code: &str) -> bool {
        *self.keys_just_released.borrow().get(code).unwrap_or(&false)
    }

    pub fn get_touch_state(&self) -> Option<TouchState> {
        *self.touch_state.borrow()
    }

    pub fn get_mouse_position(&self) -> (f32, f32) {
        *self.mouse_position.borrow()
    }

    pub fn is_mouse_button_pressed(&self, button: i16) -> bool {
        *self.mouse_buttons.borrow().get(&button).unwrap_or(&false)
    }
}