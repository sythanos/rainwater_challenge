use std::f32;
use std::fmt;
use std::ops::Sub;

/// Environment is the center structure of the program.
///
/// It stores the current state of the program. The Environment consists of a vector of n `Columns`
/// which represent the relief + 2 infinite walls on each side and a rain bank of n-2 values.
///
/// When it rains for every hour rain bank fills up by 1 unit of rain.
#[derive(Debug)]
pub struct Environment {
    columns: Vec<Column>,
    rain: Vec<f32>,
}

impl Environment {
    /// Constructs a new `Environment`
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

    /// Adds Infinite Sides to the start and end of the array
    fn add_sides(mut self) -> Self {
        let mut columns = vec![Column::new(f32::MAX)];
        columns.append(&mut self.columns);
        columns.push(Column::new(f32::MAX));

        self.columns = columns;

        self
    }

    /// Returns the water level of the columns in position `pos`
    #[allow(dead_code)]
    pub fn water_level(&self, pos: usize) -> f32 {
        self.columns[pos].water_level()
    }

    /// Accepts the number of hours it has rain and mutate the environment to its endstate.
    ///
    /// Main Public method of the `Environment`. Calling this method will simulate `rain_hours` hours of rain that
    /// has fallen on the `Environment`.
    ///
    /// It will return remaining water. That value should be 0 if algorithm worked correctly.
    pub fn rain(&mut self, rain_hours: f32) -> f32 {
        self.rain = vec![rain_hours; self.columns.len() - 2];

        let mut backwater = self.flow(1, 0.);
        while backwater > 0. {
            backwater = self.flow(1, backwater);
        }

        return 0.;
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

    /// The entry point for the recursive algorithm. It asks `rain_water` units of water to flow into the colums at position `curr_pos`
    ///
    /// This function is the main recursive functin and it is used to calculate the end distrubtion of the water in colums located
    /// at `curr_pos` if `rain_water` units of water it has falled on it.
    ///
    /// It calls the correct handle method, depending on the topology of the local relief.
    fn flow(&mut self, curr_pos: usize, mut rain_water: f32) -> f32 {
        // println!("FLOW {} {}", curr_pos, rain_water);
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
        } else if prev_col < curr_col && next_col < curr_col {
            // A Single Width Peak
            return self.handle_peak(curr_pos, rain_water, curr_pos + 1);
        } else if prev_col >= curr_col && next_col < curr_col {
            // Downwards -
            return self.handle_downwards(curr_pos, rain_water);
        } else if prev_col < curr_col && next_col == curr_col {
            // Start of the S-Plateau -
            return self.handle_s_plateau(curr_pos, rain_water);
        } else if prev_col > curr_col {
            // Start of a L-Plateau -
            return self.handle_l_plateau(curr_pos, rain_water, diff_left);
        } else if prev_col < curr_col && next_col > curr_col {
            // Upwards - Return all water for now
            return rain_water;
        } else if prev_col == curr_col && next_col >= curr_col {
            // If on level ground just retrack to first slope
            return rain_water;
        } else {
            println!("Curr Pos: {}, rainwater: {}", curr_pos, rain_water);
            println!("diff_left: {}, diff_right: {}", diff_left, diff_right);
            println!("Env: {:?}", self);
            unimplemented!("ERROR: All cases should be handled");
        }
    }

    /// Handles a flat peak relief
    ///
    /// Handles a flat peak streching from `curr_pos` to `end_pos`. Splits the rain water
    /// between left and right.
    fn handle_peak(&mut self, _curr_pos: usize, rain_water: f32, end_pos: usize) -> f32 {
        // println!("PEAK {} {} {}", _curr_pos, end_pos, rain_water);

        let mut backwater = 0.5 * rain_water;
        // println!("PEAK {} {} backwater {}", _curr_pos, end_pos, backwater);
        backwater += self.flow(end_pos, 0.5 * rain_water);
        // println!("PEAK {} {} backwater {}", _curr_pos, end_pos, backwater);
        backwater
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
        // println!("VALLEY {} {} {}", curr_pos, end_pos, rain_water);
        let new_water = f32::min(
            rain_water / (end_pos - curr_pos) as f32,
            f32::min(left_diff, right_diff),
        );

        for pos in curr_pos..end_pos {
            self.columns[pos].add_water(new_water);
            rain_water -= new_water;
        }

        if rain_water > 0. {
            // println!("DIFFS {} {}", left_diff, right_diff);
            if right_diff > left_diff {
                return rain_water;
            } else if right_diff < left_diff {
                return self.flow(curr_pos, rain_water);
            }
            return 0.5 * rain_water + self.flow(end_pos, rain_water * 0.5);
        }

        rain_water = self.flow(end_pos, 0.0);
        if rain_water > 0. {
            return self.flow(curr_pos, rain_water);
        }

        0.
    }

