//! Camera system

use crate::bike::Bike;
use nalgebra::{Matrix4, Vector3, Point3, Perspective3};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CameraMode {
    ThirdPerson,
    FirstPerson,
    Hood,
    Cinematic,
    Drone,
}

pub struct Camera {
    pub mode: CameraMode,
    pub position: Vector3<f32>,
    pub target: Vector3<f32>,
    pub up: Vector3<f32>,
    pub fov: f32,
    pub near: f32,
    pub far: f32,
    
    // Third person settings
    pub distance: f32,
    pub height: f32,
    pub angle: f32,
    pub smoothness: f32,
    
    // Shake
    pub shake_intensity: f32,
    pub shake_decay: f32,
    pub shake_timer: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self::new()
    }
}

impl Camera {
    pub fn new() -> Self {
        Self {
            mode: CameraMode::ThirdPerson,
            position: Vector3::new(0.0, 3.0, -8.0),
            target: Vector3::new(0.0, 1.0, 0.0),
            up: Vector3::y(),
            fov: 70.0,
            near: 0.1,
            far: 1000.0,
            distance: 8.0,
            height: 2.5,
            angle: 0.0,
            smoothness: 0.1,
            shake_intensity: 0.0,
            shake_decay: 0.9,
            shake_timer: 0.0,
        }
    }

    pub fn update(&mut self, bike: &Bike, dt: f32) {
        match self.mode {
            CameraMode::ThirdPerson => self.update_third_person(bike, dt),
            CameraMode::FirstPerson => self.update_first_person(bike, dt),
            CameraMode::Hood => self.update_hood(bike, dt),
            CameraMode::Cinematic => self.update_cinematic(bike, dt),
            CameraMode::Drone => self.update_drone(bike, dt),
        }
        
        // Apply shake
        if self.shake_timer > 0.0 {
            self.shake_timer -= dt;
            let shake = (rand::random::<f32>() - 0.5) * self.shake_intensity * 2.0;
            self.position += Vector3::new(shake, shake * 0.5, shake);
            self.shake_intensity *= self.shake_decay;
        }
    }

    fn update_third_person(&mut self, bike: &Bike, dt: f32) {
        let bike_pos = bike.transform.position;
        let bike_forward = bike.transform.forward();
        let bike_up = bike.transform.up();
        let bike_right = bike.transform.right();
        
        // Calculate desired camera position behind bike
        let desired_pos = bike_pos 
            - bike_forward * self.distance 
            + bike_up * self.height
            + bike_right * self.angle;
        
        // Smooth interpolation
        self.position = self.position.lerp(&desired_pos, self.smoothness * dt * 60.0);
        self.target = bike_pos + bike_up * 1.0;
        
        // Add wheelie tilt compensation
        if bike.is_wheelie_active {
            let tilt_compensation = bike.wheelie_angle.to_radians() * 0.5;
            self.position.y += tilt_compensation;
        }
    }

    fn update_first_person(&mut self, bike: &Bike, dt: f32) {
        let bike_pos = bike.transform.position;
        let bike_forward = bike.transform.forward();
        let bike_up = bike.transform.up();
        
        // Camera at rider's head position
        let head_offset = bike_up * 1.3 + bike_forward * 0.3;
        let desired_pos = bike_pos + head_offset;
        
        self.position = self.position.lerp(&desired_pos, self.smoothness * dt * 60.0 * 5.0);
        self.target = self.position + bike_forward;
    }

    fn update_hood(&mut self, bike: &Bike, dt: f32) {
        let bike_pos = bike.transform.position;
        let bike_forward = bike.transform.forward();
        let bike_up = bike.transform.up();
        
        // Camera on hood
        let hood_offset = bike_up * 0.8 + bike_forward * 1.5;
        let desired_pos = bike_pos + hood_offset;
        
        self.position = self.position.lerp(&desired_pos, self.smoothness * dt * 60.0 * 3.0);
        self.target = bike_pos + bike_forward * 10.0 + bike_up * 0.5;
    }

    fn update_cinematic(&mut self, bike: &Bike, dt: f32) {
        // Cinematic camera - orbits around bike
        self.angle += dt * 0.3;
        let radius = 15.0;
        let height = 5.0;
        
        let bike_pos = bike.transform.position;
        self.position = bike_pos + Vector3::new(
            radius * self.angle.sin(),
            height,
            radius * self.angle.cos()
        );
        self.target = bike_pos + Vector3::y() * 1.0;
    }

    fn update_drone(&mut self, bike: &Bike, dt: f32) {
        // Drone camera - follows from above and behind
        let bike_pos = bike.transform.position;
        let bike_forward = bike.transform.forward();
        
        let desired_pos = bike_pos + Vector3::new(0.0, 20.0, 0.0) - bike_forward * 10.0;
        self.position = self.position.lerp(&desired_pos, self.smoothness * dt * 60.0);
        self.target = bike_pos;
    }

    pub fn toggle_mode(&mut self) {
        self.mode = match self.mode {
            CameraMode::ThirdPerson => CameraMode::FirstPerson,
            CameraMode::FirstPerson => CameraMode::Hood,
            CameraMode::Hood => CameraMode::Cinematic,
            CameraMode::Cinematic => CameraMode::Drone,
            CameraMode::Drone => CameraMode::ThirdPerson,
        };
    }

    pub fn add_shake(&mut self, intensity: f32, duration: f32) {
        self.shake_intensity = intensity;
        self.shake_timer = duration;
    }

    pub fn get_view_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_at_rh(
            &Point3::from(self.position),
            &Point3::from(self.target),
            &self.up,
        )
    }

    pub fn get_projection_matrix(&self, aspect: f32) -> Matrix4<f32> {
        Perspective3::new(aspect, self.fov.to_radians(), self.near, self.far).into()
    }
}

use rand;