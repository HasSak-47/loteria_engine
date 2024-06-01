pub mod engine;
pub mod log;

pub mod capi {
    use std::ffi::CStr;

    use crate::engine as ce;

    #[repr(C)]
    pub struct BoardBuilderC(
        Option<Box<ce::BoardBuilder>>,
        Option<Box<Vec<[u8; 16]>>>,
        usize,
    );


    #[no_mangle]
    pub extern "C" fn new_builder() -> BoardBuilderC{
        BoardBuilderC(Some(Box::new(ce::BoardBuilder::new())), None, 0)
    }

    #[no_mangle]
    pub extern "C" fn drop_builder(builder: BoardBuilderC){
        drop(builder);
    }

    #[no_mangle]
    pub unsafe extern "C" fn add_actor(builder: *mut BoardBuilderC, actor: *const i8, params: *const u8)  -> bool{
        let actor = unsafe { CStr::from_ptr(actor) }.to_str().unwrap();
        match actor {
            "SetCount" => {
                let count = *params as usize;
                let inside = (*builder).0.take().unwrap();
                (*builder).0 = Some(Box::new(inside.act_on(ce::SetTotal(count))));
            },
            _ => { return false; }
        }

        return true;
    }


    #[no_mangle]
    pub unsafe extern "C" fn build(builder: *mut BoardBuilderC) { 
        let _builder = match (*builder).0.take(){
            Some(b) => b.generate_tapes(),
            None => {
                println!("No builder found!");
                return;
            }
        };

        let v_boards = _builder.generate_boards();
        let mut boards = Vec::new();
        for board in v_boards {
            let mut b = [0u8; 16];
            for i in 0..16 {
                b[i] = board.get(i).unpack();
            }
            boards.push(b);
        }

        (*builder).1 = Some(Box::new(boards));
    }

    #[no_mangle]
    pub unsafe extern "C" fn get_board(builder: *mut BoardBuilderC, index: usize) -> *const u8 {
        match (*builder).1.as_ref() {
            Some(boards) => {
                let board = &boards[index];
                board.as_ptr()
            }
            None => std::ptr::null(),
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn get_boards_len(builder: &mut BoardBuilderC) -> usize {
        match (*builder).1.as_ref() {
            Some(boards) => boards.len(),
            None => 0,
        }
    }
}
