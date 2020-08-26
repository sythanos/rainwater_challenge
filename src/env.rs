use std::slice::IterMut;

pub struct Environment {
    columns: Vec<Column>,
}

impl Environment {
    /// Constructs a new `Environment`
    pub fn new(columns: Vec<u32>) -> Self {
        Self {
            columns: columns
                .iter()
                .map(|height| Column::new(*height as f32))
                .collect::<Vec<Column>>(),
        }
    }

    /// Returns the Water Level of the environment at position `pos`.
    pub fn water_level(&self, pos: usize) -> f32 {
        self.columns[pos].water_level()
    }

    /// Accepts the number of hours it has rain and mutate the environment to
    /// its endstate.
    pub fn rain(&mut self, rain_hours: f32) {
        let mut rain = vec![rain_hours; self.columns.len()];

        Self::flow(self.columns.iter_mut(), rain.iter_mut());
    }

    fn flow(mut curr_col: IterMut<Column>, mut rain: IterMut<f32>) {
        let rain_water = *(rain.next().unwrap_or(&mut 0.));

        match curr_col.next() {
            Some(col) => col.add_water(rain_water),
            None => return,
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

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq as approx_eq;

    #[test]
    fn test_1_cols_1_water() {
        let mut env = Environment::new(vec![1]);
        env.rain(1.0);
        approx_eq!(env.water_level(0), 2.0)
    }

    #[test]
    fn test_11_cols_1_water() {
        let mut env = Environment::new(vec![1, 1]);
        env.rain(1.0);
        approx_eq!(env.water_level(0), 2.0)
    }

    // #[test]
    // fn test_21_cols_1_water() {
    //     let mut env = Environment::new(vec![2, 1]);
    //     env.rain(1.0);
    //     approx_eq!(env.water_level(0), 2.0);
    //     approx_eq!(env.water_level(1), 2.0);
    // }
}
