#![cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = Math)]
    pub fn random() -> f32;

    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    pub fn warn(s: &str);
    #[wasm_bindgen(js_namespace = console)] pub fn error(s: &str);
}

// wrappers fun
use super::motor::BoardGenerator;

#[wasm_bindgen]
pub struct Generator(BoardGenerator);

#[wasm_bindgen]
#[derive(Default)]
pub struct Array16u8([u8; 16]);

#[wasm_bindgen]
impl Generator{
    #[wasm_bindgen(constructor)]
    pub fn new(unique: usize, duplicate: usize) -> Generator{
        Generator(BoardGenerator::new(unique, duplicate))
    }

    pub fn next(&mut self) -> Option<Array16u8>{
        let mut arr = Array16u8::default();
        match self.0.back(){
         
        }
    } 
}
