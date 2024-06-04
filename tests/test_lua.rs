use loteria_engine::engine::*;

const LUA_CODE: &str = "
for i=1, 4 do
    print(\"lua: \", i)
    local p = board_prototypes[i]
    for j=1,16 do
        p[i] = i
    end
end
";

#[test]
fn lua_test() {
    let mut builder = BoardBuilder::new();
    builder.set_total_ref(54);
    builder.set_count_ref(4);
    builder.act_on_ref(LuaActor::new(LUA_CODE.to_string()));

    let boards = builder.generate_boards();
    for board in boards{
        println!("{board}");
    }

}
