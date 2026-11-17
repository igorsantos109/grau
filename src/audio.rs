//! Audio system using Web Audio API - Simplified version

use crate::bike::Bike;
use web_sys::{AudioContext, GainNode, OscillatorNode, OscillatorType, AudioNode, AudioDestinationNode, AudioParam};

pub struct AudioManager {
    context: Option<AudioContext>,
    master_gain: Option<GainNode>,
    engine_oscillator: Option<OscillatorNode>,
    engine_gain: Option<GainNode>,
    wind_oscillator: Option<OscillatorNode>,
    wind_gain: Option<GainNode>,
    tire_screech_oscillator: Option<OscillatorNode>,
    tire_screech_gain: Option<GainNode>,
    initialized: bool,
}

impl AudioManager {
    pub fn new() -> Self {
        Self {
            context: None,
            master_gain: None,
            engine_oscillator: None,
            engine_gain: None,
            wind_oscillator: None,
            wind_gain: None,
            tire_screech_oscillator: None,
            tire_screech_gain: None,
            initialized: false,
        }
    }

    /// Inicializa o AudioContext. Deve ser chamado a partir de um gesto do usuário
    /// (clique/tecla) para satisfazer a política de autoplay do navegador.
    pub fn init(&mut self) {
        if self.initialized {
            return;
        }

        if let Ok(context) = AudioContext::new() {
            // Tenta retomar imediatamente (funciona quando há gesto do usuário)
            let _ = context.resume();
            self.context = Some(context.clone());
            
            // Master gain
            if let Ok(master_gain) = context.create_gain() {
                let gain_param = master_gain.gain();
                gain_param.set_value(0.3);
                let dest = context.destination();
                master_gain.connect_with_audio_node(&dest).ok();
                self.master_gain = Some(master_gain);
            }

            // Engine sound (low frequency oscillator)
            if let Ok(engine_osc) = context.create_oscillator() {
                engine_osc.set_type(OscillatorType::Sawtooth);
                let freq_param = engine_osc.frequency();
                freq_param.set_value(60.0);
                
                if let Ok(engine_gain) = context.create_gain() {
                    let gain_param = engine_gain.gain();
                    gain_param.set_value(0.0);
                    engine_osc.connect_with_audio_node(&engine_gain).ok();
                    if let Some(master) = &self.master_gain {
                        engine_gain.connect_with_audio_node(master).ok();
                    }
                    engine_osc.start().ok();
                    
                    self.engine_oscillator = Some(engine_osc);
                    self.engine_gain = Some(engine_gain);
                }
            }

            // Wind sound
            if let Ok(wind_osc) = context.create_oscillator() {
                wind_osc.set_type(OscillatorType::Sine);
                let freq_param = wind_osc.frequency();
                freq_param.set_value(200.0);
                
                if let Ok(wind_gain) = context.create_gain() {
                    let gain_param = wind_gain.gain();
                    gain_param.set_value(0.0);
                    wind_osc.connect_with_audio_node(&wind_gain).ok();
                    if let Some(master) = &self.master_gain {
                        wind_gain.connect_with_audio_node(master).ok();
                    }
                    wind_osc.start().ok();
                    
                    self.wind_oscillator = Some(wind_osc);
                    self.wind_gain = Some(wind_gain);
                }
            }

            // Tire screech
            if let Ok(screech_osc) = context.create_oscillator() {
                screech_osc.set_type(OscillatorType::Square);
                let freq_param = screech_osc.frequency();
                freq_param.set_value(1000.0);
                
                if let Ok(screech_gain) = context.create_gain() {
                    let gain_param = screech_gain.gain();
                    gain_param.set_value(0.0);
                    screech_osc.connect_with_audio_node(&screech_gain).ok();
                    if let Some(master) = &self.master_gain {
                        screech_gain.connect_with_audio_node(master).ok();
                    }
                    screech_osc.start().ok();
                    
                    self.tire_screech_oscillator = Some(screech_osc);
                    self.tire_screech_gain = Some(screech_gain);
                }
            }

            self.initialized = true;
        }
    }

    /// Garante que o contexto está rodando; se ainda não foi inicializado por um
    /// gesto do usuário, não faz nada (evita spam de warnings de autoplay).
    pub fn ensure_running(&mut self) {
        if let Some(context) = &self.context {
            let _ = context.resume();
        }
    }

    pub fn update(&mut self, bike: &Bike) {
        if !self.initialized {
            // Não inicializa sozinho aqui: AudioContext exige gesto do usuário.
            // A tela inicial (start screen) chama init() no clique.
            return;
        }
        self.ensure_running();

        let Some(_context) = &self.context else { return };

        // Engine sound based on RPM/speed
        if let (Some(osc), Some(gain)) = (&self.engine_oscillator, &self.engine_gain) {
            let rpm = (bike.speed * 50.0 + 800.0).clamp(60.0, 8000.0);
            let target_freq = rpm / 60.0 * 2.0; // Convert to frequency
            
            // Smooth frequency change
            let freq_param = osc.frequency();
            let current_freq = freq_param.value();
            let new_freq = current_freq + (target_freq - current_freq) * 0.1;
            freq_param.set_value(new_freq);
            
            // Volume based on throttle
            let gain_param = gain.gain();
            let target_gain = bike.throttle * 0.3 + 0.05;
            gain_param.set_value(target_gain);
        }

        // Wind sound based on speed
        if let (Some(osc), Some(gain)) = (&self.wind_oscillator, &self.wind_gain) {
            let gain_param = gain.gain();
            let wind_volume = (bike.speed / 50.0).clamp(0.0, 0.3);
            gain_param.set_value(wind_volume);
            
            let freq_param = osc.frequency();
            let wind_freq = 200.0 + bike.speed * 2.0;
            freq_param.set_value(wind_freq);
        }

        // Tire screech during drift
        if let (Some(osc), Some(gain)) = (&self.tire_screech_oscillator, &self.tire_screech_gain) {
            let gain_param = gain.gain();
            if bike.is_drift_active && bike.speed > 15.0 {
                let screech_volume = ((bike.speed - 15.0) / 30.0).clamp(0.0, 0.4);
                gain_param.set_value(screech_volume);
            } else {
                gain_param.set_value(0.0);
            }
            
            let freq_param = osc.frequency();
            if bike.is_drift_active && bike.speed > 15.0 {
                let screech_freq = 800.0 + bike.speed * 5.0;
                freq_param.set_value(screech_freq);
            }
        }
    }

    pub fn play_wheelie_sound(&mut self) {
        // Could play a specific sound effect for wheelie start
    }

    pub fn play_crash_sound(&mut self) {
        // Could play crash sound
    }

    pub fn set_master_volume(&mut self, volume: f32) {
        if let Some(gain) = &self.master_gain {
            let gain_param = gain.gain();
            gain_param.set_value(volume.clamp(0.0, 1.0));
        }
    }
}