use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{AudioContext, AudioBuffer, AudioBufferSourceNode, GainNode, OscillatorNode};
use serde::{Serialize, Deserialize};
use js_sys::Reflect;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioSample {
    pub frequency: f32,
    pub duration: f32,
    pub amplitude: f32,
    pub waveform: Waveform,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Waveform {
    Sine,
    Square,
    Sawtooth,
    Triangle,
}

pub struct AudioProcessor {
    context: AudioContext,
}

impl AudioProcessor {
    pub fn new(context: AudioContext) -> Self {
        Self { context }
    }

    pub fn create_oscillator(&self, sample: &AudioSample) -> Result<OscillatorNode, JsValue> {
        let oscillator = OscillatorNode::new(&self.context)?;
        oscillator.frequency().set_value(sample.frequency);
        let type_str = match sample.waveform {
            Waveform::Sine => "sine",
            Waveform::Square => "square",
            Waveform::Sawtooth => "sawtooth",
            Waveform::Triangle => "triangle",
        };
        Reflect::set(
            oscillator.as_ref(),
            &JsValue::from_str("type"),
            &JsValue::from_str(type_str),
        )?;
        Ok(oscillator)
    }

    pub fn create_gain(&self, amplitude: f32) -> Result<GainNode, JsValue> {
        let gain = GainNode::new(&self.context)?;
        gain.gain().set_value(amplitude);
        Ok(gain)
    }

    pub fn play_sample(&self, sample: &AudioSample, start_time: f64) -> Result<(), JsValue> {
        let oscillator = self.create_oscillator(sample)?;
        let gain = self.create_gain(sample.amplitude)?;
        oscillator.connect_with_audio_node(&gain)?;
        gain.connect_with_audio_node(&self.context.destination())?;
        oscillator.start_with_when(start_time)?;
        oscillator.stop_with_when(start_time + sample.duration as f64)?;
        Ok(())
    }
}

#[wasm_bindgen]
pub fn create_audio_buffer(context: &AudioContext, sample_rate: u32, duration: f32) -> Result<AudioBuffer, JsValue> {
    let length = (sample_rate as f32 * duration) as u32;
    context.create_buffer(1, length, sample_rate as f32)
} 