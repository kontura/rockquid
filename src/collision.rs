use bevy::prelude::*;

//use crate::debug;

//TODO(amatej): use Separating Axis Theorem: for not axis align collision detection
pub fn collide(
    //commands: &mut Commands,
    transform_a: &Transform,
    img_a: &Image,
    transform_b: &Transform,
    img_b: &Image,
) -> bool {
    let a_min = transform_a.translation.truncate() - img_a.size() / 2.0;
    let a_max = transform_a.translation.truncate() + img_a.size() / 2.0;

    let b_min = transform_b.translation.truncate() - img_b.size() / 2.0;
    let b_max = transform_b.translation.truncate() + img_b.size() / 2.0;

    // check to see if the two rectangles are intersecting
    if a_min.x < b_max.x && a_max.x > b_min.x && a_min.y < b_max.y && a_max.y > b_min.y {
        let intersect_min = a_min.max(b_min);
        let intersect_max = a_max.min(b_max);

        //debug::spawn_square(commands, intersect_min, intersect_max, Color::rgb(0.0, 0.0, 0.0));
        //debug::spawn_square(commands, a_min, a_max, Color::rgb(1.0, 0.0, 0.0));
        //debug::spawn_square(commands, b_min, b_max, Color::rgb(0.0, 1.0, 0.0));

        // Since we want to index from top left corner (that is where image data starts)
        // we go from smallest x to largest (moving left to right) and from largest y to
        // smallest (moving top to bottom)
        // to reflect that in the translation from global to local coords..
        let mut y = intersect_min.y;
        while y < intersect_max.y {
            let mut x = intersect_min.x;
            while x < intersect_max.x {
                let mut a_local_index: Vec2 =
                    global_to_local(Vec2::new(x, y), transform_a, img_a.size());
                let mut b_local_index: Vec2 =
                    global_to_local(Vec2::new(x, y), transform_b, img_b.size());
                a_local_index = rotate_index(a_local_index, transform_a, img_a.size());
                b_local_index = rotate_index(b_local_index, transform_b, img_b.size());
                let a_index = (a_local_index.x.floor() * 4.0
                    + ((a_local_index.y.floor()) * (img_a.size().x.floor() * 4.0)))
                    as usize
                    + 3;
                let b_index = (b_local_index.x.floor() * 4.0
                    + ((b_local_index.y.floor()) * (img_b.size().x.floor() * 4.0)))
                    as usize
                    + 3;

                let color_a = img_a.data[a_index];
                let color_b = img_b.data[b_index];
                if color_a >= 1 && color_b >= 1 {
                    //println!("collided at {:?},{:?}", x, y);
                    //debug::spawn_square(
                    //    commands,
                    //    Vec2::new(x, y),
                    //    Vec2::new(x, y),
                    //    Color::rgb(0.0, 0.0, 10.0),
                    //);
                    //ret = true;
                    return true;
                }
                x += 1.0;
            }
            y += 1.0;
        }

        return false;
    }

    return false;
}

// It takes top left indexing and returns top left indexing
fn rotate_index(index: Vec2, img_transform: &Transform, img_size: Vec2) -> Vec2 {
    if img_transform.rotation.to_scaled_axis().z > 3.14 {
        return img_size - index - Vec2::new(1.0, 1.0);
    } else {
        return index;
    }
}

