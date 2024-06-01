use std::fmt::{Display, Debug};

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct BasicBoard<T>([T; 16]);

impl<T> BasicBoard<T>{
    pub fn get(&self, i: usize) -> &T{ &self.0[i] }
    pub fn get_mut(&mut self, i: usize) -> &mut T{ &mut self.0[i] }

    pub const fn new(data: [T; 16]) -> Self{
        Self(data)
    }
}

impl<T: Copy> BasicBoard<T>{
    pub const fn new_copy(data: T) -> Self{ Self([data; 16]) }
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

impl<T> Display for BasicBoard<T> 
where
    T : Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for ij in 0..16{
            let i = ij % 4;
            write!(f, "{} ", self.0[ij])?;
            if i == 3{
                writeln!(f)?;
            }

        }
        Ok(())
    }
}
