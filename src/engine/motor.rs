
use super::random::random;

const NONE: u8 = 255;
const DECK_SIZE: usize = 54;

fn pop_deck(index: usize, src_set: &mut Vec<u8>, dst_set: &mut [u8; DECK_SIZE]){
    let pop = random() % src_set.len();
    dst_set[index] = src_set.remove(pop);
}

fn fill_tail_set(head_set: &mut [u8; DECK_SIZE]) {
    let mut ordered_tape : Vec<u8> = (0..DECK_SIZE as u8).collect();

    for i in 0..DECK_SIZE{
        pop_deck(i, &mut ordered_tape, head_set);
    }
}

fn fill_head_set(head_set: &mut [u8; DECK_SIZE], last_set : &[u8; DECK_SIZE]) {
    let mut ordered_tape : Vec<u8> = (0..DECK_SIZE as u8).collect();
    let mut last_16 = last_set.clone()[DECK_SIZE - 16..DECK_SIZE].to_vec();

    for val in &last_16{
        let index = ordered_tape.iter().position(|&x| x == *val);
        match index{
            Some(index) => {ordered_tape.remove(index);},
            None => {},
        }
    }

    for i in 0..(DECK_SIZE - 16){
        pop_deck(i, &mut ordered_tape, head_set);
    }

    for i in DECK_SIZE - 16..DECK_SIZE{
        pop_deck(i, &mut last_16, head_set);
    }
}

fn eat_16(tape : &mut Vec<u8>) -> GeneratedBoard{
    let mut board = GeneratedBoard::default();

    for i in 0..16{
        let index = tape.pop().unwrap();
        board.0[i] = Cell{image_index: index as usize, cell_type: CellType::Unique};
    }

    board
}

fn eat_15(tape : &mut Vec<u8>) -> GeneratedBoard{
    let mut board = GeneratedBoard::default();
    board.1 = 1;

    for i in 0..15{
        let index = tape.pop().unwrap();
        board.0[i] = Cell::new(index as usize, CellType::Unique);
    }


    let get_index = || 1 + 4 + (random() % 2) + 4 * (random() % 2);
 
    let copy_index = get_index();

    // please I want a do while
    let mut swap_index = get_index();
    while swap_index == copy_index{
        swap_index = get_index();
    }

    board.0[copy_index].cell_type = CellType::Original;

    board.0[15] = board.0[swap_index];
    board.0[15].cell_type = CellType::Swap;

    board.0[swap_index] = board.0[copy_index];
    board.0[swap_index].cell_type = CellType::Duplicate;

    board
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum CellType{
    #[default]
    Undetermined,
    Unique,
    Swap,
    Original,
    Duplicate,
}

impl CellType{
    fn to_char(&self) -> char{
        match self{
            Self::Undetermined => 'X',
            Self::Unique       => 'U',
            Self::Swap         => 'S',
            Self::Original     => 'O',
            Self::Duplicate    => 'D',
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Cell{
    pub image_index: usize,
    pub cell_type: CellType,
}

impl Cell{
    pub const fn new(image_index: usize, cell_type : CellType) -> Self {
        Cell {image_index, cell_type}
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct GeneratedBoard(pub [Cell;16], pub u8);

impl std::fmt::Display for GeneratedBoard{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for indx in 0..16 {
            let i = indx % 4;
            write!(f, "({:2},{}),", self.0[indx].image_index, self.0[indx].cell_type.to_char())?;
            if i == 3{ write!(f, "\n")?; }
        }
        Ok(())
    }
}


pub struct BoardGenerator{
    pub dup: usize,
    pub uni: usize,
    pub boards: Vec<GeneratedBoard>,
}

#[cfg(test)]
pub fn test_tape(tape: &Vec<u8>){
    let tape = tape.clone();
    for i in 0..tape.len() - 16{
        let next16 = &tape[i + 1..i + 17];
        if next16.contains(&tape[i]){
            panic!("duplicate in tape!");
        }

    }
}

#[allow(dead_code)]
impl BoardGenerator{
    pub fn new(dup: usize, uni: usize) -> BoardGenerator {
        let mut sets = Vec::new();
        let lenght = dup * 15 + uni * 16;

        let sets_lenght = 1 + lenght / DECK_SIZE;
        sets.resize(sets_lenght, [NONE; DECK_SIZE]);

        for i in 0..sets_lenght{
            if i == 0{
                fill_tail_set(&mut sets[i]);
            }
            else{
                let last = sets[i - 1].clone();
                fill_head_set(&mut sets[i], &last);
            }
        }

        let mut tape = Vec::new();
        tape.resize(lenght, NONE);

        for i in 0..lenght{
            let set_index = i / DECK_SIZE;
            let set_subindex = i % DECK_SIZE;

            tape[i] = sets[set_index][set_subindex];
        }

        #[cfg(test)]
        test_tape(&tape);

        let mut boards = Vec::new();
        for _ in 0..uni{
            boards.push(eat_16(&mut tape));
        }

        for _ in 0..dup{
            boards.push(eat_15(&mut tape));
        }

        BoardGenerator { dup, uni, boards}
    }

    pub fn enough(&self) -> bool{
        self.dup * 15 + self.uni * 16 < DECK_SIZE 
    }
}

#[cfg(test)]
mod test{
use super::*;
impl GeneratedBoard {
    pub fn verify(&self) -> Result<(), String>{
        for indx in 0..16 * 16{
            let i = indx % 16;
            let j = indx / 16;
            if i == j{
                continue;
            }

            if self.0[i].image_index == self.0[j].image_index &&
                self.0[i].cell_type == CellType::Unique &&
                self.0[j].cell_type == CellType::Unique
            { return Err(format!("{} {} {}", false, i, j)); }

        }

        Ok(())
    }

    // todo : do this
    fn vefify_vector(_v : Vec<(Cell, usize)>) -> Result<(), String>{
        Ok(())
    }

    // this is shit lmao
    pub fn verify2(&self) -> Result<(), String>{
        let mut map : Vec<(usize, Vec<(Cell, usize)>)> = Vec::new();
        for ite in self.0.into_iter().enumerate(){
            let cell_index = ite.0;
            let cell = ite.1;
            match map.iter().position(|x| x.0 == cell.image_index){
                Some(index) => {
                    map[index].1.push((cell, cell_index));
                },
                None => {
                    let mut v = Vec::new();
                    v.push((cell, cell_index));
                    map.push((cell.image_index, v));
                },
            }
        }

        let mut duplicates = 0;
        for data in map{
            if data.1.len() > 2{
                return Err("something went wrong!".to_string());
            }
            else
            if data.1.len() == 2{
                duplicates += 1;
                Self::vefify_vector(data.1).unwrap();
            }
            else
            if data.1.len() == 0{
                return Err("someting went wrong!".to_string());
            }


        }

        if self.1 != duplicates {
            return Err(format!("duplicates don't match expected:{} got:{}!", self.1, duplicates));
        }

        Ok(())
    }
}
}
