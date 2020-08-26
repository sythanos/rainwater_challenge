use std::f32;
use std::ops::Sub;

#[derive(Debug)]
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
    pub fn rain(&mut self, rain_hours: f32) -> f32 {
        self.rain = vec![rain_hours; self.columns.len() - 2];
        let mut backdraft = 0.;

        for rain_index in 0..self.rain.len() {
            if self.rain[rain_index] == 0. {
                continue;
            }

            backdraft += self.flow(rain_index + 1, 0.);
        }

        backdraft
    }

    /// Grabs the rain from the rain bank in the environemnt
    ///
    /// Will drain the bank if used. After that calling `new_rain` for the same field will
    /// reutrn 0
    fn new_rain(&mut self, curr_pos: usize) -> f32 {
        let rain_water = self.rain[curr_pos - 1];
        if rain_water != 0. {
            self.rain[curr_pos - 1] = 0.;
        }
        rain_water
    }

    fn flow(&mut self, curr_pos: usize, mut rain_water: f32) -> f32 {
        if curr_pos >= self.columns.len() - 1 {
            return rain_water;
        }

        // Update rain water and walk forward if there is no rainwater
        rain_water += self.new_rain(curr_pos);
        if rain_water < f32::EPSILON {
            return self.flow(curr_pos + 1, 0.);
        }

        let prev_col = self.columns[curr_pos - 1];
        let curr_col = self.columns[curr_pos];
        let next_col = self.columns[curr_pos + 1];

        let diff_left = prev_col - curr_col;
        let diff_right = next_col - curr_col;

        if prev_col > curr_col && next_col > curr_col {
            // Single Width Valley - If there is backwater return it
            return self.handle_valley(curr_pos, rain_water, diff_left, diff_right, curr_pos + 1);
        } else if prev_col >= curr_col && next_col < curr_col {
            // Downwards -
            self.handle_downwards(curr_pos, rain_water);
        } else if prev_col > curr_col {
            // Start of a Plateau -
            return self.handle_plateau(curr_pos, rain_water, diff_left);
        } else if prev_col < curr_col && next_col > curr_col {
            // Upwards - Return all water for now
            return rain_water;
        } else {
            println!("Curr Pos: {}, rainwater: {}", curr_pos, rain_water);
            println!("diff_left: {}, diff_right: {}", diff_left, diff_right);
            println!("Env: {:?}", self);
            unimplemented!("ERROR: All cases should be handled");
        }

        0.
    }

    /// An internal method to Handle a Valley case.
    ///
    /// A indirect recursive function which handles a valley case in the calculation of the
    /// water level in a vallye.
    ///
    /// # Geography
    /// A column is considered to be a valley if and only if both its left and right side are strictly larger then
    /// the water level in the valley itself.
    ///
    /// # Water Level
    /// The water level fills up to the lowest of the 2 sides, given that there is enough rain water. If there is
    /// rain water remaining it will spill into the lower column.
    ///
    /// # Backwater
    /// Any water that cannot be returned will be backtracked, returned to the caller.
    fn handle_valley(
        &mut self,
        curr_pos: usize,
        mut rain_water: f32,
        left_diff: f32,
        right_diff: f32,
        end_pos: usize,
    ) -> f32 {
        let new_water = f32::min(
            rain_water / (end_pos - curr_pos) as f32,
            f32::min(left_diff, right_diff),
        );

        for pos in curr_pos..end_pos {
            self.columns[pos].add_water(new_water);
            rain_water -= new_water;
        }

        if rain_water > 0. {
            if right_diff > left_diff {
                return rain_water;
            }
            return self.handle_plateau(curr_pos, rain_water, left_diff);
        }

        rain_water = self.flow(end_pos, 0.0);
        if rain_water > 0. {
            self.flow(curr_pos, rain_water);
        }

        0.
    }

    /// An internal method to handle a full plateau.
    ///
    /// Handles a plateu starting with a decrease in height followed by at least 1 unit of equal height.
    ///
    /// The plateu can be either followed by an increase or further decrease.
    fn handle_plateau(&mut self, curr_pos: usize, mut rain_water: f32, left_diff: f32) -> f32 {
        let mut end_pos = curr_pos + 1;
        while self.columns[curr_pos] == self.columns[end_pos] {
            rain_water += self.new_rain(end_pos);
            end_pos += 1;
        }

        let right_diff = self.columns[end_pos] - self.columns[curr_pos];

        if right_diff > 0. {
            return self.handle_valley(curr_pos, rain_water, left_diff, right_diff, end_pos);
        } else {
            unimplemented!("Downard Slope")
        }
    }

    /// An internal method to Handle Downwards Case.
    ///
    /// Downwards case is when left water level is equal or more to the one at the current
    /// position and right water level is strictly less.
    fn handle_downwards(&mut self, curr_pos: usize, rain_water: f32) {
        let mut backwater = self.flow(curr_pos + 1, rain_water);

        while backwater > 0. {
            backwater = self.flow(curr_pos, backwater);
        }
    }
}