    /// An internal method to handle a full plateau.
    ///
    /// Handles a plateu starting with a decrease in height followed by at least 1 unit of equal height.
    ///
    /// The plateu can be either followed by an increase or further decrease.
    fn handle_l_plateau(&mut self, curr_pos: usize, mut rain_water: f32, left_diff: f32) -> f32 {
        // println!("L PLATEAU {} {} ", curr_pos, rain_water);
        let mut end_pos = curr_pos + 1;
        while self.columns[curr_pos] == self.columns[end_pos] {
            rain_water += self.new_rain(end_pos);
            end_pos += 1;
        }

        let right_diff = self.columns[end_pos] - self.columns[curr_pos];

        if right_diff > 0. {
            return self.handle_valley(curr_pos, rain_water, left_diff, right_diff, end_pos);
        } else {
            let backwater = self.flow(end_pos, rain_water);
            return self.flow(curr_pos, backwater);
        }
    }

    /// An internal method of S-Shaped Plateau
    ///
    /// S- Shaped Plateau is of form
    ///   --------(?)
    ///   |
    ///  (?)
    ///   |
    ///  --
    fn handle_s_plateau(&mut self, curr_pos: usize, mut rain_water: f32) -> f32 {
        // println!("S PLATEAU {} {} ", curr_pos, rain_water);
        let mut end_pos = curr_pos + 1;
        while self.columns[curr_pos] == self.columns[end_pos] {
            rain_water += self.new_rain(end_pos);
            end_pos += 1;
        }

        let right_diff = self.columns[end_pos] - self.columns[curr_pos];

        if right_diff < 0. {
            // A (end_pos - curr_pos) wide peak
            return self.handle_peak(curr_pos, rain_water, end_pos);
        }
        rain_water
    }

    /// An internal method to Handle Downwards Case.
    ///
    /// Downwards case is when left water level is equal or more to the one at the current
    /// position and right water level is strictly less.
    fn handle_downwards(&mut self, curr_pos: usize, rain_water: f32) -> f32 {
        let backwater = self.flow(curr_pos + 1, rain_water);
        return self.flow(curr_pos, backwater);
    }
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut max = 0;
        for col in 1..self.columns.len() - 1 {
            let col = self.columns[col];
            if max < col.water_level() as u32 {
                max = col.water_level() as u32;
            }
        }

        f.write_str("\nSimple Result:\n")?;
        for level in (0..max).rev() {
            for col in 1..self.columns.len() - 1 {
                let col = self.columns[col];
                if col.height as u32 > level {
                    f.write_str("O")?;
                } else if col.water_level() as u32 > level {
                    f.write_str("x")?;
                } else {
                    f.write_str(" ")?;
                }
            }
            f.write_str("\n")?;
        }

        f.write_str("\n Exact Results: \n")?;
        for col in 1..self.columns.len() - 1 {
            let column = self.columns[col];
            f.write_fmt(format_args!(
                "Columns {} has height of {} and water_level at {}\n",
                col,
                column.height,
                column.water_level(),
            ))?;
        }

