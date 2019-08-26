use std::intrinsics::sqrtf32;

/**
    Math related utilities
*/

pub mod vec;
pub mod matrix;

pub fn normalize(vector: &mut vec::Vec3) {
    let wrap = |mut value: f32| -> f32 {
        while value < -180.0 {
            value += 360.0;
        }
        while value > 180.0 {
            value -= 360.0;
        }

        value
    };

    vector.x = wrap(vector.x);
    vector.y = wrap(vector.y);
    vector.z = wrap(vector.z);
}

pub fn clamp(vector: &mut vec::Vec3) {
    if vector.y > 180.0 {
        vector.y = 180.0;
    } else if vector.y < -180.0 {
        vector.y = -180.0;
    }

    if vector.x > 89.0 {
        vector.x = 89.0;
    } else if vector.x < -89.0 {
        vector.x = -89.0;
    }

    vector.z = 0.0;
}

pub fn calc_angle(source: vec::Vec3, dest: vec::Vec3) -> vec::Vec3 {
    unsafe {
        let mut res = vec::Vec3::empty();
        let delta = source - dest;

        let hyp = (delta.x * delta.x + delta.y * delta.y).sqrt();

        res.x = f32::to_degrees(f32::atan(delta.z / hyp));
        res.y = f32::to_degrees(f32::atan(delta.y / delta.x));
        res.z = 0.0;

        if delta.x >= 0.0 {
            res.y += 180.0;
        }

        res
    }
}