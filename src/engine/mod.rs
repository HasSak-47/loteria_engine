pub mod random;
pub mod board;
use board::BasicBoard;

use crate::engine::random::{rand_range, rand_range_pair};

#[derive(Default, Debug, Clone, Copy)]
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
}

pub type Board = BasicBoard<Card>;

#[derive(Default, Debug, Clone, Copy)]
pub enum DataCard{
    #[default]
    NotSpecial,
    CloneMark,
    Set(u8),
}

pub type DataBoard = BasicBoard<DataCard>;

pub trait BoardActor{
    fn act_on(&self, b: &mut BoardBuilder) -> Result<(),()>;
}

#[derive(Default, Debug, Clone)]
pub struct BoardBuilder{
    board: DataBoard,
    blacklist: Vec<u8>,

    board_size: usize,
    tape: Vec<u8>,
    total: usize
}

const CARD_COUNT : usize = 54;

fn generate_deck(last_16 : &[u8], blacklist : &[u8]) -> Vec<u8> {
    let mut v = Vec::new();
    let mut cards  : Vec<u8> = (0u8..CARD_COUNT as u8).collect();
    let mut last_16: Vec<u8> = last_16.into();

    // remove the blacklist from the cards
    println!("removing blacklist!");
    for lastc in blacklist.iter(){
        let i = cards.iter().position(|x| x == lastc);
        if i.is_none(){ continue; }
        cards.remove(i.unwrap());
    }

    // remove the last_16 from the cards 
    println!("removing last cards!");
    for lastc in blacklist.iter(){
        let i = cards.iter().position(|x| x == lastc);
        if i.is_none(){ continue; }
        cards.remove(i.unwrap());
    }

    println!("adding last cards!");
    while !last_16.is_empty(){
        let index = rand_range(0, last_16.len());
        v.push(last_16.remove(index));
    }

    println!("adding remaining cards!");
    while !cards.is_empty(){
        let index = rand_range(0, cards.len());
        v.push(cards.remove(index));
    }
    v.reverse();

    v
}

impl BoardBuilder{
    pub fn new() -> Self{
        Self {board_size: 16, ..Default::default()}
    }

    pub fn set_total(mut self, total: usize) -> Self{
        self.total = total;
        self
    }

    pub fn act_on<B: BoardActor>(mut self, actor: B) -> Self{
        let _ = actor.act_on(&mut self);
        self

    }

    pub fn generate_tape(mut self) -> Self{
        let total_cards = CARD_COUNT / self.total;
        let mut tape = Vec::new();
        tape.append(&mut generate_deck(&[], &self.blacklist));
        for _ in 0..total_cards {
            tape.append(&mut generate_deck(&tape[(tape.len() - 16)..(tape.len())], &self.blacklist));
        }
        self.tape = tape;

        self
    }

    pub fn get_card(&mut self, data_card: DataCard, clone_val: &mut Option<Card>) -> Card{
        use DataCard as DC;
        match data_card{
            DC::NotSpecial => Card::Value(self.tape.remove(0)),
            DC::CloneMark => {
                if clone_val.is_none(){
                    *clone_val = Some(Card::Value(self.tape[0]));
                    Card::Value(self.tape.remove(0))
                }
                else {
                    clone_val.unwrap()
                }
            },
            DC::Set(s) => Card::Value(s),
        }
    }

    pub fn create_board(&mut self) -> Board{
        let mut b = Board::default();
        let mut clone_val = None;
        for ij in 0..16{
            let i = ij % 4;
            let j = ij / 4;

            let data_card = self.board[i][j];
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

pub struct BlackList(pub u8);
pub struct Set(pub usize, pub usize, pub u8);
pub struct MarkPair(pub usize, pub usize, pub usize, pub usize);
pub struct RandomMarkPair;
pub struct RandomCenterMarkPair;
pub struct UpperCenterMarkPair;
pub struct LowerCenterMarkPair;

impl BoardActor for BlackList{
    fn act_on(&self, b: &mut BoardBuilder) -> Result<(),()> {
        b.blacklist.push(self.0);
        Ok(()) 
    }
}

impl BoardActor for Set{
    fn act_on(&self, b: &mut BoardBuilder) -> Result<(),()> {
        b.board_size -= 1;
        b.board[self.0][self.1] = DataCard::Set(self.2);
        Ok(()) 
    }
}

impl BoardActor for MarkPair{
    fn act_on(&self, b: &mut BoardBuilder) -> Result<(),()> {
        b.board_size -= 1;
        b.board[self.0][self.1] = DataCard::CloneMark;
        b.board[self.2][self.3] = DataCard::CloneMark;
        Ok(()) 
    }
}

impl BoardActor for RandomMarkPair{
    fn act_on(&self, b: &mut BoardBuilder) -> Result<(),()> {
        b.board_size -= 1;
        let source = rand_range_pair(0, 4);
        let mut target =  rand_range_pair(0, 4);
        while target == source{ target = rand_range_pair(0, 4); }
        
        b.board[source.0][source.1] = DataCard::CloneMark;
        b.board[target.0][target.1] = DataCard::CloneMark;
        Ok(()) 
    }
}

impl BoardActor for RandomCenterMarkPair{
    fn act_on(&self, b: &mut BoardBuilder) -> Result<(),()> {
        b.board_size -= 1;
        let source = rand_range_pair(0, 2);
        let mut target =  rand_range_pair(0, 2);
        while target == source{ target = rand_range_pair(0, 2); }
        
        b.board[source.0 + 1][source.1 + 1] = DataCard::CloneMark;
        b.board[target.0 + 1][target.1 + 1] = DataCard::CloneMark;
        Ok(()) 
    }
}

impl BoardActor for UpperCenterMarkPair{
    fn act_on(&self, b: &mut BoardBuilder) -> Result<(),()> {
        b.board[1][1] = DataCard::CloneMark;
        b.board[2][1] = DataCard::CloneMark;
        Ok(()) 
    }
}

impl BoardActor for LowerCenterMarkPair{
    fn act_on(&self, b: &mut BoardBuilder) -> Result<(),()> {
        b.board[1][2] = DataCard::CloneMark;
        b.board[2][2] = DataCard::CloneMark;
        Ok(()) 
    }
}
