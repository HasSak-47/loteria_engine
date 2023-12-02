mod engine;
mod log;

use engine::*;


fn main(){
    let boards = BoardBuilder::new()
        .set_total(8)
        .act_on(UpperCenterMarkPair)
        .generate_tape()
        .generate_boards();

    for board in boards{
        println!("{board}");
    }

}
