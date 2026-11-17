//! Bike entity and components

use nalgebra::{Vector3, UnitQuaternion};

#[derive(Debug, Clone)]
pub struct Transform {
    pub position: Vector3<f32>,
    pub rotation: UnitQuaternion<f32>,
    #[allow(dead_code)]
    pub scale: Vector3<f32>,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Vector3::new(0.0, 0.5, 0.0),
            rotation: UnitQuaternion::identity(),
            scale: Vector3::new(1.0, 1.0, 1.0),
        }
    }
}

impl Transform {
    pub fn forward(&self) -> Vector3<f32> {
        self.rotation * Vector3::new(0.0, 0.0, 1.0)
    }
    
    pub fn right(&self) -> Vector3<f32> {
        self.rotation * Vector3::new(1.0, 0.0, 0.0)
    }
    
    pub fn up(&self) -> Vector3<f32> {
        self.rotation * Vector3::new(0.0, 1.0, 0.0)
    }
    
    pub fn euler_angles(&self) -> (f32, f32, f32) {
        self.rotation.euler_angles()
    }
}

#[derive(Debug, Clone)]
pub struct Bike {
    pub transform: Transform,
    pub velocity: Vector3<f32>,
    pub angular_velocity: Vector3<f32>,
    pub accumulated_force: Vector3<f32>,
    pub accumulated_torque: Vector3<f32>,
    
    // Input state
    pub throttle: f32,
    pub brake: f32,
    pub steering_input: f32,
    
    // Physics properties
    pub mass: f32,
    pub wheelbase: f32,
    
    // State
    pub speed: f32,
    pub on_ground: bool,
    pub is_wheelie_active: bool,
    pub is_drift_active: bool,
    pub wheelie_angle: f32,
    #[allow(dead_code)]
    pub drift_angle: f32,
    
    // Visual
    pub lean_angle: f32,
    pub wheel_rotation: f32,
    
    // Score/stats
    pub wheelie_time: f32,
    pub max_wheelie_angle: f32,
    pub total_distance: f32,
    pub best_wheelie_time: f32,
    pub best_wheelie_angle: f32,

    // Dano visual/funcional (0 = novo, 1 = destruído)
    #[allow(dead_code)]
    pub damage: f32,
    // Se a moto tombou (queda) — usado para reset suave
    pub fallen: bool,
}

impl Default for Bike {
    fn default() -> Self {
        Self::new()
    }
}

impl Bike {
    pub fn new() -> Self {
        Self {
            transform: Transform::default(),
            velocity: Vector3::zeros(),
            angular_velocity: Vector3::zeros(),
            accumulated_force: Vector3::zeros(),
            accumulated_torque: Vector3::zeros(),
            throttle: 0.0,
            brake: 0.0,
            steering_input: 0.0,
            mass: 250.0,
            wheelbase: 1.4,
            speed: 0.0,
            on_ground: true,
            is_wheelie_active: false,
            is_drift_active: false,
            wheelie_angle: 0.0,
            drift_angle: 0.0,
            lean_angle: 0.0,
            wheel_rotation: 0.0,
            wheelie_time: 0.0,
            max_wheelie_angle: 0.0,
            total_distance: 0.0,
            best_wheelie_time: 0.0,
            best_wheelie_angle: 0.0,
            damage: 0.0,
            fallen: false,
        }
    }

    pub fn apply_throttle(&mut self, value: f32) {
        self.throttle = value.clamp(0.0, 1.0);
    }

    pub fn apply_brake(&mut self, value: f32) {
        self.brake = value.clamp(0.0, 1.0);
    }

    pub fn apply_steering(&mut self, value: f32) {
        self.steering_input = value.clamp(-1.0, 1.0);
    }

    pub fn start_wheelie(&mut self) {
        if self.on_ground && self.speed > 5.0 {
            self.is_wheelie_active = true;
        }
    }

    pub fn stop_wheelie(&mut self) {
        self.is_wheelie_active = false;
    }

    pub fn start_drift(&mut self) {
        if self.on_ground && self.speed > 10.0 {
            self.is_drift_active = true;
        }
    }

