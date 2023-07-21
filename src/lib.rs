mod engine;


/*

use rand::random;
use image::RgbImage;

const NONE: u8 = 255;
static mut DEBUG: bool = false;

fn debug() -> bool { unsafe{DEBUG} }

fn rand_card() -> u8 { random::<u8>() % 54 }

// needs refactoring
fn non_repeating(tape: &[u8]) -> u8{
    'search: loop{
        let id = rand_card();
        for val in tape{
            if *val == id{
                continue 'search;
            }
        }
        return id;
    }
}

fn eat_15(tape: &mut Vec<u8>) -> [u8; 16]{
    let mut result = [NONE; 16];
    for i in 0..15{
        result[i] = tape.pop().unwrap(); 
    }

    let c : usize = random::<usize>() % 15;
    result[15] = result[c];

    let swap_coord = (random::<usize>() % 2, random::<usize>() % 2);
    let swap_index = swap_coord.0 + swap_coord.1 * 4;

    //swapp
    let aux = result[swap_index];
    result[swap_index] = result[15];
    result[15] = aux;

    result
}

fn eat_16(tape: &mut Vec<u8>) -> [u8; 16]{
    let mut result = [NONE; 16];
    for i in 0..16{
        result[i] = tape.pop().unwrap(); 
    }

    result
}

fn main() {
    let args : Vec<_> = std::env::args().into_iter().collect();

    let (duplicates, unique) = match args.len() {
        3 => {
            (usize::from_str_radix(args[1].as_str(), 10).expect("duplicate not a number"),
            usize::from_str_radix(args[2].as_str(), 10).expect("duplicate not a number"))
        },
        4 => {
            if args[1] == "--debug" {
                unsafe {DEBUG = true;}
            }
            (usize::from_str_radix(args[2].as_str(), 10).expect("duplicate not a number"),
            usize::from_str_radix(args[3].as_str(), 10).expect("duplicate not a number"))
        },
        0..=2 => {
            println!("usage: cmd [duplicate] [unique]");
            return;
        }
        _ =>{
            println!("too many arguments!");
            return;
        }
    };
    let complete = (duplicates * 15) + (unique * 16);
    if complete < 54 {
        println!("does not use 54 cards!");
        return;
    }
    let total = (duplicates + unique) * 16;

    //creates tape
    let mut tape : Vec<u8> = Vec::new();
    tape.resize(total, NONE);

    //populate tape with 54 steps
    for i in 0..total{
        let min = (54.0 * (i as f32/ 54.0).floor()) as usize;
        tape[i] = non_repeating(&tape[min..i]);
    }
        
    let mut boards : Vec<[u8; 16]> = Vec::new();

    // creates both unique and duplicate boards
    for _ in 0..unique{
        boards.push(eat_16(&mut tape));
    }

    for _ in 0..duplicates{
        boards.push(eat_15(&mut tape));
    }

    if debug(){
        for (index, board) in (&boards).into_iter().enumerate(){
            let _type = if index >= unique { "duplicate" } else { "unique" };
            println!("board {index}: {board:03?} type: {_type}");
        }
    }

    // load images
    let images = {
        let mut o : Vec<_> = Vec::new();
        o.reserve(54);
        for i in 0..54{
            let path = format!("images/out.image-{i:03}.png");
            o.push(image::open(path).unwrap().into_rgb8());
        }
        o
    };

    // create image buffer
    let (width, height) = images[0].dimensions();
    let mut image = RgbImage::new(width * 4, height * 4);

    // save images
    if !debug() {
        return;
    }
    for iter in boards.iter().zip(0..boards.len()) {
        let brd = iter.0;
        let id = iter.1;
        for indx in 0..16{
            let i = indx % 4;
            let j = indx / 4;
            for w in 0..width{ for h in 0..height {
                *image.get_pixel_mut(w + i * width, h + j * height) = 
                *images[brd[indx as usize] as usize].get_pixel(w, h);
           }}
       }
       
       image.save(format!("out/out-{id:03}.png")).unwrap();
    }
}
*/
