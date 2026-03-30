use std::ops::{BitOr, BitOrAssign};

#[derive(Clone, Copy, Debug)]
pub struct Range {
    pub min: f64,
    pub max: f64,
}

impl BitOr<Range> for Range {
    type Output = Range;

    fn bitor(self, rhs: Range) -> Self::Output {
        Range {
            min: f64::min(self.min, rhs.min),
            max: f64::max(self.max, rhs.max),
        }
    }
}

impl BitOrAssign<Range> for Range {
    fn bitor_assign(&mut self, rhs: Range) {
        self.min = f64::min(self.min, rhs.min);
        self.max = f64::max(self.max, rhs.max);
    }
}

impl BitOrAssign<f64> for Range {
    fn bitor_assign(&mut self, rhs: f64) {
        self.min = f64::min(self.min, rhs);
        self.max = f64::max(self.max, rhs);
    }
}

impl Range {
    pub fn new(v: f64) -> Range {
        Range { min: v, max: v }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }
}
