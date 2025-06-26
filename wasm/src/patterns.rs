use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use crate::audio::AudioSample;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pattern {
    pub id: u32,
    pub name: String,
    pub steps: Vec<Option<AudioSample>>,
    pub length: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternBank {
    patterns: std::collections::HashMap<u32, Pattern>,
    next_id: u32,
}

impl Pattern {
    pub fn new(name: String, length: usize) -> Self {
        Self {
            id: 0,
            name,
            steps: vec![None; length],
            length,
        }
    }

    pub fn set_step(&mut self, step: usize, sample: Option<AudioSample>) -> Result<(), String> {
        if step >= self.length {
            return Err(format!("Step {} out of bounds (length: {})", step, self.length));
        }
        self.steps[step] = sample;
        Ok(())
    }

    pub fn get_step(&self, step: usize) -> Option<&Option<AudioSample>> {
        self.steps.get(step)
    }

    pub fn clear(&mut self) {
        self.steps = vec![None; self.length];
    }
}

impl PatternBank {
    pub fn new() -> Self {
        Self {
            patterns: std::collections::HashMap::new(),
            next_id: 1,
        }
    }

    pub fn add_pattern(&mut self, mut pattern: Pattern) -> u32 {
        let id = self.next_id;
        pattern.id = id;
        self.patterns.insert(id, pattern);
        self.next_id += 1;
        id
    }

    pub fn get_pattern(&self, id: u32) -> Option<&Pattern> {
        self.patterns.get(&id)
    }

    pub fn get_pattern_mut(&mut self, id: u32) -> Option<&mut Pattern> {
        self.patterns.get_mut(&id)
    }

    pub fn remove_pattern(&mut self, id: u32) -> Option<Pattern> {
        self.patterns.remove(&id)
    }

    pub fn list_patterns(&self) -> Vec<u32> {
        self.patterns.keys().cloned().collect()
    }

    pub fn create_default_pattern(&mut self) -> u32 {
        let pattern = Pattern::new("Default Pattern".to_string(), 16);
        self.add_pattern(pattern)
    }
}

#[wasm_bindgen]
pub struct PatternWrapper {
    pattern: Pattern,
}

#[wasm_bindgen]
impl PatternWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String, length: usize) -> Self {
        Self {
            pattern: Pattern::new(name, length),
        }
    }

    pub fn set_step(&mut self, step: usize, sample: JsValue) -> Result<(), JsValue> {
        let sample: Option<AudioSample> = if sample.is_null() || sample.is_undefined() {
            None
        } else {
            Some(serde_wasm_bindgen::from_value(sample)
                .map_err(|e| JsValue::from_str(&format!("Failed to deserialize sample: {}", e)))?)
        };
        
        self.pattern.set_step(step, sample)
            .map_err(|e| JsValue::from_str(&e))?;
        Ok(())
    }

    pub fn get_length(&self) -> usize {
        self.pattern.length
    }

    pub fn get_name(&self) -> String {
        self.pattern.name.clone()
    }

    pub fn set_name(&mut self, name: String) {
        self.pattern.name = name;
    }
}

#[wasm_bindgen]
pub struct PatternBankWrapper {
    bank: PatternBank,
}

#[wasm_bindgen]
impl PatternBankWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            bank: PatternBank::new(),
        }
    }

    pub fn add_pattern(&mut self, pattern: PatternWrapper) -> u32 {
        self.bank.add_pattern(pattern.pattern)
    }

    pub fn create_default_pattern(&mut self) -> u32 {
        self.bank.create_default_pattern()
    }

    pub fn list_patterns(&self) -> js_sys::Array {
        let array = js_sys::Array::new();
        for id in self.bank.list_patterns() {
            array.push(&JsValue::from_f64(id as f64));
        }
        array
    }
} 