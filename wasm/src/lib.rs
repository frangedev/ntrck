use wasm_bindgen::prelude::*;
use web_sys::{AudioContext, AudioBuffer, AudioBufferSourceNode, GainNode, OscillatorNode};
use serde::{Serialize, Deserialize};

mod audio;
mod sequencer;
mod patterns;

pub use audio::*;
pub use sequencer::*;
pub use patterns::*;

#[wasm_bindgen]
pub struct NtrckEngine {
    audio_context: AudioContext,
    sequencer: Sequencer,
    patterns: PatternBank,
}

#[wasm_bindgen]
impl NtrckEngine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<NtrckEngine, JsValue> {
        let audio_context = AudioContext::new()
            .map_err(|_| JsValue::from_str("Failed to create AudioContext"))?;
        
        let sequencer = Sequencer::new();
        let patterns = PatternBank::new();
        
        Ok(NtrckEngine {
            audio_context,
            sequencer,
            patterns,
        })
    }

    pub fn start(&self) -> Result<(), JsValue> {
        self.audio_context.resume()
            .map_err(|_| JsValue::from_str("Failed to resume AudioContext"))?;
        Ok(())
    }

    pub fn stop(&self) -> Result<(), JsValue> {
        self.audio_context.suspend()
            .map_err(|_| JsValue::from_str("Failed to suspend AudioContext"))?;
        Ok(())
    }

    pub fn set_bpm(&mut self, bpm: f64) {
        self.sequencer.set_bpm(bpm);
    }

    pub fn play_pattern(&self, pattern_id: u32) -> Result<(), JsValue> {
        if let Some(pattern) = self.patterns.get_pattern(pattern_id) {
            self.sequencer.play_pattern(pattern, &self.audio_context)
                .map_err(|e| JsValue::from_str(&format!("Failed to play pattern: {:?}", e)))?;
        }
        Ok(())
    }

    pub fn add_pattern(&mut self, pattern: JsValue) -> Result<u32, JsValue> {
        let pattern: Pattern = serde_wasm_bindgen::from_value(pattern)
            .map_err(|e| JsValue::from_str(&format!("Failed to deserialize pattern: {}", e)))?;
        
        let id = self.patterns.add_pattern(pattern);
        Ok(id)
    }

    /// List all pattern IDs
    #[wasm_bindgen]
    pub fn list_patterns(&self) -> js_sys::Array {
        let array = js_sys::Array::new();
        for id in self.patterns.list_patterns() {
            array.push(&JsValue::from_f64(id as f64));
        }
        array
    }

    /// Export a pattern as JSON
    #[wasm_bindgen]
    pub fn export_pattern(&self, pattern_id: u32) -> Result<JsValue, JsValue> {
        if let Some(pattern) = self.patterns.get_pattern(pattern_id) {
            serde_wasm_bindgen::to_value(pattern).map_err(|e| JsValue::from_str(&format!("Export error: {:?}", e)))
        } else {
            Err(JsValue::from_str("Pattern not found"))
        }
    }

    /// Import a pattern from JSON
    #[wasm_bindgen]
    pub fn import_pattern(&mut self, pattern_json: JsValue) -> Result<u32, JsValue> {
        let pattern: crate::patterns::Pattern = serde_wasm_bindgen::from_value(pattern_json)
            .map_err(|e| JsValue::from_str(&format!("Import error: {}", e)))?;
        Ok(self.patterns.add_pattern(pattern))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_creation() {
        // This would need a proper test environment with Web APIs
        // For now, just ensure compilation
        assert!(true);
    }
}
