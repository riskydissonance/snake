use crate::{Vector2};

pub(crate) struct Rectangle {
    min_x: i32,
    min_y: i32,
    width: i32,
    height: i32,
}

impl Rectangle {
    pub(crate) const fn new(min_x: i32, min_y: i32, width: i32, height: i32) -> Rectangle {
        Rectangle {
            min_x,
            min_y,
            width,
            height,
        }
    }
    pub(crate) fn intersects(&self, point: &Vector2) -> bool {
        (point.x > self.min_x && point.x <= (self.min_x + self.width)) && (point.y > self.min_y && point.y <= (self.min_y + self.height))
    }

}