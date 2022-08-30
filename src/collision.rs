use bevy::prelude::*;

//use crate::debug;

pub fn collide(
    //commands: &mut Commands,
    transform_a: &Transform,
    img_a: &Image,
    transform_b: &Transform,
    img_b: &Image,
) -> bool {
    let a_min = (transform_a.translation.truncate() - img_a.size() / 2.0).as_ivec2();
    let a_max = (transform_a.translation.truncate() + img_a.size() / 2.0).as_ivec2();

    let b_min = (transform_b.translation.truncate() - img_b.size() / 2.0).as_ivec2();
    let b_max = (transform_b.translation.truncate() + img_b.size() / 2.0).as_ivec2();

    // check to see if the two rectangles are intersecting
    if a_min.x < b_max.x && a_max.x > b_min.x && a_min.y < b_max.y && a_max.y > b_min.y {
        //println!("texture desc of a: {:?}", img_a.texture_view_descriptor.as_ref().unwrap());
        //println!("size of img data: {}", img_a.data.len());

        let intersect_min = a_min.max(b_min);
        let intersect_max = a_max.min(b_max);

        //debug::spawn_square(commands, intersect_min, intersect_max, Color::rgb(0.0, 0.0, 0.0));
        //debug::spawn_square(commands, a_min, a_max, Color::rgb(1.0, 0.0, 0.0));
        //debug::spawn_square(commands, b_min, b_max, Color::rgb(0.0, 1.0, 0.0));
        let total_img_a_data_size = ((img_a.size().x as i64 * img_a.size().y as i64 * 4) - 1) as usize;
        let total_img_b_data_size = ((img_b.size().x as i64 * img_b.size().y as i64 * 4) -1) as usize;

        //let mut ret = false;
        for x in intersect_min.x..intersect_max.x {
            for y in intersect_min.y..intersect_max.y {
                let a_index =
                    ((x - a_min.x) + ((y - a_min.y) * (img_a.size().x as i32))) as usize * 4 + 3;
                let b_index = ((x - b_min.x) + ((y - b_min.y) * (img_b.size().y.round() as i32))) as usize * 4 + 3;

                let color_a = img_a.data[total_img_a_data_size - a_index];
                let color_b = img_b.data[b_index];
                if color_a > 1 && color_b > 1 {
                    //debug::spawn_square(commands, IVec2::new(x, y), IVec2::new(x-1, y-1), Color::rgb(0.0, 0.0, 1.0));

                    //println!("a color: {:?} -- b color {:?}", color_a, color_b);
                    //ret = true;
                    return true;
                }
            }
        }

        //return ret;
        return false;
    }

    return false;
}
