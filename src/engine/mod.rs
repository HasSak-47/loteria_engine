pub mod random;
pub mod board;

#[cfg(test)]
pub mod test;

use std::fmt::Display;

use board::BasicBoard;
use crate::engine::random::{rand_range, rand_range_pair};
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

pub type Board = BasicBoard<Card>;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataCard{
    #[default]
    NotSpecial,
    CloneMark,
    Forced(u8),
    Set(u8),
}

pub type DataBoard = BasicBoard<DataCard>;

fn generate_deck(card_count : usize, last_16 : &[u8], blacklist : &[u8]) -> Vec<u8> {
    let mut v = Vec::new();
    let mut cards  : Vec<u8> = (0u8..card_count as u8).collect();
    let mut last_16: Vec<u8> = last_16.into();

    // remove the blacklist from the cards
    for lastc in blacklist.iter(){
        let i = cards.iter().position(|x| x == lastc);
        if i.is_none(){ continue; }
        cards.remove(i.unwrap());
    }

    // remove the last_16 from the cards 
    for lastc in last_16.iter(){
        let i = cards.iter().position(|x| x == lastc);
        if i.is_none(){ continue; }
        cards.remove(i.unwrap());
    }

    while !last_16.is_empty(){
        let index = rand_range(0, last_16.len());
        v.push(last_16.remove(index));
    }

    while !cards.is_empty(){
        let index = rand_range(0, cards.len());
        v.push(cards.remove(index));
    }
    v.reverse();

    v
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct BoardBuilder{
    board_prototypes: Vec<DataBoard>,
    blacklist: Vec<u8>,
    forcelist: Vec<u8>,

    count: usize,

    board_size: usize,
    tape: Vec<u8>,
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

    pub fn act_on<B: BoardActor>(mut self, actor: B) -> Self{
        actor.act_on(&mut self).unwrap();
        self
    }

    pub fn generate_tape(mut self) -> Self{
        let total_cards = 1 + ((self.total * 16) / self.count);
        let mut tape = Vec::new();
        tape.append(&mut generate_deck(self.count, &[], &self.blacklist));
        for _ in 0..total_cards {
            tape.append(&mut generate_deck(self.count, &tape[(tape.len() - 16)..(tape.len())], &self.blacklist));
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

        if self.tape.len() == 0{
            return v;
        }
        for _ in 0..total_cards{
            v.push(self.create_board());
        }

        v
    }
}

pub trait BoardActor{
    fn act_on(&self, b: &mut BoardBuilder) -> Result<(),()>;
}

#[repr(transparent)]
pub struct BoardActorC(
    Box<dyn BoardActor>,
);

macro_rules! new_board_actor {
    ($name:ident, $cname:ident, $($tname: ident: $type:ty), *) => {
        #[repr(C)]
        #[derive(Debug, PartialEq, Eq)]
        pub struct $name(  $( pub(super) $type, )* );

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
    fn act_on(&self, b: &mut BoardBuilder) -> Result<(),()> {
        b.blacklist.push(self.0);
        Ok(()) 
    }
}

impl BoardActor for Force{
    fn act_on(&self, b: &mut BoardBuilder) -> Result<(),()> {
        b.board_size -= 1;
        b.forcelist.push(self.0);
        Ok(()) 
    }
}

impl BoardActor for Set{
    fn act_on(&self, b: &mut BoardBuilder) -> Result<(),()> {
        b.board_size -= 1;
        for board in &mut b.board_prototypes{
            board[self.0][self.1] = DataCard::Set(self.2);
        }
        Ok(()) 
    }
}

impl BoardActor for MarkPair{
    fn act_on(&self, b: &mut BoardBuilder) -> Result<(),()> {
        b.board_size -= 1;
        for board in &mut b.board_prototypes{
            board[self.0][self.1] = DataCard::CloneMark;
            board[self.2][self.3] = DataCard::CloneMark;
        }
        Ok(()) 
    }
}

impl BoardActor for RandomMarkPair{
    fn act_on(&self, b: &mut BoardBuilder) -> Result<(),()> {
        b.board_size -= 1;
        for board in &mut b.board_prototypes{
            let source = rand_range_pair(0, 4);
            let mut target =  rand_range_pair(0, 4);
            while target == source{ target = rand_range_pair(0, 4); }
            board[source.0][source.1] = DataCard::CloneMark;
            board[target.0][target.1] = DataCard::CloneMark;
        }
        Ok(()) 
    }
}

impl BoardActor for RandomCenterMarkPair{
    fn act_on(&self, b: &mut BoardBuilder) -> Result<(),()> {
        b.board_size -= 1;
        for board in &mut b.board_prototypes{
            let source = rand_range_pair(0, 2);
            let mut target =  rand_range_pair(0, 2);
            while target == source{ target = rand_range_pair(0, 2); }
            board[source.0 + 1][source.1 + 1] = DataCard::CloneMark;
            board[target.0 + 1][target.1 + 1] = DataCard::CloneMark;
        }
        Ok(()) 
    }
}

impl BoardActor for UpperCenterMarkPair{
    fn act_on(&self, b: &mut BoardBuilder) -> Result<(),()> {
        for board in &mut b.board_prototypes{
            board[1][1] = DataCard::CloneMark;
            board[2][1] = DataCard::CloneMark;
        }
        Ok(()) 
    }
}

impl BoardActor for LowerCenterMarkPair{
    fn act_on(&self, b: &mut BoardBuilder) -> Result<(),()> {
        for board in &mut b.board_prototypes{
            board[1][2] = DataCard::CloneMark;
            board[2][2] = DataCard::CloneMark;
        }
        Ok(()) 
    }
}

impl BoardActor for SetTotal{
    fn act_on(&self, b: &mut BoardBuilder) -> Result<(),()> {
        b.set_total_ref(self.0);
        Ok(()) 
    }
}

impl BoardActor for SetCount{
    fn act_on(&self, b: &mut BoardBuilder) -> Result<(),()> {
        b.count = self.0;
        Ok(()) 
    }
}

impl BoardActor for SetPair{
    fn act_on(&self, b: &mut BoardBuilder) -> Result<(),()> {
        b.board_size -= 1;
        b.blacklist.push(self.0 as u8);
        let board = &mut b.board_prototypes[self.1];
        for i in 0..16{
            let t = board.get_mut(i);
            *t = DataCard::Set(self.0 as u8);
        }
        Ok(()) 
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

        Ok(())
    }
}
