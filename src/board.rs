use std::fmt::{Display, Debug};

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct BasicBoard<T>{
    data: Vec<T>, 
    width: usize, 
    height: usize
}

impl<T> BasicBoard<T>{
    pub const fn new() -> Self{
        Self{data: Vec::new(), width: 0, height: 0}
    }

    pub const fn get(&self, x: usize, y: usize ) -> &T{
        &self.data[ x + y * self.width ]
    }

    pub const fn get_mut(&mut self, x: usize, y: usize ) -> &T{
        &mut self.data[ x + y * self.width ]
    }
}

impl<T> std::ops::Index<usize> for BasicBoard<T>{
    type Output = [T];
    fn index(&self, index: usize) -> &Self::Output {
        let index = index * self.width;
        &self.data[index..index+self.width]
    }
}

impl<T> std::ops::IndexMut<usize> for BasicBoard<T>{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let index = index * self.width;
        &mut self.data[index..index+self.width]
    }
}

impl<T> Display for BasicBoard<T> 
where
    T : Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for j in 0..self.height{
            for i in 0..self.width{
                write!(f, "{} ", self.get(i, j))?;
                if i == self.width - 1{
                    writeln!(f)?;
                }
            }
        }
        Ok(())
    }
}
