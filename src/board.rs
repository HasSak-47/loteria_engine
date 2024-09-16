use std::fmt::{Display, Debug};

#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct BasicBoard<T>{
    width : usize, 
    height: usize,
    data: Vec<T>, 
}

impl<T> BasicBoard<T>{
    pub const fn new() -> Self{
        Self{data: Vec::new(), width: 0, height: 0}
    }

    pub fn get(&self, x: usize, y: usize ) -> &T{
        &self.data[ x + y * self.width ]
    }

    pub fn get_mut(&mut self, x: usize, y: usize ) -> &mut T{
        &mut self.data[ x + y * self.width ]
    }

}

impl<T> BasicBoard<T> where 
    T : Default + Clone,
{
    pub fn init(width: usize, height: usize) -> Self{
        let mut k = Self{data: Vec::new(), width, height};
        k.data.resize(width * height, T::default());
        k
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
