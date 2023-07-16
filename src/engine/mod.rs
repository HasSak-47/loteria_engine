#![allow(dead_code)]

use wasm_bindgen::prelude::*;

const NONE: u8 = 255;
const DECK_SIZE: usize = 54;

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

fn _random() -> usize {
    (random() * (1 << 23) as f32) as usize
}

fn rand_card() -> u8{
    (_random() % DECK_SIZE) as u8
}

// dumb way to do it
pub fn generate_set(last_set: Option<[u8; DECK_SIZE]>, current_set: &mut [u8; DECK_SIZE]) {
    let mut orderder_tape = Vec::new();
    for i in 0..DECK_SIZE{
        orderder_tape.push(i as u8);
    }

    for i in 0..DECK_SIZE{
        let pop = _random() % orderder_tape.len();
        let x = orderder_tape[pop];

        if last_set.is_some() && last_set.unwrap().contains(&x){

        }
        current_set[i] = orderder_tape.remove(i);
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
        let mut sets = Vec::new();
        let lenght = dup * 15 + uni * 16;
        if lenght < DECK_SIZE {
            warn(&format!("Unfinished set! {lenght} out of 56 ({dup}, {uni})"));
        }

        let sets_lenght = 1 + lenght / DECK_SIZE;
        sets.resize(sets_lenght * DECK_SIZE, [NONE; DECK_SIZE]);

        for i in 0..sets_lenght{
            generate_set(
                if i == 0 {None} else {Some(sets[i - 1].clone())},
                 &mut sets[i]
            );
        }

        let tape = Vec::new();

        BoardGenerator { dup, uni, tape, boards: Vec::new()}
    }

    pub fn enough(&self) -> bool{
        self.dup * 15 + self.uni * 16 < DECK_SIZE 
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
