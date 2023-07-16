#![allow(dead_code)]

use wasm_bindgen::prelude::*;

const NONE: u8 = 255;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = Math)]
    pub fn random() -> f32;

    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    pub fn warn(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    pub fn error(s: &str);
}
fn rand_card() -> u8{
    (random() * 54.0) as u8
}

pub fn non_repeating(tape16: &[u8], tape56: &[u8]) -> u8{
    log(&format!("16: {tape16:?}\n 56: {tape56:?}"));
    loop{
        let x = rand_card(); if tape56.binary_search(&x).is_ok(){
            continue;
        }
        if tape16.binary_search(&x).is_ok(){
            continue;
        }

        return x;
    }
}

#[derive(Debug, Default, Clone, Copy)]
enum CellType{
    #[default]
    Undetermined,
    Unique,
    Duplicate,
}

#[derive(Debug, Default, Clone, Copy)]
struct Cell{
    image_index: usize,
    cell_type: CellType,
}

#[derive(Debug, Default, Clone, Copy)]
struct GeneratedBoard([Cell;16]);
type Eater = fn(&mut Vec<u8>) -> GeneratedBoard;

pub struct Board([u8; 16]);

impl Board {
    pub fn new() -> Board {
        Board([0; 16])
    }
}

#[wasm_bindgen]
pub struct BoardGenerator{
    pub dup: usize,
    pub uni: usize,
    tape: Vec<u8>,
    boards: Vec<GeneratedBoard>,
}

#[wasm_bindgen]
impl BoardGenerator{
    #[wasm_bindgen(constructor)]
    pub fn new(dup: usize, uni: usize) -> BoardGenerator {
        let mut tape = Vec::new();
        let lenght = dup * 15 + uni * 16;
        tape.resize(lenght, NONE);

        if lenght < 54 {
            warn("Unfinished set!");
        }

        for i in 0..lenght{
            let min16 = if i < 16 { 0 } else { i - 15 };
            let min56 = if i < 56 { 0 } else { (i / 56) * 56};
            tape[i] = non_repeating(&tape[min16..i], &tape[min56..i]);
        }

        BoardGenerator { dup, uni, tape, boards: Vec::new()}
    }

    pub fn enough(&self) -> bool{
        self.dup * 15 + self.uni * 16 < 54
    }

    pub fn make_next(&self){
    }

    pub fn log_tape(&self) {
        let mut tape = String::new();
        for i in &self.tape {
            tape.push_str(&format!("{i}, "));
        }

        log(&tape);
    }


}
