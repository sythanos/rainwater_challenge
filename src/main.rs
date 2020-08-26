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

    // let mut result = Vec::new();

    // let mut left_index = 0;
    // let mut right_index = 0;

    // while left_index < left_env.len() && right_index < right_env.len() {
    //     if left_env[left_index] > right_env[right_index] {
    //         let new_level = (left_env[left_index] - right_env[right_index]) / 2.;
    //         let negative_level = new_level - left_env[left_index].water_level();

    //         left_env[left_index].add_water(right_env[right_index].request_water(negative_level));
    //         result.push(left_env[left_index]);
    //         result.push(right_env[right_index]);
    //         left_index += 1;
    //         right_index += 1;
    //     }
    // }

    // for index in left_index..left_env.len() {
    //     result.push(left_env[index]);
    // }

    // for index in right_index..right_env.len() {
    //     result.push(right_env[index]);
    // }

    // left_env.append(&mut right_env);
    // left_env
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

    // #[test]
    // fn test_143_with_2_water() {
    //     let env = vec![Column::new(1.), Column::new(4.), Column::new(3.)];
    //     rain(env, 2.0);
    // }
}
