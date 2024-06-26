use anyhow::*;
use std::{fmt::Display, fs::File, io::Read, path::PathBuf};

use mlua::prelude::*;

use super::{BoardActor, BoardBuilder};

#[derive(Debug, Default)]
pub struct LuaActor{
    src: String,
    lua: Lua,
}

impl LuaActor {
    pub fn new<S: Into<String>>(src: S) -> Self{
        let lua = Lua::new();
        return Self{src: src.into(), lua}
    }
    
    pub fn from_file(path: PathBuf) -> Result<Self>{
        let mut file = File::open(path)?;
        let mut src = String::new();
        file.read_to_string(&mut src)?;

        Ok(Self{src, ..Default::default()})
    }
}

impl BoardActor for LuaActor{
    fn act_on(&self, b: &mut super::BoardBuilder) -> Result<()> {

        self.lua
            .load(&self.src)
            .exec()
            .map_err(|lua_err| anyhow!("[LUA ERROR] : {lua_err}"))?;

        let luatables : LuaTable = self.lua.globals().get("board_prototypes")
            .map_err(|lua_err| anyhow!("[LUA ERROR] : PROTOTYPES NOT FOUND - {lua_err}"))?;
        for (i, table) in b.board_prototypes.iter_mut().enumerate(){
            let t : LuaTable = luatables.get(i + 1)?;
            for i in 0..16{
                // lua starts the indices at 1
                let lua_card : LuaValue = t.get(i + 1)
                    .map_err(|lua_err| anyhow!("[LUA ERROR] : CARD NOT FOUND - {lua_err}"))?;

                use super::DataAction as DC;
                table.get_mut(i).0 = match lua_card{
                    LuaValue::Integer(v) => DC::Forced(v as u8),
                    LuaValue::String(s) => {
                        if s == "CloneMark" { DC::CloneMark }
                        else { DC::NotSpecial }

                    },
                    _ => DC::NotSpecial ,
                };
            }
        }

        Ok(())
    }
}

impl Display for LuaActor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "lua src:\n{}", self.src)
    }
}

#[derive(Debug, Default)]
pub struct LuaScript{
    src: String,
}

impl LuaScript{
    pub fn new<P: Into<PathBuf>>(path: P){
        let path = path.into();
        let mut src = String::new();

        let mut file = File::open(path).unwrap();
        file.read_to_string(&mut src).unwrap();
    }
}

impl BoardActor for LuaScript{
    fn act_on(&self, mut b: BoardBuilder) -> Result<BoardBuilder> {
        let lua = Lua::new();

        let set_count = |l: &Lua, count: usize| -> LuaResult<()> {
            b.set_count_ref(count);
            LuaResult::Ok(())
        };

        Ok(())
    }
}
