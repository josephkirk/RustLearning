use std::cmp::{max, min};

use super::{IPosition, Position};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct IRect {
    pub x_min: i32,
    pub y_min: i32,
    pub x_max: i32,
    pub y_max: i32,
}

pub struct IRectIter {
    rect: IRect,
    countx: i32,
    county: i32,
    next: i32,
}

impl IRectIter {
    fn from_rect(rect: &IRect, step: i32) -> IRectIter {
        let own_rect = IRect { ..*rect };
        // debug!(target:"Drawing", "Breakdown {:?} into iter", own_rect);
        IRectIter {
            rect: own_rect,
            countx: 0,
            county: 0,
            next: step,
        }
    }
}

impl Iterator for IRectIter {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<(i32, i32)> {
        let current_value = (self.rect.x_min + self.countx, self.rect.y_min + self.county);
        if current_value.1 > self.rect.y_max {
            return None;
        } else {
            if current_value.0 >= self.rect.x_max {
                self.countx = 0;
                self.county += 1;
            } else {
                self.countx += self.next;
            }
            return Some(current_value);
        }
    }
}

impl IRect {
    pub fn new(x_min:i32, y_min:i32, x_max:i32, y_max:i32) ->IRect {
        IRect {
            x_min,
            y_min,
            x_max,
            y_max,
        }
    }
    pub fn unit() -> IRect {
        IRect {
            x_min: 0,
            y_min: 0,
            x_max: 1,
            y_max: 1,
        }
    }

    pub fn new_from_center_position(pos: &IPosition, width: i32, height: i32) -> IRect {
        let half_width = (width as f64 / 2.0).round() as i32;
        let half_height = (height as f64 / 2.0).round() as i32;
        let x_min = pos.x - (width as i32 - half_width);
        let y_min = pos.y - (height as i32 - half_height);
        let x_max = pos.x + half_width;
        let y_max = pos.y + half_height;
        IRect {
            x_min,
            y_min,
            x_max,
            y_max,
        }
    }

    pub fn width(&self) -> i32 {
        self.x_max - self.x_min
    }

    pub fn height(&self) -> i32 {
        self.y_max - self.y_min
    }

    pub fn center(&self) -> IPosition {
        IPosition {
            x: self.x_min + (self.width() as f64 / 2.0).round() as i32,
            y: self.y_min + (self.height() as f64 / 2.0).round() as i32,
        }
    }

    pub fn float_center(&self) -> Position {
        Position {
            x: self.width() as f64 / 2.0,
            y: self.height() as f64 / 2.0,
        }
    }

    pub fn intersect(&self, rect: &IRect) -> Option<IRect> {
        if self.is_intersect(rect) {
            Some(IRect {
                x_min: max(self.x_min, rect.x_min),
                y_min: max(self.y_min, rect.y_min),
                x_max: min(self.x_max, rect.x_max),
                y_max: min(self.y_max, rect.y_max),
            })
        } else {
            None
        }
    }

    pub fn expand_to_contain(&mut self, pos: &IPosition) {
        self.x_min = min(pos.x, self.x_min);
        self.y_min = min(pos.y, self.y_min);
        self.x_max = max(pos.x, self.x_max);
        self.y_max = max(pos.y, self.y_max);
    }

    pub fn fit_to(&mut self, rect: &IRect) {
        self.x_min = rect.x_min;
        self.y_min = rect.y_min;
        self.x_max = rect.x_max;
        self.y_max = rect.y_max;
    }

    pub fn constraint_to(&mut self, rect: &IRect) {
        let intersect_rect = self.intersect(rect);
        match intersect_rect {
            Some(result_rect) => {
                self.x_min = result_rect.x_min;
                self.y_min = result_rect.y_min;
                self.x_max = result_rect.x_max;
                self.y_max = result_rect.y_max;
            }
            _ => {}
        }
    }

    pub fn expand_shrink(&mut self, value:i32) {
        self.x_min -= value;
        self.y_min -= value;
        self.y_max += value;
        self.x_max += value;
    }

    pub fn expand_shrink_as(&self, value:i32) ->IRect {
        let x_min = self.x_min - value;
        let y_min = self.y_min - value;
        let x_max = self.x_max + value;
        let y_max = self.y_max + value;
        IRect {x_min, y_min, x_max, y_max}
    }