// It takes global bottom left index and returns local top left indexing
fn global_to_local(global: Vec2, img_trans: &Transform, img_size: Vec2) -> Vec2 {
    let top_left_corner = img_trans.translation.truncate()
        - Vec2::new(img_size.x / 2.0, -img_size.y / 2.0)
        - Vec2::new(0.0, 1.0);
    return Vec2::new(global.x - top_left_corner.x, top_left_corner.y - global.y);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generate_image_l() -> Image {
        //image should be:
        // 1, 1, 1, 1,
        // 1, 0, 1, 1,
        // 1, 0, 1, 1,
        // 1, 0, 0, 0
        let vec = vec![
            1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 1,
            0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
        ];
        bevy::render::texture::Image::new(
            bevy::render::render_resource::Extent3d {
                width: 4,
                height: 4,
                depth_or_array_layers: 1,
            },
            bevy::render::render_resource::TextureDimension::D2,
            vec,
            bevy::render::render_resource::TextureFormat::Rgba8Uint,
        )
    }

    fn generate_image_o() -> Image {
        //image should be:
        // 0, 0, 0, 0,
        // 0, 1, 1, 0,
        // 0, 1, 1, 0,
        // 0, 0, 0, 0
        let vec = vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0,
            0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
        ];
        bevy::render::texture::Image::new(
            bevy::render::render_resource::Extent3d {
                width: 4,
                height: 4,
                depth_or_array_layers: 1,
            },
            bevy::render::render_resource::TextureDimension::D2,
            vec,
            bevy::render::render_resource::TextureFormat::Rgba8Uint,
        )
    }

    fn generate_image() -> Image {
        //image should be:
        // 1, 1, 1, 1,
        // 1, 1, 1, 1,
        // 1, 1, 1, 1,
        // 1, 1, 1, 1
        let vec = vec![
            1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1,
            0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0,
            0, 1, 1, 0, 0, 1,
        ];
        bevy::render::texture::Image::new(
            bevy::render::render_resource::Extent3d {
                width: 4,
                height: 4,
                depth_or_array_layers: 1,
            },
            bevy::render::render_resource::TextureDimension::D2,
            vec,
            bevy::render::render_resource::TextureFormat::Rgba8Uint,
        )
    }

    #[test]
    fn collide_test_1() {
        let mut trans_a = Transform::from_xyz(4.0, 4.0, 0.0);
        let trans_b = Transform::from_xyz(4.0, 4.0, 0.0);
        let img_a = generate_image();
        let img_b = generate_image();

        assert_eq!(true, collide(&trans_a, &img_a, &trans_b, &img_b));

        trans_a = Transform::from_xyz(3.0, 4.0, 0.0);
        assert_eq!(true, collide(&trans_a, &img_a, &trans_b, &img_b));

        trans_a = Transform::from_xyz(2.0, 4.0, 0.0);
        assert_eq!(true, collide(&trans_a, &img_a, &trans_b, &img_b));

        trans_a = Transform::from_xyz(1.0, 4.0, 0.0);
        assert_eq!(true, collide(&trans_a, &img_a, &trans_b, &img_b));

        trans_a = Transform::from_xyz(0.0, 4.0, 0.0);
        assert_eq!(false, collide(&trans_a, &img_a, &trans_b, &img_b));

        trans_a = Transform::from_xyz(1.0, 3.0, 0.0);
        assert_eq!(true, collide(&trans_a, &img_a, &trans_b, &img_b));

        trans_a = Transform::from_xyz(1.0, 2.0, 0.0);
        assert_eq!(true, collide(&trans_a, &img_a, &trans_b, &img_b));

        trans_a = Transform::from_xyz(1.0, 1.0, 0.0);
        assert_eq!(true, collide(&trans_a, &img_a, &trans_b, &img_b));

        trans_a = Transform::from_xyz(1.4, 1.0, 0.0);
        assert_eq!(true, collide(&trans_a, &img_a, &trans_b, &img_b));

        trans_a = Transform::from_xyz(1.0, 1.5, 0.0);
        assert_eq!(true, collide(&trans_a, &img_a, &trans_b, &img_b));

        trans_a = Transform::from_xyz(4.0, 1.1, 0.0);
        assert_eq!(true, collide(&trans_a, &img_a, &trans_b, &img_b));

        trans_a = Transform::from_xyz(4.0, 0.0, 0.0);
        assert_eq!(false, collide(&trans_a, &img_a, &trans_b, &img_b));

        trans_a = Transform::from_xyz(0.0, 4.0, 0.0);
        assert_eq!(false, collide(&trans_a, &img_a, &trans_b, &img_b));

        trans_a = Transform::from_xyz(0.0, 0.0, 0.0);
        assert_eq!(false, collide(&trans_a, &img_a, &trans_b, &img_b));

        trans_a = Transform::from_xyz(4.0, 0.1, 0.0);
        assert_eq!(true, collide(&trans_a, &img_a, &trans_b, &img_b));

        trans_a = Transform::from_xyz(0.1, 0.1, 0.0);
        assert_eq!(true, collide(&trans_a, &img_a, &trans_b, &img_b));
    }

    #[test]
    fn collide_test_2() {
        let mut trans_a = Transform::from_xyz(4.0, 4.0, 0.0);
        let trans_b = Transform::from_xyz(4.0, 4.0, 0.0);
        let img_a = generate_image_l();
        let img_b = generate_image_l();

        assert_eq!(true, collide(&trans_a, &img_a, &trans_b, &img_b));

        trans_a = Transform::from_xyz(5.0, 1.0, 0.0);
        assert_eq!(false, collide(&trans_a, &img_a, &trans_b, &img_b));

        trans_a = Transform::from_xyz(6.0, 1.0, 0.0);
        assert_eq!(false, collide(&trans_a, &img_a, &trans_b, &img_b));

        trans_a = Transform::from_xyz(7.0, 1.0, 0.0);
        assert_eq!(false, collide(&trans_a, &img_a, &trans_b, &img_b));

        trans_a = Transform::from_xyz(8.0, 1.0, 0.0);
        assert_eq!(false, collide(&trans_a, &img_a, &trans_b, &img_b));

        trans_a = Transform::from_xyz(4.0, 1.0, 0.0);
        assert_eq!(true, collide(&trans_a, &img_a, &trans_b, &img_b));

        trans_a = Transform::from_xyz(3.0, 7.0, 0.0);
        assert_eq!(false, collide(&trans_a, &img_a, &trans_b, &img_b));

        trans_a = Transform::from_xyz(2.0, 7.0, 0.0);
        assert_eq!(false, collide(&trans_a, &img_a, &trans_b, &img_b));

        trans_a = Transform::from_xyz(2.0, 8.0, 0.0);
        assert_eq!(false, collide(&trans_a, &img_a, &trans_b, &img_b));

        trans_a = Transform::from_xyz(2.0, 6.0, 0.0);
        assert_eq!(true, collide(&trans_a, &img_a, &trans_b, &img_b));

        trans_a = Transform::from_xyz(4.0, 7.0, 0.0);
        assert_eq!(true, collide(&trans_a, &img_a, &trans_b, &img_b));
    }

    #[test]
    fn collide_test_upside_down() {
        let mut trans_a = Transform::from_xyz(4.0, 4.0, 0.0);
        let mut trans_b = Transform::from_xyz(4.0, 4.0, 0.0);
        trans_b.rotate_z(f32::to_radians(180.0));
        let img_a = generate_image();
        let img_b = generate_image_l();

        assert_eq!(true, collide(&trans_a, &img_a, &trans_b, &img_b));

        trans_a = Transform::from_xyz(5.0, 1.0, 0.0);
        assert_eq!(true, collide(&trans_a, &img_a, &trans_b, &img_b));

        trans_a = Transform::from_xyz(3.0, 7.0, 0.0);
        assert_eq!(false, collide(&trans_a, &img_a, &trans_b, &img_b));

        trans_a = Transform::from_xyz(3.0, 6.0, 0.0);
        assert_eq!(true, collide(&trans_a, &img_a, &trans_b, &img_b));

        trans_a = Transform::from_xyz(4.0, 7.0, 0.0);
        assert_eq!(true, collide(&trans_a, &img_a, &trans_b, &img_b));
    }

    #[test]
    fn collide_test_transparent_sides() {
        let mut trans_a = Transform::from_xyz(4.0, 4.0, 0.0);
        let mut trans_b = Transform::from_xyz(4.0, 4.0, 0.0);
        trans_b.rotate_z(f32::to_radians(180.0));
        let img_a = generate_image_o();
        let img_b = generate_image_o();

        assert_eq!(true, collide(&trans_a, &img_a, &trans_b, &img_b));

        trans_a = Transform::from_xyz(5.0, 1.0, 0.0);
        assert_eq!(false, collide(&trans_a, &img_a, &trans_b, &img_b));

        trans_a = Transform::from_xyz(2.0, 4.0, 0.0);
        assert_eq!(false, collide(&trans_a, &img_a, &trans_b, &img_b));

        trans_a = Transform::from_xyz(6.0, 4.0, 0.0);
        assert_eq!(false, collide(&trans_a, &img_a, &trans_b, &img_b));

        trans_a = Transform::from_xyz(5.0, 4.0, 0.0);
        assert_eq!(true, collide(&trans_a, &img_a, &trans_b, &img_b));
    }

    #[test]
    fn collide_test_at_zero() {
        let mut trans_a = Transform::from_xyz(0.0, 0.0, 0.0);
        let mut trans_b = Transform::from_xyz(0.0, 0.0, 0.0);
        trans_b.rotate_z(f32::to_radians(180.0));
        let img_a = generate_image_o();
        let img_b = generate_image_o();

        assert_eq!(true, collide(&trans_a, &img_a, &trans_b, &img_b));

        trans_a = Transform::from_xyz(1.0, -3.0, 0.0);
        assert_eq!(false, collide(&trans_a, &img_a, &trans_b, &img_b));

        trans_a = Transform::from_xyz(-2.0, 0.0, 0.0);
        assert_eq!(false, collide(&trans_a, &img_a, &trans_b, &img_b));

        trans_a = Transform::from_xyz(2.0, 0.0, 0.0);
        assert_eq!(false, collide(&trans_a, &img_a, &trans_b, &img_b));

        trans_a = Transform::from_xyz(1.0, 0.0, 0.0);
        assert_eq!(true, collide(&trans_a, &img_a, &trans_b, &img_b));

        trans_a = Transform::from_xyz(-1.0, 1.0, 0.0);
        assert_eq!(true, collide(&trans_a, &img_a, &trans_b, &img_b));

        trans_a = Transform::from_xyz(-1.0, 2.0, 0.0);
        assert_eq!(false, collide(&trans_a, &img_a, &trans_b, &img_b));
    }

    #[test]
    fn collide_test_at_negative() {
        let mut trans_a = Transform::from_xyz(-4.0, -4.0, 0.0);
        let mut trans_b = Transform::from_xyz(-4.0, -4.0, 0.0);
        trans_b.rotate_z(f32::to_radians(180.0));
        let img_a = generate_image_o();
        let img_b = generate_image_o();

        assert_eq!(true, collide(&trans_a, &img_a, &trans_b, &img_b));

        trans_a = Transform::from_xyz(-3.0, -7.0, 0.0);
        assert_eq!(false, collide(&trans_a, &img_a, &trans_b, &img_b));

        trans_a = Transform::from_xyz(-6.0, -4.0, 0.0);
        assert_eq!(false, collide(&trans_a, &img_a, &trans_b, &img_b));

        trans_a = Transform::from_xyz(-2.0, -4.0, 0.0);
        assert_eq!(false, collide(&trans_a, &img_a, &trans_b, &img_b));

        trans_a = Transform::from_xyz(-3.0, -4.0, 0.0);
        assert_eq!(true, collide(&trans_a, &img_a, &trans_b, &img_b));

        trans_a = Transform::from_xyz(-5.0, -3.0, 0.0);
        assert_eq!(true, collide(&trans_a, &img_a, &trans_b, &img_b));

        trans_a = Transform::from_xyz(-5.0, -2.0, 0.0);
        assert_eq!(false, collide(&trans_a, &img_a, &trans_b, &img_b));
    }

    //TODO(amatej): add collide test with float indexes, such as 4.321, 9.4324

    #[test]
    fn global_to_local_test() {
        let trans = Transform::from_xyz(4.0, 4.0, 0.0);
        assert_eq!(
            Vec2::new(0.0, 3.0),
            global_to_local(Vec2::new(2.0, 2.0), &trans, Vec2::new(4.0, 4.0))
        );
        assert_eq!(
            Vec2::new(3.0, 0.0),
            global_to_local(Vec2::new(5.0, 5.0), &trans, Vec2::new(4.0, 4.0))
        );
        assert_eq!(
            Vec2::new(2.0, 1.0),
            global_to_local(Vec2::new(4.0, 4.0), &trans, Vec2::new(4.0, 4.0))
        );
        assert_eq!(
            Vec2::new(1.0, 0.0),
            global_to_local(Vec2::new(3.0, 5.0), &trans, Vec2::new(4.0, 4.0))
        );
        assert_eq!(
            Vec2::new(3.0, 3.0),
            global_to_local(Vec2::new(5.0, 2.0), &trans, Vec2::new(4.0, 4.0))
        );

        let trans = Transform::from_xyz(13.0, 8.0, 0.0);
        assert_eq!(
            Vec2::new(0.0, 1.0),
            global_to_local(Vec2::new(10.0, 7.0), &trans, Vec2::new(6.0, 2.0))
        );
        assert_eq!(
            Vec2::new(3.0, 0.0),
            global_to_local(Vec2::new(13.0, 8.0), &trans, Vec2::new(6.0, 2.0))
        );

        let trans = Transform::from_xyz(-3.5, -2.0, 0.0);
        assert_eq!(
            Vec2::new(0.0, 0.0),
            global_to_local(Vec2::new(-4.0, -2.0), &trans, Vec2::new(1.0, 2.0))
        );
        assert_eq!(
            Vec2::new(0.0, 1.0),
            global_to_local(Vec2::new(-4.0, -3.0), &trans, Vec2::new(1.0, 2.0))
        );
    }

    #[test]
    fn rotate_index_when_no_rotation_and_no_scale_return_identity() {
        let zero_trans = Transform::from_xyz(0.0, 0.0, 0.0);
        assert_eq!(
            Vec2::new(0.0, 0.0),
            rotate_index(Vec2::new(0.0, 0.0), &zero_trans, Vec2::new(2.0, 2.0))
        );
        assert_eq!(
            Vec2::new(2.0, 2.0),
            rotate_index(Vec2::new(2.0, 2.0), &zero_trans, Vec2::new(2.0, 2.0))
        );
        assert_eq!(
            Vec2::new(20.0, 15.0),
            rotate_index(Vec2::new(20.0, 15.0), &zero_trans, Vec2::new(64.0, 64.0))
        );
        assert_eq!(
            Vec2::new(20.0, 15.0),
            rotate_index(
                Vec2::new(20.0, 15.0),
                &Transform::from_xyz(49.0, 99.0, 0.0),
                Vec2::new(64.0, 64.0)
            )
        );
    }

    #[test]
    fn rotate_index_upside_down() {
        let mut upside_down_trans = Transform::from_xyz(0.0, 0.0, 0.0);
        upside_down_trans.rotate_z(f32::to_radians(180.0));
        assert_eq!(
            Vec2::new(1.0, 1.0),
            rotate_index(Vec2::new(0.0, 0.0), &upside_down_trans, Vec2::new(2.0, 2.0))
        );
        assert_eq!(
            Vec2::new(0.0, 0.0),
            rotate_index(Vec2::new(1.0, 1.0), &upside_down_trans, Vec2::new(2.0, 2.0))
        );
        assert_eq!(
            Vec2::new(1.0, 0.0),
            rotate_index(Vec2::new(0.0, 1.0), &upside_down_trans, Vec2::new(2.0, 2.0))
        );
        assert_eq!(
            Vec2::new(0.0, 1.0),
            rotate_index(Vec2::new(1.0, 0.0), &upside_down_trans, Vec2::new(2.0, 2.0))
        );

        assert_eq!(
            Vec2::new(3.0, 3.0),
            rotate_index(Vec2::new(0.0, 0.0), &upside_down_trans, Vec2::new(4.0, 4.0))
        );
        assert_eq!(
            Vec2::new(0.0, 0.0),
            rotate_index(Vec2::new(3.0, 3.0), &upside_down_trans, Vec2::new(4.0, 4.0))
        );
        assert_eq!(
            Vec2::new(1.0, 1.0),
            rotate_index(Vec2::new(2.0, 2.0), &upside_down_trans, Vec2::new(4.0, 4.0))
        );
        assert_eq!(
            Vec2::new(0.0, 3.0),
            rotate_index(Vec2::new(3.0, 0.0), &upside_down_trans, Vec2::new(4.0, 4.0))
        );
        assert_eq!(
            Vec2::new(1.0, 2.0),
            rotate_index(Vec2::new(2.0, 1.0), &upside_down_trans, Vec2::new(4.0, 4.0))
        );
        assert_eq!(
            Vec2::new(0.0, 2.0),
            rotate_index(Vec2::new(3.0, 1.0), &upside_down_trans, Vec2::new(4.0, 4.0))
        );
    }
}
