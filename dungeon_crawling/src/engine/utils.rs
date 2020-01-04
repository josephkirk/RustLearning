use crate::WINDOW_WIDTH;

pub fn xy_idx(x: i32, y: i32) -> usize {
    ((y * WINDOW_WIDTH as i32) + x) as usize
}