    pub fn stop_drift(&mut self) {
        self.is_drift_active = false;
    }

    pub fn add_force(&mut self, force: Vector3<f32>) {
        self.accumulated_force += force;
    }

    pub fn add_torque(&mut self, torque: Vector3<f32>) {
        self.accumulated_torque += torque;
    }

    pub fn update_state(&mut self, dt: f32) {
        // Update wheelie angle from pitch
        let (pitch, _, _) = self.transform.euler_angles();
        self.wheelie_angle = pitch.to_degrees().abs();
        
        if self.is_wheelie_active {
            self.wheelie_time += dt;
            self.max_wheelie_angle = self.max_wheelie_angle.max(self.wheelie_angle);
            self.best_wheelie_time = self.best_wheelie_time.max(self.wheelie_time);
            self.best_wheelie_angle = self.best_wheelie_angle.max(self.max_wheelie_angle);
        } else {
            self.wheelie_time = 0.0;
            self.max_wheelie_angle = 0.0;
        }

        // Update lean angle for visual
        let (_, _, roll) = self.transform.euler_angles();
        self.lean_angle = roll.to_degrees();

        // Wheel rotation
        self.wheel_rotation += self.speed * dt * 10.0;

        // Distance
        self.total_distance += self.speed * dt;

        // Detecta tombo: inclinação lateral (roll) grande com pouca velocidade
        // ou ângulo de grau acima do crítico (75°) conta como "capotou p/ trás"
        let (pitch, _, roll) = self.transform.euler_angles();
        let roll_deg = roll.to_degrees().abs();
        let pitch_deg = pitch.to_degrees().abs();
        if (roll_deg > 70.0 && self.speed < 3.0) || pitch_deg > 100.0 {
            self.fallen = true;
        }
    }

    /// Aplica dano e marca tombo se necessário. Retorna true se acabou de tombar.
    #[allow(dead_code)]
    pub fn apply_damage(&mut self, amount: f32) -> bool {
        self.damage = (self.damage + amount).clamp(0.0, 1.0);
        if self.damage >= 1.0 && !self.fallen {
            self.fallen = true;
            return true;
        }
        false
    }

    pub fn reset(&mut self) {
        // Preserva recordes (best_*) entre resets
        let best_time = self.best_wheelie_time;
        let best_angle = self.best_wheelie_angle;
        *self = Self::new();
        self.best_wheelie_time = best_time;
        self.best_wheelie_angle = best_angle;
    }

    /// Persiste recordes no localStorage (best angle/time). Chamado ao encerrar sessão.
    pub fn save_records(&self) {
        if let Some(win) = web_sys::window() {
            if let Ok(Some(storage)) = win.local_storage() {
                let _ = storage.set_item("grau_best_time", &format!("{:.2}", self.best_wheelie_time));
                let _ = storage.set_item("grau_best_angle", &format!("{:.2}", self.best_wheelie_angle));
            }
        }
    }

    /// Carrega recordes do localStorage, se existirem.
    pub fn load_records(&mut self) {
        if let Some(win) = web_sys::window() {
            if let Ok(Some(storage)) = win.local_storage() {
                if let Ok(Some(t)) = storage.get_item("grau_best_time") {
                    self.best_wheelie_time = t.parse().unwrap_or(0.0);
                }
                if let Ok(Some(a)) = storage.get_item("grau_best_angle") {
                    self.best_wheelie_angle = a.parse().unwrap_or(0.0);
                }
            }
        }
    }

    /// Salva e zera? não — apenas salva. Mantido simples.

    pub fn get_wheel_positions(&self) -> (Vector3<f32>, Vector3<f32>) {
        let front_offset = self.transform.forward() * (self.wheelbase / 2.0);
        let rear_offset = -self.transform.forward() * (self.wheelbase / 2.0);
        let up_offset = Vector3::new(0.0, 0.35, 0.0); // wheel radius
        
        let front = self.transform.position + front_offset - up_offset;
        let rear = self.transform.position + rear_offset - up_offset;
        
        (front, rear)
    }
}