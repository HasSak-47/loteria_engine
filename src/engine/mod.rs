use std::fmt::Display;

use crate::tape::*;
use crate::board::BasicBoard;
use crate::random::rand_range_pair;
use anyhow::Result;


/**
the value in the produced board
 */
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Card{
    Value(u8),
    #[default]
    None,
}

impl std::fmt::Display for Card{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }

}

#[allow(dead_code)]
impl Card {
    pub fn set(&mut self, v: u8){
        *self = Self::Value(v)
    }

    pub fn unset(&mut self){
        *self = Self::None;
    }

    pub fn unpack(&self) -> u8{
        match self{
            Self::Value(v) => *v,
            _ => 0,
        }
    }
}

/**
The final board
 */
pub type Board = BasicBoard<Card>;

/**
holds the configuration of a card
 .0 is the tape that it will use
 */
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfigCard(pub u8);
 
/**
The action that the builder will do when the stack is consumed
NotSpecial: Will just set the top value in the stack
CloneMark: All of this will take the same value from the top of the stack
Forced: Will ignore the stack and put a value in that cell
 */
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataAction{
    #[default]
    NotSpecial,
    CloneMark,
    Forced(u8),
}

/**
This holds the action that will be used and which stack will be poped
.0 is the DataAction and .1 is the index of the Tape
 */
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct DataCard(DataAction, usize);

/**
Is the Instructions that the consumption of the tape/stack will follow
 */
pub type DataBoard = BasicBoard<DataCard>;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct BoardBuilder{
    board_prototypes: Vec<DataBoard>,
    blacklist: Vec<u8>,
    forcelist: Vec<u8>,

    tapes: Vec<Tape>,
    board_size: usize,

    /** the amount of cards that are in the deck */
    count: usize,
    /** the total number of boards to generate */
    total: usize
}

impl BoardBuilder{
    pub fn new() -> Self{
        Self {board_size: 16, count: 54, ..Default::default()}
    }

    pub fn set_count(mut self, count: usize) -> Self{
        self.count = count;
        self
    }

    pub fn set_count_ref(&mut self, count: usize) -> &mut Self{
        self.count = count;
        self
    }

    pub fn set_total(mut self, total: usize) -> Self{
        self.total = total;
        self.board_prototypes.resize(total, DataBoard::default());
        self
    }

    pub fn set_total_ref(&mut self, total: usize) -> &mut Self{
        self.total = total;
        self.board_prototypes.resize(total, DataBoard::default());
        self
    }

    pub fn generate_tapes(mut self) -> Self{
        let tape = TapeGenerator::new(self.count, self.total, &self.blacklist);
        self.tapes.push(tape.generate());
        
        self
    }

    pub fn get_card(&mut self, data_card: DataCard, clone_val: &mut Option<Card>) -> Card{
        use DataAction as DC;
        match data_card.0{
            DC::Forced(s) => Card::Value(s),
            DC::NotSpecial => Card::Value(self.tapes[0].0.remove(0)),
            DC::CloneMark => {
                if clone_val.is_none(){
                    *clone_val = Some(Card::Value(self.tapes[0].0[0]));
                    Card::Value(self.tapes[0].0.remove(0))
                }
                else {
                    clone_val.unwrap()
                }
            },
        }
    }

    pub fn create_board(&mut self) -> Board{
        let mut b = Board::default();

        let board_proto = self.board_prototypes.pop().unwrap();
        println!("creating board: {board_proto:?}");
        let mut clone_val = None;
        for ij in 0..16{
            let i = ij % 4;
            let j = ij / 4;

            let data_card = board_proto[i][j];
            b[i][j] = self.get_card(data_card, &mut clone_val);
        }

        b
    }

    pub fn generate_boards(mut self) -> Vec<Board>{
        let mut v = Vec::new();
        let total_cards = self.total;

        for _ in 0..total_cards{
            v.push(self.create_board());
        }

        v
    }
}
