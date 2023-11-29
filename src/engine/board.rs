#[derive(Default, Debug, Clone, Copy)]
pub struct BasicBoard<T>([T; 16]);

#[allow(dead_code)]
impl<T> BasicBoard<T>{
    pub fn get(&self, i: usize) -> &T{ &self.0[i] }
    pub fn get_mut(&mut self, i: usize) -> &T{ &mut self.0[i] }
}

impl<T> std::ops::Index<usize> for BasicBoard<T>{
    type Output = [T];
    fn index(&self, index: usize) -> &Self::Output {
        let index = index * 4;
        &self.0[index..index+4]
    }
}

impl<T> std::ops::IndexMut<usize> for BasicBoard<T>{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let index = index * 4;
        &mut self.0[index..index+4]
    }
}