        Ok(())
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Column {
    pub height: f32,
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
    fn test_handle_valley_overflow_equal() {
        let mut env = Environment::new(vec![3, 1, 1, 3, 1]);
        env.rain = vec![0., 0., 0., 0., 0.];

        let backwater = env.flow(2, 5.0);
        approx_eq!(backwater, 0.5);

        approx_eq!(env.water_level(2), 3.);
        approx_eq!(env.water_level(3), 3.);
        approx_eq!(env.water_level(4), 3.);
        approx_eq!(env.water_level(5), 1.5);
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
    fn test_peak_splitting() {
        let mut env = Environment::new(vec![1, 4, 2]);
        env.rain = vec![0., 0., 0.];

        let backwater = env.flow(2, 1.0);
        approx_eq!(backwater, 0.5);

        approx_eq!(env.water_level(2), 4.);
        approx_eq!(env.water_level(3), 2.5);
    }

    #[test]
    fn test_wide_peak() {
        let mut env = Environment::new(vec![1, 4, 4, 2]);
        env.rain = vec![0., 0., 1., 0.];

        let backwater = env.flow(2, 1.0);
        approx_eq!(backwater, 1.);

        approx_eq!(env.water_level(2), 4.);
        approx_eq!(env.water_level(3), 4.);
        approx_eq!(env.water_level(4), 3.);
    }

    #[test]
    fn test_s_steps_backwater() {
        let mut env = Environment::new(vec![1, 4, 4, 6]);
        env.rain = vec![0., 0., 1., 0.];

        let backwater = env.flow(2, 1.0);
        approx_eq!(backwater, 2.);

        approx_eq!(env.water_level(2), 4.);
        approx_eq!(env.water_level(3), 4.);
        approx_eq!(env.water_level(4), 6.);
    }

    #[test]
    fn test_peak_overflow() {
        let mut env = Environment::new(vec![1, 4, 2]);
        env.rain = vec![0., 0., 0.];

        let backwater = env.flow(2, 5.0);
        approx_eq!(backwater, 3.);

        approx_eq!(env.water_level(2), 4.);
        approx_eq!(env.water_level(3), 4.);
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
    fn test_handle_plateau_downward_no_backwater() {
        let mut env = Environment::new(vec![4, 2, 2, 1]);
        env.rain = vec![0., 0., 0., 0.];

        let backwater = env.flow(2, 1.0);
        approx_eq!(backwater, 0.);

        approx_eq!(env.water_level(2), 2.0);
        approx_eq!(env.water_level(3), 2.0);
        approx_eq!(env.water_level(4), 2.0);
    }

    #[test]
    fn test_handle_plateau_downward_with_backwater() {
        let mut env = Environment::new(vec![4, 2, 2, 1]);
        env.rain = vec![0., 0., 0., 0.];

        let backwater = env.flow(2, 2.0);
        approx_eq!(backwater, 0.);

        approx_eq!(env.water_level(2), 2.333333);
        approx_eq!(env.water_level(3), 2.333333);
        approx_eq!(env.water_level(4), 2.333333);
    }

    #[test]
    fn test_handle_plateau_downward_with_overflow() {
        let mut env = Environment::new(vec![4, 2, 2, 1]);
        env.rain = vec![0., 0., 0., 0.];

        let backwater = env.flow(2, 8.0);
        approx_eq!(backwater, 1.);

        approx_eq!(env.water_level(2), 4.0);
        approx_eq!(env.water_level(3), 4.0);
        approx_eq!(env.water_level(4), 4.0);
    }

    #[test]
    fn test_handle_downwards_with_overflow() {
        let mut env = Environment::new(vec![4, 3, 2, 1]);
        env.rain = vec![0., 0., 0., 0.];

        let backwater = env.flow(2, 7.0);
        approx_eq!(backwater, 1.);

        approx_eq!(env.water_level(2), 4.0);
        approx_eq!(env.water_level(3), 4.0);
        approx_eq!(env.water_level(4), 4.0);
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

    #[test]
    fn test_13_cols_2_water() {
        let mut env = Environment::new(vec![1, 3]);

        let backwater = env.rain(2.0);
        approx_eq!(backwater, 0.);

        approx_eq!(env.water_level(1), 4.0);
        approx_eq!(env.water_level(2), 4.0);
    }

    #[test]
    fn test_37453_cols_2_water() {
        let mut env = Environment::new(vec![3, 7, 4, 5, 3]);

        let backwater = env.rain(2.0);
        approx_eq!(backwater, 0.);

        approx_eq!(env.water_level(1), 6.0);
        approx_eq!(env.water_level(2), 7.0);
        approx_eq!(env.water_level(3), 6.3333333);
        approx_eq!(env.water_level(4), 6.3333333);
        approx_eq!(env.water_level(5), 6.3333333);
    }

    #[test]
    fn test_3_50_50_50_50_50_1_water() {
        let mut env = Environment::new(vec![3, 50, 50, 50, 50, 50, 3]);

        let backwater = env.rain(1.0);
        approx_eq!(backwater, 0.);

        approx_eq!(env.water_level(1), 6.5);
        approx_eq!(env.water_level(2), 50.0);
        approx_eq!(env.water_level(4), 50.0);
        approx_eq!(env.water_level(5), 50.0);
        approx_eq!(env.water_level(6), 50.0);
        approx_eq!(env.water_level(7), 6.5);
    }

    #[test]
    fn test_316489_1_water() {
        let mut env = Environment::new(vec![3, 1, 6, 4, 8, 9]);

        let backwater = env.rain(1.0);
        approx_eq!(backwater, 0.);

        println!("{}", env);
        approx_eq!(env.water_level(1), 4.);
        approx_eq!(env.water_level(2), 4.);
        approx_eq!(env.water_level(3), 6.);
        approx_eq!(env.water_level(4), 6.);
        approx_eq!(env.water_level(5), 8.);
        approx_eq!(env.water_level(6), 9.);
    }

    #[test]
    fn test_123456789_1_water() {
        let mut env = Environment::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);

        let backwater = env.rain(1.0);
        approx_eq!(backwater, 0.);

        println!("{}", env);
        approx_eq!(env.water_level(1), 4.75);
        approx_eq!(env.water_level(2), 4.75);
        approx_eq!(env.water_level(3), 4.75);
        approx_eq!(env.water_level(4), 4.75);
        approx_eq!(env.water_level(5), 5.);
        approx_eq!(env.water_level(6), 6.);
        approx_eq!(env.water_level(7), 7.);
        approx_eq!(env.water_level(8), 8.);
        approx_eq!(env.water_level(9), 9.);
    }
}
