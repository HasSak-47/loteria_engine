// #![cfg(target_arch="wasm32")]
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
use super::motor::{BoardGenerator, GeneratedBoard};

#[wasm_bindgen]
pub struct Generator(BoardGenerator);

#[wasm_bindgen]
#[derive(Default)]
pub struct Array16u8([u8; 16]);

#[wasm_bindgen]
impl Array16u8{
    pub fn get(&self, index: usize) -> u8{ self.0[index] } 
    pub fn set(&mut self, index: usize, val : u8) { self.0[index] = val; } 
    pub fn to_string(&self) -> String{format!("{self}")}

    pub fn get_row(&self, i: usize) -> Vec<u8> {
        vec![
            self.0[(i * 4) + 0],
            self.0[(i * 4) + 1],
            self.0[(i * 4) + 2],
            self.0[(i * 4) + 3],
        ]
    }
}

fn make_arr(board: GeneratedBoard) -> Array16u8{
    let mut arr = Array16u8::default();    

    for i in 0..16{
        arr.0[i] = board.0[i].image_index as u8;
    }

    arr
}

impl std::fmt::Display for Array16u8{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..16{
            write!(f, "{:2}, ", self.0[i])?;
            if i % 4 == 3{
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

#[wasm_bindgen]
impl Generator{
    #[wasm_bindgen(constructor)]
    pub fn new(unique: usize, duplicate: usize) -> Generator{
        Generator(BoardGenerator::new(unique, duplicate))
    }

    pub fn next(&mut self) -> Option<Array16u8>{
        // there must be a better way to do this
        match self.0.boards.pop(){
            None => None,
            Some(l) => {Some(make_arr(l)) }
        }
    } 
}
