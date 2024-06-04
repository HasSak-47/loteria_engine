use anyhow::*;
use super::*;

const CARD_COUNT : usize = 54;

fn test_tape(total: usize) -> Result<()>{
    let t = BoardBuilder::new()
        .act_on(SetTotal(total))
        .act_on(SetCount(CARD_COUNT))
        .generate_tapes();
    let tape = &t.tapes[0];
    // check if each 16 card chunck is unique
    println!("tape: ");
    for i in 0..tape.0.len(){
        print!("{:2} ", tape.0[i]);
        if i % 16 == 15{
            println!();
        }
    }
    println!();
    for i in 0..total{
        let start = i * 16;
        let end = start + 16;
        let chunk = &tape.0[start..end];
        println!("chunk: {chunk:?}");
        let mut count = [ 0; CARD_COUNT];
        for j in start..end{
            let v = tape.0[j];
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

fn test_boards(total: usize) -> Result<()>{
    let boards = BoardBuilder::new()
        .act_on(SetCount(CARD_COUNT))
        .act_on(SetTotal(total))
        .generate_tapes()
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
