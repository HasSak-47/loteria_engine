use super::random::random;

const NONE: u8 = 255;
const DECK_SIZE: usize = 54;

fn fill_tail_set(head_set: &mut [u8; DECK_SIZE]) {
    let mut orded_tape : Vec<u8> = (0..DECK_SIZE as u8).collect();

    for i in 0..DECK_SIZE{
        let pop = random() % orded_tape.len();
        head_set[i] = orded_tape.remove(pop);
    }
}

fn fill_head_set(head_set: &mut [u8; DECK_SIZE], last_set : [u8; DECK_SIZE]) {
    let mut orded_tape : Vec<u8> = (0..DECK_SIZE as u8).collect();
    let last_16 = &last_set[DECK_SIZE - 16..DECK_SIZE];
    for i in 0..16{
        // no do while :)
        let mut pop = random() % orded_tape.len();
        while last_16.contains(&(pop as u8)){
            pop = random() % orded_tape.len();
        }
        head_set[i] = orded_tape.remove(pop);
    }

    for i in 16..DECK_SIZE{
        let pop = random() % orded_tape.len();
        head_set[i] = orded_tape.remove(pop);
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

// dumb way to do it
fn generate_set(last_set: Option<[u8; DECK_SIZE]>, current_set: &mut [u8; DECK_SIZE]) {
    match last_set{
        None => fill_tail_set(current_set),
        Some(last_set) => fill_head_set(current_set, last_set),
    }
}

#[derive(Debug, Default, Clone, Copy)]
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
    image_index: usize,
    cell_type: CellType,
}

impl Cell{
    pub const fn new(image_index: usize, cell_type : CellType) -> Self {
        Cell {image_index, cell_type}
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct GeneratedBoard([Cell;16]);

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

#[allow(dead_code)]
impl BoardGenerator{
    pub fn new(dup: usize, uni: usize) -> BoardGenerator {
        let mut sets = Vec::new();
        let lenght = dup * 15 + uni * 16;
        if lenght < DECK_SIZE {
        }

        let sets_lenght = 1 + lenght / DECK_SIZE;
        sets.resize(sets_lenght, [NONE; DECK_SIZE]);

        for i in 0..sets_lenght{
            generate_set(
                if i == 0 {None} else {Some(sets[i - 1].clone())},
                &mut sets[i]
            );
        }

        let mut tape = Vec::new();
        tape.resize(lenght, NONE);

        for i in 0..lenght{
            let set_index = i / DECK_SIZE;
            let set_subindex = i % DECK_SIZE;

            tape[i] = sets[set_index][set_subindex];
        }

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
