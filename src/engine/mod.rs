pub mod random;
pub mod tape;
pub mod board;
mod lua;

#[cfg(test)]
pub mod test;

use std::fmt::Display;

use board::BasicBoard;
use crate::engine::random::rand_range_pair;
use anyhow::Result;

use tape::*;

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

    count: usize,
    board_size: usize,
    tapes: Vec<Tape>,
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

    pub fn act_on<B: BoardActor>(self, actor: B) -> Self{
        actor.act_on(self).unwrap()
    }

    pub fn generate_tapes(mut self) -> Self{
        let tape = TapeGenerator::new(self.count, self.total, &self.blacklist);
        self.tapes.push(tape.generate());
        
        self
    }

    pub fn get_card(&mut self, data_card: DataCard, clone_val: &mut Option<Card>) -> Card{
        use DataAction as DC;
        match data_card.0{
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
            DC::Forced(s) => Card::Value(s),
        }
    }

    pub fn create_board(&mut self) -> Board{
        let mut b = Board::default();

        let board_proto = self.board_prototypes.pop().unwrap();
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

        if self.tapes[0].0.len() == 0{
            return v;
        }
        for _ in 0..total_cards{
            v.push(self.create_board());
        }

        v
    }

    // ugly getters
    pub fn get_count(&self) -> usize{ self.count }
}

pub trait BoardActor{
    fn act_on(&self, b: BoardBuilder) -> Result<BoardBuilder>;
}

#[repr(transparent)]
pub struct BoardActorC(
    Box<dyn BoardActor>,
);

macro_rules! new_board_actor {
    ($name:ident, $cname:ident, $($tname: ident: $type:ty), *) => {
        #[repr(C)]
        #[derive(Debug, PartialEq, Eq)]
        pub struct $name(  $( pub $type, )* );

        impl $name{
            pub fn new($( $tname: $type, ) *) -> Self{
                Self($( $tname, )*)
            }
        }
    };
}

new_board_actor!(BlackList, new_blacklist, val: u8);
new_board_actor!(Force, new_force, val: u8);
new_board_actor!(Set, new_set, i: usize, j: usize, val: u8);
new_board_actor!(MarkPair, new_mark_pair, i: usize, j: usize, k: usize, l: usize);
new_board_actor!(RandomMarkPair, new_random_mark_pair,);
new_board_actor!(RandomCenterMarkPair, new_random_center_mark_pair,);
new_board_actor!(UpperCenterMarkPair, new_upper_center_mark_pair,);
new_board_actor!(LowerCenterMarkPair, new_lower_center_mark_pair,);
new_board_actor!(SetTotal, new_set_total, val: usize);
new_board_actor!(SetCount, new_set_count, val: usize);
new_board_actor!(SetPair, new_set_pair, pair: usize, card: usize);

impl BoardActor for BlackList{
    fn act_on(&self, mut b: BoardBuilder) -> Result<BoardBuilder> {
        b.blacklist.push(self.0);
        Ok(b) 
    }
}

impl BoardActor for Force{
    fn act_on(&self, mut b: BoardBuilder) -> Result<BoardBuilder> {
        b.board_size -= 1;
        b.forcelist.push(self.0);
        Ok(b) 
    }
}

impl BoardActor for Set{
    fn act_on(&self, mut b: BoardBuilder) -> Result<BoardBuilder> {
        b.board_size -= 1;
        for board in &mut b.board_prototypes{
            board[self.0][self.1].0 = DataAction::Forced(self.2);
        }
        Ok(b) 
    }
}

impl BoardActor for MarkPair{
    fn act_on(&self, mut b: BoardBuilder) -> Result<BoardBuilder> {
        b.board_size -= 1;
        for board in &mut b.board_prototypes{
            board[self.0][self.1].0 = DataAction::CloneMark;
            board[self.2][self.3].0 = DataAction::CloneMark;
        }
        Ok(b) 
    }
}

impl BoardActor for RandomMarkPair{
    fn act_on(&self, mut b: BoardBuilder) -> Result<BoardBuilder> {
        b.board_size -= 1;
        for board in &mut b.board_prototypes{
            let source = rand_range_pair(0, 4);
            let mut target =  rand_range_pair(0, 4);
            while target == source{ target = rand_range_pair(0, 4); }
            board[source.0][source.1].0 = DataAction::CloneMark;
            board[target.0][target.1].0 = DataAction::CloneMark;
        }
        Ok(b) 
    }
}

impl BoardActor for RandomCenterMarkPair{
    fn act_on(&self, mut b: BoardBuilder) -> Result<BoardBuilder> {
        b.board_size -= 1;
        for board in &mut b.board_prototypes{
            let source = rand_range_pair(0, 2);
            let mut target =  rand_range_pair(0, 2);
            while target == source{ target = rand_range_pair(0, 2); }
            board[source.0 + 1][source.1 + 1].0 = DataAction::CloneMark;
            board[target.0 + 1][target.1 + 1].0 = DataAction::CloneMark;
        }
        Ok(b) 
    }
}

impl BoardActor for UpperCenterMarkPair{
    fn act_on(&self, mut b: BoardBuilder) -> Result<BoardBuilder> {
        for board in &mut b.board_prototypes{
            board[1][1].0 = DataAction::CloneMark;
            board[2][1].0 = DataAction::CloneMark;
        }
        Ok(b) 
    }
}

impl BoardActor for LowerCenterMarkPair{
    fn act_on(&self, mut b: BoardBuilder) -> Result<BoardBuilder> {
        for board in &mut b.board_prototypes{
            board[1][2].0 = DataAction::CloneMark;
            board[2][2].0 = DataAction::CloneMark;
        }
        Ok(b) 
    }
}

impl BoardActor for SetTotal{
    fn act_on(&self, mut b: BoardBuilder) -> Result<BoardBuilder> {
        b.set_total_ref(self.0);
        Ok(b) 
    }
}

impl BoardActor for SetCount{
    fn act_on(&self, mut b: BoardBuilder) -> Result<BoardBuilder> {
        b.count = self.0;
        Ok(b) 
    }
}

impl BoardActor for SetPair{
    fn act_on(&self, mut b: BoardBuilder) -> Result<BoardBuilder> {
        b.board_size -= 1;
        b.blacklist.push(self.0 as u8);
        let board = &mut b.board_prototypes[self.1];
        for i in 0..16{
            let t = &mut board.get_mut(i).0;
            if let DataAction::CloneMark = t{
                *t = DataAction::Forced(self.0 as u8);
            }
        }
        Ok(b) 
    }
}

impl Display for BoardBuilder{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for ij in 0..16 {
            let i = ij % 4; 
            let j = ij / 4; 
            write!(f, "{:?} ", self.board_prototypes[0][i][j])?;
            if i == 3{
                writeln!(f)?;
            }
        }

        Ok(()) } }

pub use lua::*;
