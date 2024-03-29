use std::f32::consts::PI;

use crate::{Rotator, Vec2, Vec3};

pub(crate) unsafe fn aim(
    camera_pos: *const Vec3,
    aim_pos: *const Vec3,
    yaw: *mut f32,
    pitch: *mut f32,
) {
    pub(crate) static mut ROTATOR: Rotator = Rotator {
        yaw: 0.0,
        pitch: 0.0,
        roll: 0.0,
    };

    pub(crate) static mut POS: Vec3 = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    POS.x = aim_pos.read().x - camera_pos.read().x;
    POS.y = aim_pos.read().z - camera_pos.read().z;
    POS.z = aim_pos.read().y - camera_pos.read().y;

    ROTATOR.yaw = (POS.y / POS.x).atan() * 180.0 / PI;
    ROTATOR.pitch = (POS.z / (POS.x.powi(2) + POS.y.powi(2)).sqrt()).atan() * 180.0 / PI;
    ROTATOR.roll = 0.0;

    if ROTATOR.yaw < 0.0 && POS.y > 0.0 {
        ROTATOR.yaw += 180.0;
    }

    if ROTATOR.yaw > 0.0 && POS.y < 0.0 {
        ROTATOR.yaw -= 180.0;
    }

    *yaw = ROTATOR.yaw;
    *pitch = ROTATOR.pitch;
}

pub(crate) unsafe fn get_crosshair_distance_to(
    obj_screen_pos: *const Vec2,
    screen_width: f32,
    screen_height: f32,
) -> f32 {
    ((obj_screen_pos.read().x - screen_width / 2.0).powf(2.0)
        + (obj_screen_pos.read().y - screen_height as f32 / 2.0).powf(2.0))
    .sqrt()
}
