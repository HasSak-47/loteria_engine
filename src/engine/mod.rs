pub mod random;
pub mod board;
use board::BasicBoard;

#[derive(Default, Debug, Clone, Copy)]
pub enum Card{
    Value(u8),
    #[default]
    None,
}

#[allow(dead_code)]
impl Card {
    pub fn set(&mut self, v: u8){
        *self = Self::Value(v)
    }

    pub fn unset(&mut self){
        *self = Self::None;
    }
}

pub type Board = BasicBoard<Card>;

enum DataCard{
    NotSpecial,
    SwapMark,
    Set(u8),
}

pub type DataBoard = BasicBoard<DataCard>;

pub trait BoardActors{
    fn act_on(&self, b: &mut DataBoard) -> Result<(),()>;
}

pub struct BoardBuilder{
    board: DataBoard,
    blacklist: Vec<u8>,
    forcelist: Vec<u8>,

    board_size: usize,
    command_queue: Vec<Box<dyn BoardActors>>,
    tape: Vec<u8>,
}

pub struct BlackList(usize);
pub struct Set(usize);
pub struct MarkPair(usize, usize, usize, usize);
pub struct RandomMarkPair;
pub struct RandomCenterMarkPair;
pub struct RandomUpperCenterMarkPair;
pub struct RandomLowerCenterMarkPair;
pub struct Generate;
pub struct Swap;
