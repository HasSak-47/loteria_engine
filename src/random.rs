
use rand::random as rand_random;

pub fn random() -> usize {
    rand_random()
}

pub fn rand_range(min: usize, max: usize) -> usize{
    let range = max - min;

    (random() % range) + min
}

pub fn rand_range_pair(min: usize, max: usize) -> (usize, usize) {
    (
        rand_range(min, max),
        rand_range(min, max)
    )

}
