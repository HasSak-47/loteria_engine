pub mod engine;

#[cfg(test)]
mod tests{
    use crate::engine::motor::BoardGenerator;

    #[test]
    fn test_board_generator() {
        let motor = BoardGenerator::new();
        for board in motor.boards.into_iter().enumerate(){
            let res1 = board.1.verify();
            let res2 = board.1.verify2();
            let has_error = res1.is_err() || res2.is_err();
            let error =
            match res1 {
                Ok(()) => "".to_string(),
                Err(e) => e,
            } + 
            match res2 {
                Ok(()) => "".to_string(),
                Err(e) => e,
            }.as_str();
            if has_error{
                panic!("\nerror: {error}\nboard index: {}\nboard:\n{}",
                    board.0,
                    board.1,
                );
            }

        }

    }
}
