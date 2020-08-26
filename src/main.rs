use column::Column;
use std::collections::VecDeque;

mod column;

pub const RAIN_AMOUNT: f32 = 1.0;

pub fn rain<V: Into<VecDeque<Column>>>(env: V, rain_hours: f32) -> VecDeque<Column> {
    let mut env = env.into();

    if env.len() == 1 {
        env[0].add_water(rain_hours);
        return env;
    }

    let right_env = env.split_off(env.len() / 2);
    merge(rain(env, rain_hours), rain(right_env, rain_hours))
}

fn merge(mut left_env: VecDeque<Column>, mut right_env: VecDeque<Column>) -> VecDeque<Column> {
    let mut result = VecDeque::new();

    let mut left = left_env.pop_back().unwrap();
    let mut right = right_env.pop_front().unwrap();
    let mut left_count = 1.;
    let mut right_count = 1.;

    loop {
        match left_env.pop_back() {
            Some(next) => {
                if left == next {
                    left_count += 1.;
                } else {
                    break;
                }
            }
            None => break,
        }
    }

    loop {
        match right_env.pop_front() {
            Some(next) => {
                if right == next {
                    right_count += 1.;
                } else {
                    break;
                }
            }
            None => break,
        }
    }

    let new_level = (left_count * left.water_level() + right_count * right.water_level())
        / (left_count + right_count);
    if left > right {
        right.add_water(left.request_water((left - right) / 2.));
    } else if right > left {
        left.add_water(right.request_water((right - left) / 2.));
    }
    result.push_front(left);
    result.push_back(right);

    for left in left_env {
        result.push_front(left);
    }

    for right in right_env {
        result.push_back(right)
    }

    result
}

fn main() {
    let env = vec![Column::new(1.), Column::new(4.), Column::new(3.)];
    rain(env, 2.0);
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_1_with_2_water() {
        let env = vec![Column::new(1.)];
        let env = rain(env, 2.0);

        assert_approx_eq!(env[0].water_level(), 3.0);
    }

    #[test]
    fn test_31_with_1_water() {
        let env = vec![Column::new(3.), Column::new(1.)];
        let env = rain(env, 1.0);

        assert_approx_eq!(env[0].water_level(), 3.0);
        assert_approx_eq!(env[1].water_level(), 3.0);
    }

    #[test]
    fn test_13_with_1_water() {
        let env = vec![Column::new(1.), Column::new(3.)];
        let env = rain(env, 1.0);

        assert_approx_eq!(env[0].water_level(), 3.0);
        assert_approx_eq!(env[1].water_level(), 3.0);
    }

    #[test]
    fn test_22_with_1_water() {
        let env = vec![Column::new(2.), Column::new(2.)];
        let env = rain(env, 1.0);

        assert_approx_eq!(env[0].water_level(), 3.0);
        assert_approx_eq!(env[1].water_level(), 3.0);
    }

    #[test]
    fn test_31_with_2_water() {
        let env = vec![Column::new(3.), Column::new(1.)];
        let env = rain(env, 2.0);

        assert_approx_eq!(env[0].water_level(), 4.0);
        assert_approx_eq!(env[1].water_level(), 4.0);
    }

    #[test]
    fn test_321_with_1_water() {
        let env = vec![Column::new(3.), Column::new(2.), Column::new(1.)];
        let env = rain(env, 1.0);

        assert_approx_eq!(env[0].water_level(), 3.0);
        assert_approx_eq!(env[1].water_level(), 3.0);
        assert_approx_eq!(env[2].water_level(), 3.0);
    }

    // #[test]
    // fn test_143_with_2_water() {
    //     let env = vec![Column::new(1.), Column::new(4.), Column::new(3.)];
    //     rain(env, 2.0);
    // }
}
