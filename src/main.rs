use loteria_engine::engine::{BoardBuilder, RandomCenterMarkPair};

fn main() {
    let board = BoardBuilder::new().set_total(10).act_on(RandomCenterMarkPair);
    println!("{board?}");
}
