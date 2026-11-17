//! Physics engine for motorcycle simulation

use crate::bike::Bike;
use crate::input::InputManager;
use nalgebra::{Vector3, UnitQuaternion, Unit};

pub struct PhysicsEngine {
    gravity: Vector3<f32>,
    // Bike physics config (from reverse-engineered values)
    config: BikePhysicsConfig,
}

#[derive(Clone, Copy)]
#[allow(dead_code)] // vários parâmetros são mantidos para calibração futura da física
pub struct BikePhysicsConfig {
    // Wheelie physics
    pub wheelie_torque: f32,
    pub wheelie_drag: f32,
    pub wheelie_angular_multiplier: f32,
    pub wheelie_brake_force: f32,
    pub critical_angle: f32,
    pub wheelie_speed: f32,
    pub wheelie_return_speed: f32,
    
    // Drift physics
    pub drift_sideways_force: f32,
    pub drift_rear_friction: f32,
    pub drift_front_friction: f32,
    pub drift_braking_factor: f32,
    pub drift_rotation_torque: f32,
    pub drift_angular_drag: f32,
    pub min_speed_for_drift: f32,
    
    // Zerinho (donut) physics
    pub zerinho_rotation_torque: f32,
    pub zerinho_rear_friction: f32,
    pub zerinho_angular_drag: f32,
    pub steer_input_for_zerinho_threshold: f32,
    
    // Force reverse
    pub force_reverse: f32,
    pub angulo_force_reverse: f32,
    
    // Sustentation (wheelie support force)
    pub forca_sustentacao_grau: f32,
    
    // General
    pub max_steer_angle: f32,
    pub max_speed: f32,
    pub engine_power: f32,
    pub brake_power: f32,
    pub mass: f32,
    pub wheelbase: f32,
    pub center_of_mass_height: f32,
}

impl Default for BikePhysicsConfig {
    fn default() -> Self {
        Self {
            // From reverse-engineered BikePhysicsConfig
            forca_sustentacao_grau: 150.0,
            angulo_force_reverse: 45.0,
            force_reverse: 200.0,
            zerinho_rotation_torque: 500.0,
            steer_input_for_zerinho_threshold: 0.8,
            zerinho_rear_friction: 0.3,
            zerinho_angular_drag: 2.0,
            min_speed_for_drift: 10.0,
            drift_sideways_force: 300.0,
            drift_rear_friction: 0.4,
            drift_front_friction: 0.8,
            drift_braking_factor: 0.5,
            drift_rotation_torque: 200.0,
            drift_angular_drag: 1.5,
            wheelie_torque: 1000.0,
            wheelie_drag: 0.1,
            wheelie_angular_multiplier: 1.5,
            wheelie_brake_force: 500.0,
            critical_angle: 75.0,
            wheelie_speed: 30.0,
            wheelie_return_speed: 5.0,
            
            // General
            max_steer_angle: 45.0,
            max_speed: 120.0,
            engine_power: 1500.0,
            brake_power: 2000.0,
            mass: 250.0,
            wheelbase: 1.4,
            center_of_mass_height: 0.6,
        }
    }
}

impl PhysicsEngine {
    pub fn new() -> Self {
        Self {
            gravity: Vector3::new(0.0, -9.81, 0.0),
            config: BikePhysicsConfig::default(),
        }
    }

    pub fn update(&mut self, bike: &mut Bike, dt: f32, _input: &InputManager) {
        // Apply forces
        self.apply_engine_forces(bike, dt);
        self.apply_steering(bike, dt);
        self.apply_wheelie_physics(bike, dt);
        self.apply_drift_physics(bike, dt);
        self.apply_zerinho_physics(bike, dt);
        self.apply_gravity(bike, dt);
        self.apply_drag(bike, dt);
        
        // Integrate
        self.integrate(bike, dt);
        
        // Ground collision
        self.ground_collision(bike);
        
        // Update bike state
        bike.update_state(dt);
    }

