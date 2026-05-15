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
        progress: 0.833,
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

#[derive(Clone, Copy, Debug)]
pub struct Stop {
    color: Color,
    progress: f32,
}

pub enum InterpolationMethod {
    Lerp,
    LinearRGB,
    Oklab,
}

pub struct Theme {
    pub stops: Vec<Stop>,
    pub idle: Color,
    interpolation_method: InterpolationMethod,
}

impl Theme {
    pub fn new(stops: Vec<Stop>, idle: Color, interpolation_method: InterpolationMethod) -> Self {
        Self {
            stops: stops,
            idle: idle,
            interpolation_method: interpolation_method,
        }
    }

    pub fn current_color(&self, t: f32) -> Color {
        match self.interpolation_method {
            InterpolationMethod::Lerp => Self::lerp(&self, t),
            _ => todo!(),
        }
    }

    fn lerp(&self, t: f32) -> Color {
        debug_assert!(t < 1.0);
        for i in 0..self.stops.len() - 1 {
            if self.stops[i].progress <= t && t < self.stops[i + 1].progress {
                let ratio: f32 = (t - self.stops[i].progress)
                    / (self.stops[i + 1].progress - self.stops[i].progress);
                let r: f32 = self.stops[i].color.r as f32
                    + ((self.stops[i + 1].color.r as f32 - self.stops[i].color.r as f32) * ratio);
                let g: f32 = self.stops[i].color.g as f32
                    + ((self.stops[i + 1].color.g as f32 - self.stops[i].color.g as f32) * ratio);
                let b: f32 = self.stops[i].color.b as f32
                    + ((self.stops[i + 1].color.b as f32 - self.stops[i].color.b as f32) * ratio);
                let lerped_color = Color {
                    r: r.round() as u8,
                    g: g.round() as u8,
                    b: b.round() as u8,
                };
                return lerped_color;
            }
        }
        // Should never happen, since t is clamped
        panic!(
            "lerp: no segment found for t={t:.4}, stops={:?}",
            self.stops
        );
    }
}

#[cfg(test)]
mod tests {
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
    fn current_color_is_stop_0_at_t_0() {
        let theme = Theme::new(DEFAULT_STOPS.to_vec(), IDLE, InterpolationMethod::Lerp);
        let t: f32 = 0.0;
        let stop_0 = theme.current_color(t);

        assert_eq!(stop_0, STOP_0);
    }

    #[test]
    fn current_color_is_stop_1_at_t_0_point_5() {
        let theme = Theme::new(DEFAULT_STOPS.to_vec(), IDLE, InterpolationMethod::Lerp);
        let t: f32 = 0.5;
        let stop_1 = theme.current_color(t);

        assert_eq!(stop_1, STOP_1);
    }

    #[test]
    fn current_color_is_stop_2_at_t_0_point_8_3_3() {
        let theme = Theme::new(DEFAULT_STOPS.to_vec(), IDLE, InterpolationMethod::Lerp);
        let t: f32 = 0.833;
        let stop_2 = theme.current_color(t);

        assert_eq!(stop_2, STOP_2);
    }

    #[test]
    fn current_color_is_correct_at_t_0_point_25() {
        let theme = Theme::new(DEFAULT_STOPS.to_vec(), IDLE, InterpolationMethod::Lerp);
        let t: f32 = 0.25;
        let color = theme.current_color(t);
        let correct_color = Color {
            r: 102,
            g: 204,
            b: 0,
        };

        assert_eq!(color, correct_color);
    }

    #[test]
    fn current_color_is_correct_at_t_0_point_6_6_6() {
        let theme = Theme::new(DEFAULT_STOPS.to_vec(), InterpolationMethod::Lerp);
        let t: f32 = 0.666;
        let color = theme.current_color(t);
        let correct_color = Color {
            r: 204,
            g: 102,
            b: 0,
        };

        assert_eq!(color, correct_color);
    }

    #[test]
    fn current_color_is_correct_at_t_0_point_9_1_7() {
        let theme = Theme::new(DEFAULT_STOPS.to_vec(), InterpolationMethod::Lerp);
        let t: f32 = 0.917;
        let color = theme.current_color(t);
        let correct_color = Color { r: 102, g: 0, b: 0 };

        assert_eq!(color, correct_color);
    }
}