    pub fn is_intersect(&self, rect: &IRect) -> bool {
        self.x_min <= rect.x_max
            && self.x_max >= rect.x_min
            && self.y_min <= rect.y_max
            && self.y_max >= rect.y_min
    }

    pub fn is_inside(&self, rect: &IRect) -> bool {
        (rect.x_min < self.x_min && self.x_min < rect.x_max)
            && (rect.y_min < self.y_min && self.y_min < rect.y_max)
            && (rect.x_min < self.x_max && self.x_max < rect.x_max)
            && (rect.y_min < self.y_max && self.y_max < rect.y_max)
    }

    pub fn can_hold(&self, rect: &IRect) -> bool {
        rect.is_inside(self)
    }

    pub fn iter(&self, step: i32) -> IRectIter {
        IRectIter::from_rect(self, step)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct IEclipse {
    pub rect: IRect,
}

impl IEclipse {
    pub fn unit() -> IEclipse {
        IEclipse { rect: IRect::unit() }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct ITriangle {
    pub rect: IRect,
}

impl ITriangle {
    pub fn unit() -> IEclipse {
        IEclipse { rect: IRect::unit() }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    static RECT1: IRect = IRect {
        x_min: 0,
        y_min: 0,
        x_max: 5,
        y_max: 5,
    };
    static RECT2: IRect = IRect {
        x_min: 4,
        y_min: 4,
        x_max: 7,
        y_max: 7,
    };
    static RECT3: IRect = IRect {
        x_min: 1,
        y_min: 1,
        x_max: 3,
        y_max: 3,
    };
    static RECT1_INTERSECT_RECT2: IRect = IRect {
        x_min: 4,
        y_min: 4,
        x_max: 5,
        y_max: 5,
    };
    #[test]
    /// If average min max result should return a round of average result for center
    fn test_rect_center_not_prime() {
        assert_eq!(IPosition { x: 3, y: 3 }, RECT1.center());
    }
    #[test]
    fn test_rect_is_intersect() {
        assert!(RECT1.is_intersect(&RECT2));
    }

    #[test]
    fn test_rect_rev_is_intersect() {
        assert!(RECT2.is_intersect(&RECT1));
    }

    #[test]
    fn test_rect_not_intersect() {
        assert!(!RECT3.is_intersect(&RECT2));
    }

    #[test]
    fn test_rect_is_inside_rect() {
        assert!(RECT3.is_inside(&RECT1));
    }

    #[test]
    fn test_rect_not_inside_rect() {
        assert!(!RECT3.is_inside(&RECT2));
    }

    #[test]
    fn test_rect_intersect_rect() {
        assert_eq!(RECT1_INTERSECT_RECT2, RECT1.intersect(&RECT2).unwrap());
    }

    #[test]
    #[should_panic]
    fn test_rect_not_intersect_rect() {
        RECT3.intersect(&RECT2).unwrap();
    }

    #[test]
    fn test_rect_from_pos() {
        let pos = IPosition { x: 15, y: 15 };
        let width = 10;
        let height = 10;
        let new_rect = IRect::new_from_center_position(&pos, width, height);
        assert_eq!(
            IRect {
                x_min: 10,
                y_min: 10,
                x_max: 20,
                y_max: 20
            },
            new_rect
        );
    }

    #[test]
    fn test_rect_fit_rect() {
        let mut test_rect = IRect {
            x_min: 0,
            y_min: 0,
            x_max: 100,
            y_max: 100,
        };
        test_rect.fit_to(&RECT1);
        assert_eq!(RECT1, test_rect);
    }

    #[test]
    fn test_rect_iter_rect() {
        let mut result_vec = Vec::new();
        let rect = IRect {
            x_min: 0,
            y_min: 0,
            x_max: 2,
            y_max: 2,
        };
        for (x, y) in rect.iter(1) {
            result_vec.push(IPosition { x, y });
        }
        let test_vec = vec![
            IPosition { x: 0, y: 0 },
            IPosition { x: 1, y: 0 },
            IPosition { x: 0, y: 1 },
            IPosition { x: 1, y: 1 },
        ];
        assert_eq!(result_vec, test_vec);
    }
}
