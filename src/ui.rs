//! UI system using HTML/CSS overlay

use crate::bike::Bike;
use crate::camera::Camera;
use web_sys::{Document, HtmlElement, window, KeyboardEvent};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::Closure;

pub struct UI {
    document: Document,
    speed_element: Option<HtmlElement>,
    rpm_element: Option<HtmlElement>,
    gear_element: Option<HtmlElement>,
    wheelie_angle_element: Option<HtmlElement>,
    wheelie_time_element: Option<HtmlElement>,
    max_wheelie_angle_element: Option<HtmlElement>,
    best_wheelie_time_element: Option<HtmlElement>,
    best_wheelie_angle_element: Option<HtmlElement>,
    distance_element: Option<HtmlElement>,
    fps_element: Option<HtmlElement>,
    camera_mode_element: Option<HtmlElement>,
    drift_indicator: Option<HtmlElement>,
    wheelie_indicator: Option<HtmlElement>,
    throttle_bar: Option<HtmlElement>,
    brake_bar: Option<HtmlElement>,
    steering_bar: Option<HtmlElement>,
    help_overlay: Option<HtmlElement>,
    start_overlay: Option<HtmlElement>,
    pause_overlay: Option<HtmlElement>,
    on_start: Option<Closure<dyn FnMut(web_sys::MouseEvent)>>,
    #[allow(dead_code)]
    show_help: bool,
}

impl UI {
    pub fn new(document: &Document) -> Self {
        let mut ui = Self {
            document: document.clone(),
            speed_element: None,
            rpm_element: None,
            gear_element: None,
            wheelie_angle_element: None,
            wheelie_time_element: None,
            max_wheelie_angle_element: None,
            best_wheelie_time_element: None,
            best_wheelie_angle_element: None,
            distance_element: None,
            fps_element: None,
            camera_mode_element: None,
            drift_indicator: None,
            wheelie_indicator: None,
            throttle_bar: None,
            brake_bar: None,
            steering_bar: None,
            help_overlay: None,
            start_overlay: None,
            pause_overlay: None,
            on_start: None,
            show_help: true,
        };
        
        ui.create_ui_elements();
        ui
    }

