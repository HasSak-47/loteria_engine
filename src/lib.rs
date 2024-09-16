pub mod engine;
pub mod tape;
pub mod error;
pub mod board;
pub mod random;

use board::BasicBoard;
use engine::{BoardBuilder, Card, DataAction};
use mlua::{self, Lua};
use anyhow::{anyhow, Result};

pub static mut BOARD : engine::BoardBuilder = BoardBuilder::new();

/*
pub mod ffi{
    use std::ffi::{c_char, CStr};
    use board::BasicBoard;
    pub extern "C" fn c_init(ctr: *mut c_char){
        unsafe{
            let code = CStr::from_ptr(ctr).to_str().unwrap();
            init(code).unwrap();
        }
    }
    
    pub extern "C" fn get_prototypes(id: usize) -> *const BasicBoard<DataCard>{
        unsafe{
            return BOARD.get_board_prototype(id);
        }
    }
}
use ffi;
*/

pub fn init(code: &str) -> Result<()>{
    let vm = Lua::new();

macro_rules! IMPL {
    ($function: tt, $($val: tt: $type: ty),*) => {
        vm.globals().set(stringify!($function), vm.create_function_mut(|_, info: ($($type)*,)| {
            unsafe{ BOARD.$function( $(info.$val,)* ); }
            Ok(())
        })? )?;
    };
    ($function: tt) => {
        vm.globals().set(stringify!($function), vm.create_function_mut(|_, ()| {
            unsafe{ BOARD.$function(); }
            Ok(())
        })? )?;
    };
    ($name: literal, $lambda: expr) => {
        vm.globals().set($name, vm.create_function_mut($lambda)? )?;
    }
}

    IMPL!(set_width , 0 : usize);
    IMPL!(set_height, 0 : usize);
    IMPL!(set_count , 0 : usize);
    IMPL!(set_total , 0 : usize);
    IMPL!("set_in_all", |_, params: (usize, usize, usize, String, Option<usize>)|{unsafe{
            BOARD.set_in_all(params.0, params.1, params.2,
                match (params.3.as_str(), params.4) {
                    ("CloneMark", _) => DataAction::CloneMark,
                    ("Forced", Some(val)) => DataAction::Forced(val),
                    _ => DataAction::NotSpecial,
                }
            );
        }
        Ok(())
    });

    IMPL!("set_in", |_, params: (usize, usize, usize, usize, String, Option<usize>)|{unsafe{
            BOARD.set_in(params.0, params.1, params.2, params.3,
                match (params.4.as_str(), params.5) {
                    ("CloneMark", _) => DataAction::CloneMark,
                    ("Forced", Some(val)) => DataAction::Forced(val),
                    _ => DataAction::NotSpecial,
                }
            );
        }
        Ok(())
    });


    IMPL!(init);

    vm.load(code).exec()?;
    drop(vm);

    return Ok(());
}

pub fn run() -> Result<Vec<BasicBoard<Card>>>{ unsafe{
    Ok(BOARD.generate()?)
}}
