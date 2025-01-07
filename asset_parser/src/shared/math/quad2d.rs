use glam::Vec2;

use crate::shared::prefabs::item_assembly_prefab::Rect;

#[derive(Clone, Copy)]
pub struct Quad2D {
    pub a: Vec2,
    pub b: Vec2,
    pub c: Vec2,
    pub d: Vec2,
}

impl Quad2D {
    pub fn from_submarine_rectangle(rect: Rect) -> Self {
        Self {
            a: Vec2 {
                x: rect.x as f32,
                y: rect.y as f32,
            },
            b: Vec2 {
                x: (rect.x + rect.width as i32) as f32,
                y: rect.y as f32,
            },
            c: Vec2 {
                x: (rect.x + rect.width as i32) as f32,
                y: (rect.y - rect.height as i32) as f32,
            },
            d: Vec2 {
                x: rect.x as f32,
                y: (rect.y - rect.height as i32) as f32,
            },
        }
    }

    pub fn centroid(&self) -> Vec2 {
        (self.a + self.b + self.c + self.d) / 4.0
    }

    pub fn rotated(self, radians: f32) -> Self {
        let centroid = self.centroid();
        Quad2D {
            a: rotate_point_around_target(self.a, centroid, radians, true),
            b: rotate_point_around_target(self.b, centroid, radians, true),
            c: rotate_point_around_target(self.c, centroid, radians, true),
            d: rotate_point_around_target(self.d, centroid, radians, true),
        }
    }
}

pub fn rotate_point_around_target(
    point: Vec2,
    target: Vec2,
    radians: f32,
    clock_wise: bool,
) -> Vec2 {
    let (mut sin, cos) = radians.sin_cos();
    if !clock_wise {
        sin = -sin;
    }
    let dir = point - target;
    let x = (cos * dir.x) - (sin * dir.y) + target.x;
    let y = (sin * dir.x) - (cos * dir.y) + target.y;
    Vec2 { x, y }
}
