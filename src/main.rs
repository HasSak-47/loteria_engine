mod engine;
mod log;

use engine::Board;

fn main(){
    let mut b = Board::default();
    for ij in 0..16{
        let i = ij % 4;
        let j = ij / 4;
        b[i][j].set(ij as u8);
    }
    for ij in 0..16{
        let i = ij % 4;
        let j = ij / 4;
        let v = b[i][j];
        print!("{v:?} ");
        if i == 3{
            println!()
        }
    }

    println!("{b:?}");
}
