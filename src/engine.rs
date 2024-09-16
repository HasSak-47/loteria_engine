use crate::error::*;
use std::fmt::Display;

use crate::{board::BasicBoard, tape::TapeGenerator};

/**
the value in the produced board
 */
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Card{
    Value(usize),
    #[default]
    None,
}

impl std::fmt::Display for Card{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }

}

impl Card {
    pub fn set(&mut self, v: usize){
        *self = Self::Value(v)
    }

    pub fn unset(&mut self){
        *self = Self::None;
    }

    pub fn unpack(&self) -> usize{
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
pub struct ConfigCard(pub usize);
 
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
    Forced(usize),
}

/**
This holds the action that will be used and which stack will be poped
.0 is the DataAction and .1 is the index of the Tape
 */
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct DataCard(pub DataAction, pub usize);

impl Display for DataAction{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match  self {
            DataAction::CloneMark =>  String::from("Clone"),
            DataAction::Forced(u) =>       format!("F({u:2})"),
            DataAction::NotSpecial => String::from("NtSpl"),
            
        })?;
        Ok(())
    }
}

impl Display for DataCard{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.0, self.1)?;
        Ok(())
    }
}

/**
Is the Instructions that the consumption of the tape/stack will follow
 */
pub type DataBoard = BasicBoard<DataCard>;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
enum State{
    #[default]
    Setup,
    Inited,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct BoardBuilder{
    state: State,
    board_prototypes: Vec<DataBoard>,
    /** board width */
    width: usize,
    /** board height */
    height: usize, 

    /** cards that are not allowed in any board*/
    blacklist: Vec<usize>,
    /** cards that are forced in all board*/
    forcelist: Vec<usize>,


    /** the amount of cards that are in the deck */
    count: usize,
    /** the total number of boards to generate */
    total: usize,
    /** the total number of tapes needed*/
    tapes: usize,

}

impl BoardBuilder{
    pub const fn new() -> Self{
        Self {
            state: State::Setup,
            height: 4,
            width : 4,
            count : 54,
            total : 4,
            tapes : 1,

            blacklist: Vec::new(),
            forcelist: Vec::new(),
            board_prototypes: Vec::new(),
        }
    }

    // lua
    pub fn set_count(&mut self, count: usize) {
        if self.state == State::Inited{ return; }
        self.count = count;
    }

    // lua
    pub fn set_width(&mut self, width: usize) {
        if self.state == State::Inited{ return; }
        self.width = width;
    }

    // lua
    pub fn set_height(&mut self, height: usize) {
        if self.state == State::Inited{ return; }
        self.height = height;
    }

    // lua
    pub fn set_total(&mut self, total: usize) {
        if self.state == State::Inited{ return; }
        self.total = total;
        self.board_prototypes.reserve(total);
    }

    // lua
    pub fn init(&mut self) {
        if self.state == State::Inited{ return; }

        self.state = State::Inited;
        self.board_prototypes.resize(self.total, BasicBoard::<DataCard>::init(self.width, self.height));
    }

    // lua
    pub fn set_in_all(&mut self, x: usize, y: usize, tape: usize, action: DataAction){
        if self.state == State::Setup{ return; }

        if tape > self.tapes{
            self.tapes = tape;
        }
        if x >= self.width || y >= self.height{
            return;
        }
        for b in self.board_prototypes.iter_mut(){
            *b.get_mut(x, y) = DataCard(action, tape);
        }
    }

    // lua
    pub fn set_in(&mut self, board: usize, x: usize, y: usize, tape: usize, action: DataAction){
        if self.state == State::Setup{ return; }

        if tape >= self.tapes{
            self.tapes = tape + 1;
        }
        if self.state == State::Setup{ return; }
        *self.board_prototypes[board].get_mut(x, y) = DataCard(action, tape);
    }

    // gotta love those getters and setters
    pub fn get_total(&self) -> usize { self.total }
    pub fn get_count(&self) -> usize { self.count }
    pub fn get_dims(&self) -> (usize, usize) {(self.width, self.height)}

    pub fn get_board_prototype(&mut self, board: usize ) -> &BasicBoard<DataCard>{
        return &self.board_prototypes[board];
    }

    pub fn generate(&mut self) -> Result<Vec<BasicBoard<Card>>, GenerationError>{
        let mut tapes = Vec::new(); 

        tapes.resize_with(self.tapes, || 
            TapeGenerator::new(self.count, self.total, &self.blacklist).generate()
        );

        let mut boards = Vec::new();
        for data_board in &self.board_prototypes{
            let mut clone_val : Vec<Option<usize>> = Vec::with_capacity(self.tapes);
            clone_val.resize(self.tapes, None);
            let mut board = BasicBoard::<Card>::init(self.width, self.height);

            for i in 0..self.width{
                for j in 0..self.height{
                    use GenerationError as GE;
                    let tape_index = data_board[i][j].1;
                    let len = clone_val.len();
                    let clone_val = clone_val
                        .get_mut(tape_index)
                        .ok_or_else(|| GE::out_of_bounds(format!("clone_val[{tape_index}] is out of bounds [0, {})!", len)))?;

                    use DataAction as DA;
                    board[i][j] = Card::Value(match ( data_board[i][j].0, &clone_val) {
                        (DA::CloneMark, None) => {
                            let last = tapes[tape_index].0
                                .pop()
                                .ok_or_else(|| GE::generation_tape("tape ran out of values"))?;
                            *clone_val = Some(last);
                            last
                        },
                        (DA::CloneMark, Some(v)) => *v,
                        (DA::Forced(v), _) => v,
                        (DA::NotSpecial, _) => tapes[tape_index].0
                            .pop()
                            .ok_or_else(|| GE::generation_tape("tape ran out of values"))?,
                    });
                }
            }

            boards.push(board)
        }

        Ok(boards)
    }
}