    fn create_ui_elements(&mut self) {
        let body = self.document.body().unwrap();
        
        // Create HUD container
        let hud = self.document.create_element("div").unwrap();
        hud.set_id("hud");
        hud.set_class_name("hud");
        hud.set_inner_html(r#"
            <div class="hud-panel speed-panel">
                <span class="hud-label">SPEED</span>
                <span id="speed-value" class="hud-value">0</span>
                <span class="hud-unit">km/h</span>
            </div>
            <div class="hud-panel rpm-panel">
                <span class="hud-label">RPM</span>
                <span id="rpm-value" class="hud-value">0</span>
                <div id="rpm-bar" class="bar"><div class="bar-fill"></div></div>
            </div>
            <div class="hud-panel gear-panel">
                <span class="hud-label">GEAR</span>
                <span id="gear-value" class="hud-value">N</span>
            </div>
            <div class="hud-panel wheelie-panel">
                <span class="hud-label">WHEELIE</span>
                <span id="wheelie-angle" class="hud-value">0.0</span>
                <span class="hud-unit">°</span>
                <div id="wheelie-time" class="hud-sub">Time: <span>0.0</span>s</div>
                <div id="max-wheelie" class="hud-sub">Max: <span>0.0</span>°</div>
            </div>
            <div class="hud-panel stats-panel">
                <span class="hud-label">BEST</span>
                <div id="best-wheelie-time" class="hud-sub">Time: <span>0.0</span>s</div>
                <div id="best-wheelie-angle" class="hud-sub">Angle: <span>0.0</span>°</div>
            </div>
            <div class="hud-panel distance-panel">
                <span class="hud-label">DIST</span>
                <span id="distance-value" class="hud-value">0</span>
                <span class="hud-unit">m</span>
            </div>
            <div class="hud-panel camera-panel">
                <span class="hud-label">CAM</span>
                <span id="camera-mode" class="hud-value">THIRD</span>
            </div>
            <div class="hud-panel fps-panel">
                <span id="fps-value" class="hud-value">60</span>
                <span class="hud-unit">FPS</span>
            </div>
        "#);
        body.append_child(&hud).unwrap();

        // Input indicators
        let indicators = self.document.create_element("div").unwrap();
        indicators.set_id("input-indicators");
        indicators.set_class_name("input-indicators");
        indicators.set_inner_html(r#"
            <div class="indicator-panel">
                <div class="indicator" id="throttle-indicator">
                    <span class="indicator-label">THROTTLE</span>
                    <div class="vertical-bar"><div id="throttle-bar" class="vertical-bar-fill"></div></div>
                </div>
                <div class="indicator" id="brake-indicator">
                    <span class="indicator-label">BRAKE</span>
                    <div class="vertical-bar"><div id="brake-bar" class="vertical-bar-fill"></div></div>
                </div>
                <div class="indicator" id="steering-indicator">
                    <span class="indicator-label">STEER</span>
                    <div class="horizontal-bar"><div id="steering-bar" class="horizontal-bar-fill"></div></div>
                </div>
            </div>
            <div class="status-indicators">
                <div id="drift-indicator" class="status-indicator inactive">DRIFT</div>
                <div id="wheelie-indicator" class="status-indicator inactive">WHEELIE</div>
            </div>
        "#);
        body.append_child(&indicators).unwrap();

        // Overlay de pausa
        let pause = self.document.create_element("div").unwrap();
        pause.set_id("pause-overlay");
        pause.set_class_name("help-overlay hidden");
        pause.set_inner_html(r#"
            <div class="help-content">
                <h2>PAUSADO</h2>
                <div class="subtitle">Pressione ESC ou P para continuar</div>
            </div>
        "#);
        body.append_child(&pause).unwrap();

        // Tela inicial (start screen): botão JOGAR que inicializa áudio via gesto do usuário
        let start = self.document.create_element("div").unwrap();
        start.set_id("start-overlay");
        start.set_class_name("help-overlay");
        start.set_inner_html(r#"
            <div class="help-content">
                <h2>GRAU</h2>
                <div class="subtitle">Jogo de manobra (wheelie) com moto — 100% original</div>
                <div class="control-group">
                    <div class="control"><kbd>W</kbd>/<kbd>↑</kbd> Acelerar</div>
                    <div class="control"><kbd>S</kbd>/<kbd>↓</kbd> Frear/Ré</div>
                    <div class="control"><kbd>A</kbd>/<kbd>←</kbd> Esquerda</div>
                    <div class="control"><kbd>D</kbd>/<kbd>→</kbd> Direita</div>
                    <div class="control"><kbd>SPACE</kbd> Grau (wheelie)</div>
                    <div class="control"><kbd>CTRL</kbd> Drift</div>
                    <div class="control"><kbd>C</kbd> Câmera</div>
                    <div class="control"><kbd>R</kbd> Resetar</div>
                </div>
                <button id="start-btn" class="btn">Jogar</button>
                <p class="mobile-hint">Celular: toque na esquerda p/ virar, direita p/ acelerar/frear, canto inferior direito p/ grau</p>
            </div>
        "#);
        body.append_child(&start).unwrap();

        // Help overlay
        let help = self.document.create_element("div").unwrap();
        help.set_id("help-overlay");
        help.set_class_name("help-overlay hidden");
        help.set_inner_html(r#"
            <div class="help-content">
                <h2>GRAU - CONTROLS</h2>
                <div class="control-group">
                    <div class="control"><kbd>W</kbd> / <kbd>↑</kbd> Accelerate</div>
                    <div class="control"><kbd>S</kbd> / <kbd>↓</kbd> Brake/Reverse</div>
                    <div class="control"><kbd>A</kbd> / <kbd>←</kbd> Steer Left</div>
                    <div class="control"><kbd>D</kbd> / <kbd>→</kbd> Steer Right</div>
                    <div class="control"><kbd>SPACE</kbd> / <kbd>SHIFT</kbd> Wheelie</div>
                    <div class="control"><kbd>CTRL</kbd> Drift</div>
                    <div class="control"><kbd>C</kbd> Camera Mode</div>
                    <div class="control"><kbd>R</kbd> Reset Bike</div>
                    <div class="control"><kbd>H</kbd> Toggle Help</div>
                </div>
                <p class="mobile-hint">Mobile: Touch left side to steer, right side for throttle/brake, bottom-right for wheelie</p>
                <button id="close-help" class="btn">Got it!</button>
            </div>
        "#);
        body.append_child(&help).unwrap();

        // Cache elements
        self.speed_element = self.document.get_element_by_id("speed-value").and_then(|e| e.dyn_into::<HtmlElement>().ok());
        self.rpm_element = self.document.get_element_by_id("rpm-value").and_then(|e| e.dyn_into::<HtmlElement>().ok());
        self.gear_element = self.document.get_element_by_id("gear-value").and_then(|e| e.dyn_into::<HtmlElement>().ok());
        self.wheelie_angle_element = self.document.get_element_by_id("wheelie-angle").and_then(|e| e.dyn_into::<HtmlElement>().ok());
        self.wheelie_time_element = self.document.query_selector("#wheelie-time span").unwrap().and_then(|e| e.dyn_into::<HtmlElement>().ok());
        self.max_wheelie_angle_element = self.document.query_selector("#max-wheelie span").unwrap().and_then(|e| e.dyn_into::<HtmlElement>().ok());
        self.best_wheelie_time_element = self.document.query_selector("#best-wheelie-time span").unwrap().and_then(|e| e.dyn_into::<HtmlElement>().ok());
        self.best_wheelie_angle_element = self.document.query_selector("#best-wheelie-angle span").unwrap().and_then(|e| e.dyn_into::<HtmlElement>().ok());
        self.distance_element = self.document.get_element_by_id("distance-value").and_then(|e| e.dyn_into::<HtmlElement>().ok());
        self.fps_element = self.document.get_element_by_id("fps-value").and_then(|e| e.dyn_into::<HtmlElement>().ok());
        self.camera_mode_element = self.document.get_element_by_id("camera-mode").and_then(|e| e.dyn_into::<HtmlElement>().ok());
        self.drift_indicator = self.document.get_element_by_id("drift-indicator").and_then(|e| e.dyn_into::<HtmlElement>().ok());
        self.wheelie_indicator = self.document.get_element_by_id("wheelie-indicator").and_then(|e| e.dyn_into::<HtmlElement>().ok());
        self.throttle_bar = self.document.get_element_by_id("throttle-bar").and_then(|e| e.dyn_into::<HtmlElement>().ok());
        self.brake_bar = self.document.get_element_by_id("brake-bar").and_then(|e| e.dyn_into::<HtmlElement>().ok());
        self.steering_bar = self.document.get_element_by_id("steering-bar").and_then(|e| e.dyn_into::<HtmlElement>().ok());
        self.help_overlay = self.document.get_element_by_id("help-overlay").and_then(|e| e.dyn_into::<HtmlElement>().ok());
        self.start_overlay = self.document.get_element_by_id("start-overlay").and_then(|e| e.dyn_into::<HtmlElement>().ok());
        self.pause_overlay = self.document.get_element_by_id("pause-overlay").and_then(|e| e.dyn_into::<HtmlElement>().ok());

        // Setup start button (gesto do usuário: aqui o AudioContext pode iniciar)
        if let Some(start_btn) = self.document.get_element_by_id("start-btn") {
            let start_overlay = self.start_overlay.clone();
            let closure = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
                if let Some(overlay) = &start_overlay {
                    overlay.set_class_name("help-overlay hidden");
                }
                // Callback externo (inicializa áudio) é disparado via evento customizado
                if let Some(win) = window() {
                    let event = web_sys::Event::new("grau-start").ok();
                    if let Some(ev) = event {
                        win.dispatch_event(&ev).ok();
                    }
                }
            }) as Box<dyn FnMut(web_sys::MouseEvent)>);
            start_btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).unwrap();
            self.on_start = Some(closure);
        }

        // Setup help toggle
        let help_overlay = self.help_overlay.clone();
        if let Some(close_btn) = self.document.get_element_by_id("close-help") {
            let help_overlay = help_overlay.clone();
            let closure = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
                if let Some(overlay) = &help_overlay {
                    overlay.set_class_name("help-overlay hidden");
                }
            }) as Box<dyn FnMut(web_sys::MouseEvent)>);
            close_btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).unwrap();
            closure.forget();
        }

        // Add H key listener for help toggle
        let help_overlay = help_overlay.clone();
        let h_closure = Closure::wrap(Box::new(move |event: KeyboardEvent| {
            if event.code() == "KeyH" {
                if let Some(overlay) = &help_overlay {
                    let class = overlay.class_name();
                    if class.contains("hidden") {
                        overlay.set_class_name("help-overlay");
                    } else {
                        overlay.set_class_name("help-overlay hidden");
                    }
                }
            }
        }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);
        
        if let Some(window) = window() {
            window.add_event_listener_with_callback("keydown", h_closure.as_ref().unchecked_ref()).unwrap();
        }
        h_closure.forget();
    }

    pub fn update(&mut self, bike: &Bike, camera: &Camera) {
        // Speed
        if let Some(el) = &self.speed_element {
            el.set_text_content(Some(&format!("{:.0}", bike.speed * 3.6))); // m/s to km/h
        }

        // RPM (simulated)
        if let Some(el) = &self.rpm_element {
            let rpm = (bike.speed * 50.0 + 800.0) as u32;
            el.set_text_content(Some(&rpm.to_string()));
        }

        // Gear (simulated)
        if let Some(el) = &self.gear_element {
            let gear = if bike.speed < 5.0 { "N" } 
                else if bike.speed < 15.0 { "1" }
                else if bike.speed < 30.0 { "2" }
                else if bike.speed < 50.0 { "3" }
                else if bike.speed < 75.0 { "4" }
                else if bike.speed < 100.0 { "5" }
                else { "6" };
            el.set_text_content(Some(gear));
        }

        // Wheelie angle
        if let Some(el) = &self.wheelie_angle_element {
            el.set_text_content(Some(&format!("{:.1}", bike.wheelie_angle)));
        }

        // Wheelie time
        if let Some(el) = &self.wheelie_time_element {
            el.set_text_content(Some(&format!("{:.1}", bike.wheelie_time)));
        }

        // Max wheelie angle
        if let Some(el) = &self.max_wheelie_angle_element {
            el.set_text_content(Some(&format!("{:.1}", bike.max_wheelie_angle)));
        }

        // Best wheelie time
        if let Some(el) = &self.best_wheelie_time_element {
            el.set_text_content(Some(&format!("{:.1}", bike.best_wheelie_time)));
        }

        // Best wheelie angle
        if let Some(el) = &self.best_wheelie_angle_element {
            el.set_text_content(Some(&format!("{:.1}", bike.best_wheelie_angle)));
        }

        // Distance
        if let Some(el) = &self.distance_element {
            el.set_text_content(Some(&format!("{:.0}", bike.total_distance)));
        }

        // Camera mode
        if let Some(el) = &self.camera_mode_element {
            let mode_str = match camera.mode {
                crate::camera::CameraMode::ThirdPerson => "THIRD",
                crate::camera::CameraMode::FirstPerson => "FIRST",
                crate::camera::CameraMode::Hood => "HOOD",
                crate::camera::CameraMode::Cinematic => "CINE",
                crate::camera::CameraMode::Drone => "DRONE",
            };
            el.set_text_content(Some(mode_str));
        }
    }

    pub fn update_dom(&mut self, bike: &Bike) {
        // Barras de input atualizadas a cada frame a partir do estado da moto
        self.set_throttle(bike.throttle);
        self.set_brake(bike.brake);
        self.set_steering(bike.steering_input);
        self.set_drift_active(bike.is_drift_active);
        self.set_wheelie_active(bike.wheelie_angle > 5.0);
    }

    /// Esconde a tela inicial (após o clique em JOGAR).
    #[allow(dead_code)]
    pub fn hide_start(&mut self) {
        if let Some(overlay) = &self.start_overlay {
            overlay.set_class_name("help-overlay hidden");
        }
    }

    /// Mostra/esconde o overlay de pausa.
    pub fn set_paused(&mut self, paused: bool) {
        if let Some(overlay) = &self.pause_overlay {
            if paused {
                overlay.set_class_name("help-overlay");
            } else {
                overlay.set_class_name("help-overlay hidden");
            }
        }
    }

    pub fn update_fps(&mut self, fps: f32) {
        if let Some(el) = &self.fps_element {
            el.set_text_content(Some(&format!("{:.0}", fps)));
        }
    }

    pub fn set_throttle(&mut self, value: f32) {
        if let Some(el) = &self.throttle_bar {
            if let Some(html_el) = el.dyn_ref::<HtmlElement>() {
                let style = html_el.style();
                style.set_property("height", &format!("{}%", value * 100.0)).unwrap();
            }
        }
    }

    pub fn set_brake(&mut self, value: f32) {
        if let Some(el) = &self.brake_bar {
            if let Some(html_el) = el.dyn_ref::<HtmlElement>() {
                let style = html_el.style();
                style.set_property("height", &format!("{}%", value * 100.0)).unwrap();
            }
        }
    }

    pub fn set_steering(&mut self, value: f32) {
        if let Some(el) = &self.steering_bar {
            if let Some(html_el) = el.dyn_ref::<HtmlElement>() {
                let style = html_el.style();
                let translate = value * 50.0; // -50% to 50%
                style.set_property("transform", &format!("translateX({}%)", translate)).unwrap();
            }
        }
    }

    pub fn set_drift_active(&mut self, active: bool) {
        if let Some(el) = &self.drift_indicator {
            if active {
                el.set_class_name("status-indicator active");
            } else {
                el.set_class_name("status-indicator inactive");
            }
        }
    }

    pub fn set_wheelie_active(&mut self, active: bool) {
        if let Some(el) = &self.wheelie_indicator {
            if active {
                el.set_class_name("status-indicator active");
            } else {
                el.set_class_name("status-indicator inactive");
            }
        }
    }
}