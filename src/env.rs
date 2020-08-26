use std::f32;

pub struct Environment {
    columns: Vec<Column>,
    rain: Vec<f32>,
}

impl Environment {
    /// Constructs a new `Environment`
    #[allow(dead_code)]
    pub fn new(columns: Vec<u32>) -> Self {
        Self {
            rain: vec![0.; columns.len()],
            columns: columns
                .iter()
                .map(|height| Column::new(*height as f32))
                .collect::<Vec<Column>>(),
        }
        .add_sides()
    }

    fn add_sides(mut self) -> Self {
        let mut columns = vec![Column::new(f32::MAX)];
        columns.append(&mut self.columns);
        columns.push(Column::new(f32::MAX));

        self.columns = columns;

        self
    }

    /// Returns the Water Level of the environment at position `pos`.
    #[allow(dead_code)]
    pub fn water_level(&self, pos: usize) -> f32 {
        self.columns[pos].water_level()
    }

    /// Accepts the number of hours it has rain and mutate the environment to
    /// its endstate.
    #[allow(dead_code)]
    pub fn rain(&mut self, rain_hours: f32) {
        self.rain = vec![rain_hours; self.columns.len()];

        self.flow(1, 0.);
    }

    fn new_rain(&mut self, curr_pos: usize) -> f32 {
        let rain_water = self.rain[curr_pos];
        if rain_water != 0. {
            self.rain[curr_pos] = 0.;
        }
        rain_water
    }

    fn flow(&mut self, curr_pos: usize, mut rain_water: f32) {
        if curr_pos >= self.columns.len() - 1 {
            return;
        }
        rain_water += self.new_rain(curr_pos);

        if self.columns[curr_pos] > self.columns[curr_pos + 1] {
            self.flow(curr_pos + 1, rain_water)
        } else {
            self.columns[curr_pos].add_water(rain_water);
            rain_water = 0.;
            self.flow(curr_pos + 1, rain_water)
        }
    }
}

#[derive(Copy, Clone)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq as approx_eq;

    #[test]
    fn test_1_cols_1_water() {
        let mut env = Environment::new(vec![1]);
        env.rain(1.0);
        approx_eq!(env.water_level(1), 2.0)
    }

    #[test]
    fn test_11_cols_1_water() {
        let mut env = Environment::new(vec![1, 1]);
        env.rain(1.0);
        approx_eq!(env.water_level(1), 2.0)
    }

    #[test]
    fn test_31_cols_1_water() {
        let mut env = Environment::new(vec![3, 1]);
        env.rain(1.0);
        approx_eq!(env.water_level(1), 3.0);
        approx_eq!(env.water_level(2), 3.0);
    }
}
