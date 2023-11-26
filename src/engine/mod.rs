pub mod random;

#[derive(Default, Debug, Clone, Copy)]
pub enum Card{
    Value(u8),
    #[default]
    None,
}

impl Card {
    pub fn set(&mut self, v: u8){
        *self = Self::Value(v)
    }

    pub fn unset(&mut self){
        *self = Self::None;
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Board([Card; 16]);

impl Board{
    pub fn get(&self, i: usize) -> &Card{ &self.0[i] }
    pub fn get_mut(&mut self, i: usize) -> &Card{ &mut self.0[i] }
}

impl std::ops::Index<usize> for Board{
    type Output = [Card];
    fn index(&self, index: usize) -> &Self::Output {
        let index = index * 4;
        &self.0[index..index+4]
    }
}

impl std::ops::IndexMut<usize> for Board{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let index = index * 4;
        &mut self.0[index..index+4]
    }
}

#[derive(Default, Debug, Clone)]
struct Logger<T>(T, Vec<String>);

#[allow(dead_code)]
impl Logger<Board> {
    fn set(&mut self, i: usize, j: usize, v: u8){
        self.1.push(format!("({i}, {j}) set to v"));
        self.0[i][j].set(v);
    }

    fn swap(&mut self, i1: usize, j1: usize, i2: usize, j2: usize){
        self.1.push(format!("({i1}, {j1}) swaped with ({i2}, {j2})"));
        let aux = self.0[i1][j1];
        self.0[i1][j1] = self.0[i2][j2];
        self.0[i2][j2] = aux;
    }
}

trait Instruction{
}
