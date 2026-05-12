pub const IDLE: Color = Color {
    r: 0,
    g: 204,
    b: 204,
};
pub const STOP_0: Color = Color { r: 0, g: 204, b: 0 };
pub const STOP_1: Color = Color {
    r: 204,
    g: 204,
    b: 0,
};
pub const STOP_2: Color = Color { r: 204, g: 0, b: 0 };
pub const STOP_3: Color = Color { r: 0, g: 0, b: 0 };
pub const DEFAULT_STOPS: &[Stop] = &[
    Stop {
        color: STOP_0,
        progress: 0.0,
    },
    Stop {
        color: STOP_1,
        progress: 0.5,
    },
    Stop {
        color: STOP_2,
        progress: 0.8333,
    },
    Stop {
        color: STOP_3,
        progress: 1.0,
    },
];

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Clone, Copy)]
pub struct Stop {
    color: Color,
    progress: f32,
}

pub enum InterpolationMethod {
    Lerp,
    LinearRGB,
    Oklab,
}

pub struct Transition {
    stops: Vec<Stop>,
    interpolation_method: InterpolationMethod,
}

impl Transition {
    pub fn new(stops: Vec<Stop>, interpolation_method: InterpolationMethod) -> Self {
        Self {
            stops: stops,
            interpolation_method: interpolation_method,
        }
    }

    pub fn current_color(&self, t: f32) -> Color {
        todo!();
    }

    fn lerp(&self, t: f32) -> Color {
        todo!();
    }
}

#[cfg(test)]
use super::*;

#[test]
fn idle_is_correctly_defined() {
    let idle = Color {
        r: 0,
        g: 204,
        b: 204,
    };

    assert_eq!(idle, IDLE);
}

#[test]
fn stop_0_is_correctly_defined() {
    let stop_0 = Color { r: 0, g: 204, b: 0 };

    assert_eq!(stop_0, STOP_0);
}

#[test]
fn stop_1_is_correctly_defined() {
    let stop_1 = Color {
        r: 204,
        g: 204,
        b: 0,
    };

    assert_eq!(stop_1, STOP_1);
}

#[test]
fn stop_2_is_correctly_defined() {
    let stop_2 = Color { r: 204, g: 0, b: 0 };

    assert_eq!(stop_2, STOP_2);
}

#[test]
fn stop_3_is_correctly_defined() {
    let stop_3 = Color { r: 0, g: 0, b: 0 };

    assert_eq!(stop_3, STOP_3);
}

#[test]
fn current_color_is_stop_0_at_t_zero() {
    let transition = Transition::new(DEFAULT_STOPS.to_vec(), InterpolationMethod::Lerp);
    let t: f32 = 0.0;
    let stop_0 = transition.current_color(t);

    assert_eq!(stop_0, STOP_0);
}

#[test]
fn current_color_is_stop_1_at_t_zero_point_five() {
    let transition = Transition::new(DEFAULT_STOPS.to_vec(), InterpolationMethod::Lerp);
    let t: f32 = 0.5;
    let stop_1 = transition.current_color(t);

    assert_eq!(stop_1, STOP_1);
}

#[test]
fn current_color_is_stop_2_at_t_zero_point_eight_three_three_three() {
    let transition = Transition::new(DEFAULT_STOPS.to_vec(), InterpolationMethod::Lerp);
    let t: f32 = 0.8333;
    let stop_2 = transition.current_color(t);

    assert_eq!(stop_2, STOP_2);
}

#[test]
fn current_color_is_stop_3_at_t_one_point_zero() {
    let transition = Transition::new(DEFAULT_STOPS.to_vec(), InterpolationMethod::Lerp);
    let t: f32 = 1.0;
    let stop_3 = transition.current_color(t);

    assert_eq!(stop_3, STOP_3);
}

#[test]
fn current_color_is_correct_at_t_zero_point_twentyfive() {
    let transition = Transition::new(DEFAULT_STOPS.to_vec(), InterpolationMethod::Lerp);
    let t: f32 = 0.25;
    let color = transition.current_color(t);
    let correct_color = Color {
        r: 102,
        g: 204,
        b: 0,
    };

    assert_eq!(color, correct_color);
}

#[test]
fn current_color_is_correct_at_t_zero_point_six_six_six_six() {
    let transition = Transition::new(DEFAULT_STOPS.to_vec(), InterpolationMethod::Lerp);
    let t: f32 = 0.6666;
    let color = transition.current_color(t);
    let correct_color = Color {
        r: 204,
        g: 102,
        b: 0,
    };

    assert_eq!(color, correct_color);
}

#[test]
fn current_color_is_correct_at_t_zero_point_nine_one_six_seven() {
    let transition = Transition::new(DEFAULT_STOPS.to_vec(), InterpolationMethod::Lerp);
    let t: f32 = 0.9167;
    let color = transition.current_color(t);
    let correct_color = Color { r: 102, g: 0, b: 0 };

    assert_eq!(color, correct_color);
}
