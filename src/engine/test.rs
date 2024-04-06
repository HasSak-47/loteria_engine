use anyhow::*;
use super::*;

fn test_tape(count: usize) -> Result<()>{
    let t = BoardBuilder::new().act_on(SetCount(count)).generate_tape();
    let tape = t.tape;
    // check if each 16 card chunck is unique
    println!("tape: ");
    for i in 0..tape.len(){
        print!("{:2} ", tape[i]);
        if i % 16 == 15{
            println!();
        }
    }
    println!();
    for i in 0..count{
        let start = i * 16;
        let end = start + 16;
        let chunk = &tape[start..end];
        println!("chunk: {chunk:?}");
        let mut count = [ 0; CARD_COUNT];
        for j in start..end{
            let v = tape[j];
            if count[v as usize] != 0{
                return Err(anyhow!("Failed at {i} with {v}"));
            }
            count[v as usize] += 1;
        }
    }

    return Ok(());
}

#[test]
pub fn test_tapes() -> Result<()>{
    for i in 0..100{
        test_tape(i)?
    }

    Ok(())
}

fn test_board(board: &Board) -> anyhow::Result<()>{
    let mut marks = [0; CARD_COUNT];
    for ij in 0..16{
        let i = ij % 4;
        let j = ij / 4;
        let card = board[i][j];
        let v = card.unpack() as usize;
        if marks[v] != 0{
            return Err(anyhow!("{board:?} failed at {i}, {j} with {v}"));
        }
        marks[v] += 1;
    }

    return Ok(());
}

fn test_boards(count: usize) -> Result<()>{
    let boards = BoardBuilder::new()
        .act_on(SetCount(count))
        .generate_tape()
        .generate_boards();

    for board in boards.iter(){
        test_board(board)?;
    }

    return Ok(());
}

#[test]
pub fn test_boards_all() -> Result<()>{
    for i in 0..100{
        test_boards(i)?;
    }

    Ok(())
}
