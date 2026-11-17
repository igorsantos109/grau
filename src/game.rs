//! Core game loop and state management

use crate::physics::PhysicsEngine;
use crate::renderer::Renderer;
use crate::input::InputManager;
use crate::audio::AudioManager;
use crate::bike::Bike;
use crate::camera::Camera;
use crate::ui::UI;
use web_sys::{window, HtmlCanvasElement};
use js_sys::Date;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;

/// Detecta ?autotest=1 na URL para ativar o modo de teste automatizado.
fn detect_autotest() -> bool {
    web_sys::window()
        .and_then(|w| w.location().search().ok())
        .map(|s| s.contains("autotest=1"))
        .unwrap_or(false)
}

pub struct Game {
    physics: PhysicsEngine,
    renderer: Renderer,
    input: InputManager,
    audio: AudioManager,
    bike: Bike,
    camera: Camera,
    ui: UI,
    last_time: f64,
    running: bool,
    paused: bool,
    autotest: bool,
    autotest_frame: u64,
    autotest_done: bool,
    frame_count: u64,
}

impl Game {
    pub fn new() -> Self {
        web_sys::console::log_1(&"[GRAU] Game::new() start".into());
        let window = window().expect("no global window");
        let document = window.document().expect("no document");
        web_sys::console::log_1(&"[GRAU] Getting canvas...".into());
        let canvas = document.get_element_by_id("game-canvas")
            .expect("canvas not found")
            .dyn_into::<HtmlCanvasElement>()
            .expect("not a canvas");
        web_sys::console::log_1(&"[GRAU] Canvas found".into());

        // Set canvas size to match display size
        let width = window.inner_width().unwrap().as_f64().unwrap() as u32;
        let height = window.inner_height().unwrap().as_f64().unwrap() as u32;
        canvas.set_width(width.max(1));
        canvas.set_height(height.max(1));
        web_sys::console::log_1(&format!("[GRAU] Canvas sized: {}x{}", width, height).into());

        web_sys::console::log_1(&"[GRAU] Creating renderer...".into());
        let renderer = Renderer::new(&canvas).unwrap_or_else(|e| {
            web_sys::console::error_1(&format!("[GRAU] Renderer failed: {:?}", e).into());
            panic!("failed to create renderer: {:?}", e);
        });
        web_sys::console::log_1(&"[GRAU] Renderer OK".into());
        
        let physics = PhysicsEngine::new();
        web_sys::console::log_1(&"[GRAU] Creating input...".into());
        let input = InputManager::new(&window);
        web_sys::console::log_1(&"[GRAU] Input OK".into());
        let mut audio = AudioManager::new();
        let mut bike = Bike::new();
        // Carrega recordes persistidos (localStorage) ao iniciar
        bike.load_records();
        let camera = Camera::new();
        web_sys::console::log_1(&"[GRAU] Creating UI...".into());
        let ui = UI::new(&document);
        web_sys::console::log_1(&"[GRAU] UI OK".into());

        // Áudio só inicia após gesto do usuário: ouvimos o evento customizado
        // "grau-start" disparado pelo botão JOGAR da tela inicial.
        {
            let audio_ptr = &mut audio as *mut AudioManager;
            let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
                unsafe {
                    (*audio_ptr).init();
                    (*audio_ptr).ensure_running();
                }
                web_sys::console::log_1(&"[GRAU] Áudio inicializado via gesto do usuário".into());
            }) as Box<dyn FnMut(web_sys::Event)>);
            window.add_event_listener_with_callback("grau-start", closure.as_ref().unchecked_ref()).ok();
            closure.forget();
        }

        Self {
            physics,
            renderer,
            input,
            audio,
            bike,
            camera,
            ui,
            last_time: Date::now(),
            running: true,
            paused: false,
            autotest: detect_autotest(),
            autotest_frame: 0,
            autotest_done: false,
            frame_count: 0,
        }
    }

    pub fn run(self) {
        web_sys::console::log_1(&"[GRAU] Game::run() - scheduling first frame".into());
        let window = web_sys::window().expect("no global window");
        
        // Leak self to get a stable pointer for the closure
        let game_ptr = Box::leak(Box::new(self)) as *mut Self;
        
        let closure = Closure::wrap(Box::new(move || {
            unsafe {
                (*game_ptr).tick();
            }
        }) as Box<dyn FnMut()>);

        match window.request_animation_frame(closure.as_ref().unchecked_ref()) {
            Ok(_) => web_sys::console::log_1(&"[GRAU] First frame scheduled".into()),
            Err(e) => web_sys::console::error_1(&format!("[GRAU] RAF failed: {:?}", e).into()),
        }
        closure.forget();
    }

    fn tick(&mut self) {
        if !self.running {
            return;
        }

        if self.frame_count == 0 {
            web_sys::console::log_1(&"[GRAU] First tick!".into());
        }
        if self.frame_count == 1 {
            web_sys::console::log_1(&"[GRAU] Second tick!".into());
        }

        let window = web_sys::window().expect("no global window");
        
        // Update canvas size on resize
        let doc = window.document().expect("no document");
        if let Some(canvas_el) = doc.get_element_by_id("game-canvas") {
            if let Ok(canvas) = canvas_el.dyn_into::<HtmlCanvasElement>() {
                let w = window.inner_width().ok().and_then(|v| v.as_f64()).unwrap_or(800.0) as u32;
                let h = window.inner_height().ok().and_then(|v| v.as_f64()).unwrap_or(600.0) as u32;
                if canvas.width() != w || canvas.height() != h {
                    canvas.set_width(w.max(1));
                    canvas.set_height(h.max(1));
                }
            }
        }
        
        let now = Date::now();
        let dt = (now - self.last_time) / 1000.0;
        self.last_time = now;
        let dt = dt.clamp(0.0, 1.0 / 30.0);

        if self.frame_count < 3 {
            web_sys::console::log_1(&format!("[GRAU] tick #{}, dt={:.4}", self.frame_count, dt).into());
        }

        self.update(dt);
        self.render();

        self.frame_count += 1;
        if self.frame_count % 60 == 0 {
            self.ui.update_fps((1.0 / dt.max(0.001)) as f32);
        }

        // Schedule next frame - use the Rc pattern correctly
        let game_ptr = self as *mut Self;
        let closure = Closure::wrap(Box::new(move || {
            unsafe {
                (*game_ptr).tick();
            }
        }) as Box<dyn FnMut()>);

        let _ = window.request_animation_frame(closure.as_ref().unchecked_ref());
        closure.forget();
    }

    fn update(&mut self, dt: f64) {
        // Update input (sempre, para detectar ESC/P mesmo pausado)
        self.input.update();

        // Toggle de pausa
        if self.input.was_key_just_pressed("Escape") || self.input.was_key_just_pressed("KeyP") {
            self.paused = !self.paused;
            self.ui.set_paused(self.paused);
            web_sys::console::log_1(&format!("[GRAU] Pausa: {}", self.paused).into());
        }

        if self.paused {
            return; // congela física/câmera, mas segue renderizando e leggando input
        }

        // Modo autotest (?autotest=1): simula input por código para validação headless
        if self.autotest && !self.autotest_done {
            self.run_autotest(dt);
        } else {
            // Handle game controls
            self.handle_input(dt);
        }

        // Update physics
        self.physics.update(&mut self.bike, dt as f32, &self.input);

        // Update camera
        self.camera.update(&self.bike, dt as f32);

        // Update audio
        self.audio.update(&self.bike);

        // Reset suave automático quando a moto tomba (fallen) — salva recordes antes
        if self.bike.fallen {
            self.bike.save_records();
            self.bike.reset();
        }

        // Update UI (HUD + barras de input)
        self.ui.update(&self.bike, &self.camera);
        self.ui.update_dom(&self.bike);
    }

    /// Autotest: simula W (acelerar) por alguns segundos, depois Space (wheelie),
    /// e loga o resultado para validação headless ler no console.
    /// Ativado com ?autotest=1 na URL.
    fn run_autotest(&mut self, dt: f64) {
        self.autotest_frame += 1;
        let f = self.autotest_frame;

        // Fase 1 (frames 10..130): acelerar (W)
        if f >= 10 && f < 130 {
            self.bike.apply_throttle(1.0);
            self.bike.apply_steering(0.0);
        } else if f >= 130 && f < 220 {
            // Fase 2: tentar wheelie (Space)
            self.bike.apply_throttle(0.3);
            self.bike.start_wheelie();
        } else if f >= 220 {
            // Fim: loga resultado
            self.autotest_done = true;
            let speed_kmh = self.bike.speed * 3.6;
            let dist = self.bike.total_distance;
            let wheelie = self.bike.wheelie_angle;
            web_sys::console::log_1(&format!(
                "[GRAU][AUTOTEST] RESULT speed_kmh={:.1} dist_m={:.1} wheelie_deg={:.1} frames={}",
                speed_kmh, dist, wheelie, f
            ).into());
            // Critério de aceite: moto andou (dist > 5m)
            if dist > 5.0 {
                web_sys::console::log_1(&"[GRAU][AUTOTEST] PASS: moto acelerou e percorreu distância".into());
            } else {
                web_sys::console::log_1(&"[GRAU][AUTOTEST] FAIL: moto não saiu do lugar".into());
            }
            return;
        }

        // Física e câmera seguem normalmente
        self.physics.update(&mut self.bike, dt as f32, &self.input);
        self.camera.update(&self.bike, dt as f32);
    }

    fn handle_input(&mut self, _dt: f64) {
        let input = &self.input;

        // Acceleration
        if input.is_key_pressed("ArrowUp") || input.is_key_pressed("KeyW") {
            self.bike.apply_throttle(1.0);
        } else if input.is_key_pressed("ArrowDown") || input.is_key_pressed("KeyS") {
            self.bike.apply_brake(1.0);
        } else {
            self.bike.apply_throttle(0.0);
        }

        // Steering
        if input.is_key_pressed("ArrowLeft") || input.is_key_pressed("KeyA") {
            self.bike.apply_steering(-1.0);
        } else if input.is_key_pressed("ArrowRight") || input.is_key_pressed("KeyD") {
            self.bike.apply_steering(1.0);
        } else {
            self.bike.apply_steering(0.0);
        }

        // Wheelie control (Space or Shift)
        if input.is_key_pressed("Space") || input.is_key_pressed("ShiftLeft") {
            self.bike.start_wheelie();
        } else {
            self.bike.stop_wheelie();
        }

        // Drift (Control)
        if input.is_key_pressed("ControlLeft") || input.is_key_pressed("ControlRight") {
            self.bike.start_drift();
        } else {
            self.bike.stop_drift();
        }

        // Camera toggle (C)
        if input.was_key_just_pressed("KeyC") {
            self.camera.toggle_mode();
        }

        // Reset (R) — salva recordes antes de resetar
        if input.was_key_just_pressed("KeyR") {
            self.bike.save_records();
            self.bike.reset();
        }

        // Touch/mobile input handling
        if let Some(touch) = input.get_touch_state() {
            self.handle_touch_input(touch);
        }
    }

    fn handle_touch_input(&mut self, touch: crate::input::TouchState) {
        // Left side of screen - steering
        if touch.x < 0.5 {
            self.bike.apply_steering((touch.x - 0.25) * 4.0);
        }
        // Right side - throttle/brake
        if touch.x > 0.5 {
            if touch.y < 0.5 {
                self.bike.apply_throttle(1.0 - touch.y * 2.0);
            } else {
                self.bike.apply_brake((touch.y - 0.5) * 2.0);
            }
        }
        // Wheelie button area (bottom right)
        if touch.x > 0.7 && touch.y > 0.7 {
            self.bike.start_wheelie();
        } else {
            self.bike.stop_wheelie();
        }
    }

    fn render(&mut self) {
        self.renderer.begin_frame();
        self.renderer.render_scene(&self.bike, &self.camera);
        self.renderer.render_ui(&mut self.ui, &self.bike);
        self.renderer.end_frame();
    }
}