#[derive(Copy, Clone, Debug)]
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

impl Sub for Column {
    type Output = f32;

    fn sub(self, rhs: Self) -> f32 {
        self.water_level() - rhs.water_level()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq as approx_eq;

    #[test]
    fn test_handle_valley_overflow_left() {
        let mut env = Environment::new(vec![3, 1]);
        env.rain = vec![0., 2.];

        let backwater = env.flow(2, 2.0);
        approx_eq!(backwater, 2.);
    }

    #[test]
    fn test_handle_valley_overflow_right() {
        let mut env = Environment::new(vec![3, 1, 2]);
        env.rain = vec![0., 0., 0.];

        let backwater = env.flow(2, 2.0);
        approx_eq!(backwater, 0.);

        approx_eq!(env.water_level(2), 2.5);
        approx_eq!(env.water_level(3), 2.5);
    }

    #[test]
    fn test_handle_valley_complete_overflow() {
        let mut env = Environment::new(vec![3, 1, 2]);
        env.rain = vec![0., 0., 0.];

        let backwater = env.flow(2, 4.0);
        approx_eq!(backwater, 1.);

        approx_eq!(env.water_level(2), 3.);
        approx_eq!(env.water_level(3), 3.);
    }

    #[test]
    fn test_handle_plateau_valley_no_overflow() {
        let mut env = Environment::new(vec![4, 2, 2]);
        env.rain = vec![0., 0., 0.];

        let backwater = env.flow(2, 3.0);
        approx_eq!(backwater, 0.);

        approx_eq!(env.water_level(2), 3.5);
        approx_eq!(env.water_level(3), 3.5);
    }

    #[test]
    fn test_handle_plateau_valley_with_overflow() {
        let mut env = Environment::new(vec![4, 2, 2]);
        env.rain = vec![0., 0., 0.];

        let backwater = env.flow(2, 5.0);
        approx_eq!(backwater, 1.);

        approx_eq!(env.water_level(2), 4.0);
        approx_eq!(env.water_level(3), 4.0);
    }

    #[test]
    fn test_complex_relief_no_rain() {
        let mut env = Environment::new(vec![4, 2, 7, 8, 8, 7, 2, 4, 5, 1]);
        let backwater = env.rain(0.0);
        approx_eq!(backwater, 0.);
    }

    #[test]
    fn test_1_cols_1_water() {
        let mut env = Environment::new(vec![1]);

        let backwater = env.rain(1.0);
        approx_eq!(backwater, 0.);

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

        let backwater = env.rain(1.0);
        approx_eq!(backwater, 0.);

        approx_eq!(env.water_level(1), 3.0);
        approx_eq!(env.water_level(2), 3.0);
    }

    #[test]
    fn test_31_cols_2_water() {
        let mut env = Environment::new(vec![3, 1]);

        let backwater = env.rain(2.0);
        approx_eq!(backwater, 0.);

        approx_eq!(env.water_level(1), 4.0);
        approx_eq!(env.water_level(2), 4.0);
    }

    #[test]
    fn test_13_cols_1_water() {
        let mut env = Environment::new(vec![1, 3]);

        let backwater = env.rain(1.0);
        approx_eq!(backwater, 0.);

        approx_eq!(env.water_level(1), 3.0);
        approx_eq!(env.water_level(2), 3.0);
    }

    // #[test]
    // fn test_13_cols_2_water() {
    //     let mut env = Environment::new(vec![1, 3]);

    //     let backwater = env.rain(2.0);
    //     approx_eq!(backwater, 0.);

    //     approx_eq!(env.water_level(1), 4.0);
    //     approx_eq!(env.water_level(2), 4.0);
    // }
}
