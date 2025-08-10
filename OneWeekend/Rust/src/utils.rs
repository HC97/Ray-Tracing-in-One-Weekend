use rand::Rng;

pub const PI: f64 = std::f64::consts::PI;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees / 180.0 * PI
}

pub fn randomf(min: f64, max: f64) -> f64 {
    rand::thread_rng().gen_range(min..max)
}

#[derive(Debug, Clone, Copy)]
pub struct Interval {
    min: f64,
    max: f64
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn min(&self) -> f64 {
        self.min
    }

    pub fn max(&self) -> f64 {
        self.max
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && self.max >= x
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && self.max > x
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }

    pub const EMPTY: Interval = Self { min: f64::INFINITY, max : f64::NEG_INFINITY };
    pub const UNIVERSE: Interval = Self { min: f64::NEG_INFINITY, max: f64::INFINITY };
}

impl Default for Interval {
    fn default() -> Self {
        Self { min: f64::NEG_INFINITY, max: f64::INFINITY }
    }
}