    fn apply_engine_forces(&self, bike: &mut Bike, _dt: f32) {
        let forward = bike.transform.rotation * Vector3::new(0.0, 0.0, 1.0);
        
        // Throttle
        if bike.throttle > 0.0 {
            let force = forward * bike.throttle * self.config.engine_power;
            bike.add_force(force);
        }
        
        // Brake
        if bike.brake > 0.0 {
            let velocity = bike.velocity;
            let speed = velocity.norm();
            if speed > 0.1 {
                let brake_force = -velocity.normalize() * bike.brake * self.config.brake_power;
                bike.add_force(brake_force);
            }
        }
        
        // Engine braking when no throttle
        if bike.throttle == 0.0 && bike.brake == 0.0 && bike.velocity.norm() > 0.1 {
            let drag = -bike.velocity.normalize() * 50.0;
            bike.add_force(drag);
        }
    }

    fn apply_steering(&self, bike: &mut Bike, dt: f32) {
        if bike.steering_input != 0.0 && bike.speed > 1.0 {
            let steer_angle = bike.steering_input * self.config.max_steer_angle.to_radians();
            let angular_velocity = bike.speed * steer_angle.tan() / self.config.wheelbase;
            
            // Apply yaw rotation
            let yaw_rotation = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), angular_velocity * dt);
            bike.transform.rotation = yaw_rotation * bike.transform.rotation;
        }
    }

    fn apply_wheelie_physics(&self, bike: &mut Bike, _dt: f32) {
        if !bike.is_wheelie_active {
            return;
        }

        let pitch_angle = bike.transform.rotation.euler_angles().0; // pitch in radians
        let pitch_deg = pitch_angle.to_degrees();

        if pitch_deg < self.config.critical_angle {
            // Apply wheelie torque
            let right = bike.transform.rotation * Vector3::new(1.0, 0.0, 0.0);
            let torque = right * self.config.wheelie_torque * (1.0 - pitch_deg / self.config.critical_angle);
            bike.add_torque(torque);
            
            // Sustentation force (upward force to maintain wheelie)
            let up = Vector3::y();
            let sustentation = up * self.config.forca_sustentacao_grau * (pitch_deg / self.config.critical_angle);
            bike.add_force(sustentation);
        } else {
            // Past critical angle - apply force reverse to prevent flipping
            let forward = bike.transform.rotation * Vector3::new(0.0, 0.0, 1.0);
            let reverse_force = -forward * self.config.force_reverse * ((pitch_deg - self.config.critical_angle) / 10.0);
            bike.add_force(reverse_force);
            
            // Strong angular drag to pull back
            let right = bike.transform.rotation * Vector3::new(1.0, 0.0, 0.0);
            let drag_torque = -right * self.config.wheelie_angular_multiplier * (pitch_deg - self.config.critical_angle);
            bike.add_torque(drag_torque);
        }

        // Wheelie brake force when throttle released
        if bike.throttle == 0.0 && pitch_deg > 10.0 {
            let right = bike.transform.rotation * Vector3::new(1.0, 0.0, 0.0);
            let brake_torque = -right * self.config.wheelie_brake_force;
            bike.add_torque(brake_torque);
        }
    }

    fn apply_drift_physics(&self, bike: &mut Bike, dt: f32) {
        if !bike.is_drift_active || bike.speed < self.config.min_speed_for_drift {
            return;
        }

        let right = bike.transform.rotation * Vector3::new(1.0, 0.0, 0.0);
        let forward = bike.transform.rotation * Vector3::new(0.0, 0.0, 1.0);
        let velocity = bike.velocity;
        let speed = velocity.norm();
        
        if speed < 0.1 {
            return;
        }

        let velocity_dir = velocity.normalize();
        let _drift_angle = velocity_dir.dot(&right).abs();

        // Sideways force for drift
        let sideways_force = right * self.config.drift_sideways_force * bike.steering_input.signum() * dt;
        bike.add_force(sideways_force);

        // Reduced rear friction, increased front friction
        let lateral_vel = velocity_dir.dot(&right) * right;
        let rear_friction_force = -lateral_vel * self.config.drift_rear_friction * speed * bike.mass;
        bike.add_force(rear_friction_force);

        // Drift rotation torque
        let yaw_torque = Vector3::y() * self.config.drift_rotation_torque * bike.steering_input * dt;
        bike.add_torque(yaw_torque);

        // Angular drag during drift
        let angular_drag = -bike.angular_velocity * self.config.drift_angular_drag * dt;
        bike.add_torque(angular_drag);

        // Braking factor
        let brake_force = -forward * self.config.drift_braking_factor * speed * bike.mass * dt;
        bike.add_force(brake_force);
    }

    fn apply_zerinho_physics(&self, bike: &mut Bike, dt: f32) {
        // Zerinho (donut/spin) - high steering input at low speed
        if bike.steering_input.abs() > self.config.steer_input_for_zerinho_threshold && bike.speed < 5.0 {
            let up = Vector3::y();
            let spin_torque = up * self.config.zerinho_rotation_torque * bike.steering_input * dt;
            bike.add_torque(spin_torque);

            // Rear friction reduction for spinning
            let rear_friction = -bike.velocity * self.config.zerinho_rear_friction * dt;
            bike.add_force(rear_friction);

            // Angular drag
            let angular_drag = -bike.angular_velocity * self.config.zerinho_angular_drag * dt;
            bike.add_torque(angular_drag);
        }
    }

    fn apply_gravity(&self, bike: &mut Bike, dt: f32) {
        bike.add_force(self.gravity * bike.mass * dt);
    }

    fn apply_drag(&self, bike: &mut Bike, dt: f32) {
        let speed = bike.velocity.norm();
        if speed > 0.1 {
            let drag_force = -bike.velocity.normalize() * speed * speed * 0.3 * dt;
            bike.add_force(drag_force);
            
            // Angular drag
            let angular_drag = -bike.angular_velocity * 0.1 * dt;
            bike.add_torque(angular_drag);
        }
    }

    fn integrate(&self, bike: &mut Bike, dt: f32) {
        // Linear integration
        bike.velocity += bike.accumulated_force / bike.mass * dt;
        bike.transform.position += bike.velocity * dt;
        
        // Angular integration
        let inertia_tensor = self.compute_inertia_tensor(bike);
        let angular_accel = inertia_tensor.try_inverse().unwrap_or_default() * bike.accumulated_torque;
        bike.angular_velocity += angular_accel * dt;
        
        // Update rotation from angular velocity
        if bike.angular_velocity.norm() > 0.001 {
            let angle = bike.angular_velocity.norm() * dt;
            let axis = bike.angular_velocity.normalize();
            let rotation = UnitQuaternion::from_axis_angle(&Unit::new_normalize(axis), angle);
            bike.transform.rotation = rotation * bike.transform.rotation;
        }

        // Clear accumulators
        bike.accumulated_force = Vector3::zeros();
        bike.accumulated_torque = Vector3::zeros();
        
        // Update speed
        bike.speed = bike.velocity.norm();
    }

    fn compute_inertia_tensor(&self, bike: &Bike) -> nalgebra::Matrix3<f32> {
        // Simplified inertia tensor for a motorcycle
        let m = bike.mass;
        let w = 0.8; // width
        let l = self.config.wheelbase; // length
        let h = 1.0; // height
        
        let ix = m / 12.0 * (w * w + h * h);
        let iy = m / 12.0 * (l * l + h * h);
        let iz = m / 12.0 * (l * l + w * w);
        
        nalgebra::Matrix3::new(ix, 0.0, 0.0,
                               0.0, iy, 0.0,
                               0.0, 0.0, iz)
    }

    fn ground_collision(&self, bike: &mut Bike) {
        // Simple ground plane at y = 0
        let wheel_radius = 0.35;
        let min_y = wheel_radius;
        
        if bike.transform.position.y < min_y {
            bike.transform.position.y = min_y;
            bike.velocity.y = 0.0;
            bike.on_ground = true;
        } else {
            bike.on_ground = false;
        }
    }
}