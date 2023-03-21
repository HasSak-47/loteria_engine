use rand;
use image;

fn main() {
    // 10 4x4 54
    let mut order = [0; 15 * 10];
    for i in 0..54{
        loop{
            let mut found = false;
            order[i] = rand::random::<u8>() % 54;
            for j in 0..i{
                if order[i] == order[j]{
                    found = true;
                    break;
                }
            }
            if found{
                continue;
            }
            break;
        }
    }
    for i in 54..15 * 10{
        order[i] = rand::random::<u8>() % 54;
    }

    let mut index = 0;
    let mut results = [[0; 16]; 10];
    for i in 0..10{
        for j in 0..15{
            results[i][j] = order[index];
            index += 1;
        }

        let copy_index = rand::random::<usize>() % 16;
        results[i][15] = results[i][copy_index];
        // let mut buffer = [0; 16];
        // let offset = rand::random::<usize>() % 16;
        // for j in 0..15{
        //     buffer[j] = results[i][j];
        // }

        // for j in 0..15{
        //     results[i][j] = buffer[(j + offset) % 16];
        // }
    }

    println!("{results:?}");

    let mut cards = Vec::new();
    for i in 0..54{
        cards.push(image::open(format!("images/out.image-{i:03}.png")).unwrap().to_rgb8());
    }
    // 100 x 158
    let mut cards10 = [
        image::RgbImage::new(100 * 4, 158 * 4),
        image::RgbImage::new(100 * 4, 158 * 4),
        image::RgbImage::new(100 * 4, 158 * 4),
        image::RgbImage::new(100 * 4, 158 * 4),
        image::RgbImage::new(100 * 4, 158 * 4),
        image::RgbImage::new(100 * 4, 158 * 4),
        image::RgbImage::new(100 * 4, 158 * 4),
        image::RgbImage::new(100 * 4, 158 * 4),
        image::RgbImage::new(100 * 4, 158 * 4),
        image::RgbImage::new(100 * 4, 158 * 4),
    ];

    for i in 0..10usize{
        for j in 0..4usize{
            for k in 0..4usize{
                /*cards[10].get_pixel_mut(j as u32, k as u32) =*/
                let card = &cards[(results[i][j + k * 4] as usize)];
                for ii in 0..100{
                    for jj in 0..158{
                        *cards10[i].get_pixel_mut((ii + j * 100) as u32, (jj + k * 158) as u32) = card.get_pixel(ii as u32, jj as u32).clone();
                    }
                }
            }
        }
        cards10[i].save(format!("out{i}.png")).unwrap();
    }


}
