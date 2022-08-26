use bevy::prelude::*;

pub fn collide(
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

        for x in intersect_min.x..intersect_max.x {
            for y in intersect_min.y..intersect_max.y {
                let a_index = (((x - a_min.x) + ((y - a_min.y) * (img_a.size().x as i32))) as usize  * 4);
                //println!("compute: ({} - {}) + {} - {} * img_a.size().x = {}", x, a_min.x, y, a_min.y, img_a.size().x as i32);
                //println!("a index: {}", a_index);

                let color_a = img_a.data[a_index];
                let color_b = img_b.data
                    [((x - b_min.x) + ((y - b_min.y) * (img_b.size().y.round() as i32))) as usize * 4];
                if color_a > 20 && color_b > 20 {
                    return true;
                }
            }
        }

        return false;
    }

    return false;
}
