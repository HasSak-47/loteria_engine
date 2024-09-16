use anyhow::Result;
use loteria_engine::*;

const LUA_CODE: &str = r#"
set_width(4)
set_height(4)
set_total(4)
init()

set_in_all(0, 0, 0, "Forced", 0)
set_in_all(1, 1, 0, "CloneMark", 0)
set_in_all(2, 1, 0, "CloneMark", 0)
for b= 0, 3, 1 do
    for i=1, 2, 1 do
        set_in(b, 0, i, 0, "Forced", 1)
    end
end
"#;

#[test]
fn lua_test() -> Result<()>{
    init(LUA_CODE)?;

    for i in 0..4{ unsafe{
        println!("{}", BOARD.get_board_prototype(i));
    }}

    unsafe{
        BOARD.generate()?;
    }


    Ok(())

}
