use std::f32;
use std::ops::Sub;

#[derive(Clone, Copy, Debug)]
pub struct Column {
    height: f32,
    water: f32,
}

impl Column {
    pub fn new(height: f32) -> Self {
        Self { height, water: 0. }
    }

    pub fn water_level(&self) -> f32 {
        self.height + self.water
    }

    pub fn add_water(&mut self, water: f32) {
        self.water += water;
    }

    pub fn request_water(&mut self, water: f32) -> f32 {
        if self.water > water {
            self.water -= water;
            return water;
        }
        let water = self.water;
        self.water = 0.0;

        water
    }
}

impl PartialEq for Column {
    fn eq(&self, other: &Self) -> bool {
        self.water_level() == other.water_level()
    }
}

impl PartialOrd for Column {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.water_level().partial_cmp(&other.water_level())
    }
}

impl Sub for Column {
    type Output = f32;

    fn sub(self, other: Column) -> f32 {
        self.water_level() - other.water_level()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_column_ops() {
        let mut column = Column::new(4.);
        assert_eq!(column.water_level(), 4.);

        column.add_water(2.5);
        assert_eq!(column.water_level(), 6.5);

        assert_approx_eq!(column.request_water(1.), 1.);
        assert_eq!(column.water_level(), 5.5);

        assert_approx_eq!(column.request_water(4.), 1.5);
        assert_eq!(column.water_level(), 4.);
    }
}
