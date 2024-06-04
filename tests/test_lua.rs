use loteria_engine::engine::*;

const LUA_CODE: &str = "
local i = 1
for i=1, 4 do
    local j = 1
    for j=1,16 do
        board_prototypes[i][j] = j
    end
end
";

#[test]
fn lua_test() {
    let mut builder = BoardBuilder::new()
        .act_on(SetTotal(4))
        .act_on(SetCount(54))
        .act_on(LuaActor::new(LUA_CODE));

    let boards = builder
        .generate_tapes()
        .generate_boards();


    let mut output_board = Board::new_copy(Card::Value(1));
    for i in 0..16{
        output_board.get_mut(i).set((i + 1) as u8);
    }
    for board in &boards{
        assert_eq!(output_board, boards[0], "comparing boards");
    }

}
