use crate::random::rand_range;

pub fn generate_deck(card_count : usize, last_16 : &[usize], blacklist : &[usize]) -> Vec<usize> {
    let mut v = Vec::new();
    let mut cards  : Vec<usize> = (0usize..card_count as usize).collect();
    let mut last_16: Vec<usize> = last_16.into();
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

#[derive(Debug, Default, Clone)]
pub struct TapeGenerator{
    count: usize, // number of cards in deck
    total: usize, // amount of cards to be generated 
    blacklist: Vec<usize>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Tape(
    pub Vec<usize>,
);

impl TapeGenerator {
    pub fn new(count: usize, total: usize, blacklist : &[usize]) -> Self{
        Self{count, total, blacklist: blacklist.into()}
    }

    pub fn generate(self) -> Tape{
        let mut tape = Tape(Vec::new());

        tape.0.append(&mut generate_deck(self.count, &[], &self.blacklist));
        if tape.0.len() < 16{
            return tape;
        }
        for _ in 1..self.total{
            tape.0.append(&mut generate_deck(self.count, &tape.0[(tape.0.len() - 16)..(tape.0.len())], &self.blacklist));
        }

        tape
    }
}
