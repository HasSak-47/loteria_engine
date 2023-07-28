var DECK_SIZE = 54;
var image_div = document.getElementById('images');

function get_paths(path) {
    var paths = [];
    for (var i = 0; i < DECK_SIZE; ++i) {
        var zero_str = i < 10 ? '0' : '';
        var image_path = "".concat(path, "/out.image-0").concat(zero_str).concat(i, ".png");
        paths.push(image_path);
    }
}
