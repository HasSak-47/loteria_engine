const DECK_SIZE = 54;
const image_div = document.getElementById('images');

function get_paths(path: string) {
    let paths : String[]= [];

    for(let i = 0; i < DECK_SIZE; ++i){
        let zero_str= i < 10 ? '0' : '';
        let image_path =  `${path}/out.image-0${zero_str}${i}.png`;
        paths.push(image_path);
    }

}
