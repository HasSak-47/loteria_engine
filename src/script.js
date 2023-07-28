import init, {Generator} from '../pkg/loteria.js';

async function get_boards() {
    const wasm = await init().catch(console.error);
    let start = Date.now();
    let generator = new Generator(4, 4);
    let delta = Date.now() - start;

    let boards = [];
    let next = generator.next();
    while(next != null){
        boards.push(next)
        next = generator.next();
    }
    

    return boards;
}

function push_board(board){
    let inner_add = "";
    inner_add += '<div class="board">';
    let row_format = '<div class="board-row">';
    for(let i = 0; i < 4; ++i){
        inner_add += '<div class="board-row">';
        let image_class = board.get_row(i).forEach(k => inner_add += `<img src='${paths[k]}'>`);
        inner_add += '</div>';
    }
    inner_add += '</div>';
    console.log(inner_add);
    image_div.innerHTML += inner_add;
}

async function main(){
    let boards = await get_boards();

    boards.forEach(board => push_board(board));
}

main();
