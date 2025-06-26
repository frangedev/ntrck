use wasm_bindgen::prelude::*;
use web_sys::AudioContext;
use serde::{Serialize, Deserialize};
use crate::audio::{AudioProcessor, AudioSample};
use crate::patterns::Pattern;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sequencer {
    bpm: f64,
    is_playing: bool,
    current_step: usize,
    step_duration: f64,
}

impl Sequencer {
    pub fn new() -> Self {
        Self {
            bpm: 120.0,
            is_playing: false,
            current_step: 0,
            step_duration: 0.0,
        }
    }

    pub fn set_bpm(&mut self, bpm: f64) {
        self.bpm = bpm;
        self.step_duration = 60.0 / (bpm * 4.0); // 16th note duration
    }

    pub fn start(&mut self) {
        self.is_playing = true;
        self.current_step = 0;
    }

    pub fn stop(&mut self) {
        self.is_playing = false;
        self.current_step = 0;
    }

    pub fn play_pattern(&self, pattern: &Pattern, context: &AudioContext) -> Result<(), JsValue> {
        let processor = AudioProcessor::new(context.clone());
        let current_time = context.current_time();
        
        for (step_idx, step) in pattern.steps.iter().enumerate() {
            if let Some(sample) = step {
                let start_time = current_time + (step_idx as f64 * self.step_duration);
                processor.play_sample(sample, start_time)?;
            }
        }
        
        Ok(())
    }

    pub fn get_current_step(&self) -> usize {
        self.current_step
    }

    pub fn is_playing(&self) -> bool {
        self.is_playing
    }

    pub fn get_bpm(&self) -> f64 {
        self.bpm
    }
}

#[wasm_bindgen]
pub struct SequencerWrapper {
    sequencer: Sequencer,
}

#[wasm_bindgen]
impl SequencerWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            sequencer: Sequencer::new(),
        }
    }

    pub fn set_bpm(&mut self, bpm: f64) {
        self.sequencer.set_bpm(bpm);
    }

    pub fn start(&mut self) {
        self.sequencer.start();
    }

    pub fn stop(&mut self) {
        self.sequencer.stop();
    }

    pub fn get_current_step(&self) -> usize {
        self.sequencer.get_current_step()
    }

    pub fn is_playing(&self) -> bool {
        self.sequencer.is_playing()
    }
} 