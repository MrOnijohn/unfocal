use serde::Deserialize;

pub fn extract_normal_colors(theme: &Theme) -> NormalColorsRgb {
    NormalColorsRgb {
        black: hex_to_rgb(&theme.colors.normal.black),
        red: hex_to_rgb(&theme.colors.normal.red),
        green: hex_to_rgb(&theme.colors.normal.green),
        yellow: hex_to_rgb(&theme.colors.normal.yellow),
        blue: hex_to_rgb(&theme.colors.normal.blue),
        magenta: hex_to_rgb(&theme.colors.normal.magenta),
        cyan: hex_to_rgb(&theme.colors.normal.cyan),
        white: hex_to_rgb(&theme.colors.normal.white),
    }
}

pub struct NormalColorsRgb {
    pub black: (u8, u8, u8),
    pub red: (u8, u8, u8),
    pub green: (u8, u8, u8),
    pub yellow: (u8, u8, u8),
    pub blue: (u8, u8, u8),
    pub magenta: (u8, u8, u8),
    pub cyan: (u8, u8, u8),
    pub white: (u8, u8, u8),
}

/// Structs for deserializing `[colors.normal]`
#[derive(Deserialize)]
pub struct NormalColors {
    pub black: String,
    pub red: String,
    pub green: String,
    pub yellow: String,
    pub blue: String,
    pub magenta: String,
    pub cyan: String,
    pub white: String,
}

#[derive(Deserialize)]
pub struct Colors {
    pub normal: NormalColors,
}

#[derive(Deserialize)]
pub struct Theme {
    pub colors: Colors,
}

/// Parse alacritty theme file and return Colors
pub fn load_theme(path: &std::path::Path) -> std::io::Result<Theme> {
    let data = std::fs::read_to_string(path)?;
    let theme: Theme = toml::from_str(&data).unwrap();
    Ok(theme)
}

/// Load theme from file or return default colors
pub fn load_theme_or_default(path: &std::path::Path) -> Theme {
    if let Ok(data) = std::fs::read_to_string(path) {
        if let Ok(theme) = toml::from_str::<Theme>(&data) {
            return theme;
        }
    }
    // Fallback to default colors
    Theme {
        colors: Colors {
            normal: NormalColors {
                black: "#000000".into(),
                red: "#FF0000".into(),
                green: "#00FF00".into(),
                yellow: "#FFFF00".into(),
                blue: "#0000FF".into(),
                magenta: "#FF00FF".into(),
                cyan: "#00FFFF".into(),
                white: "#FFFFFF".into(),
            },
        },
    }
}

pub fn hex_to_rgb(s: &str) -> (u8, u8, u8) {
    let s = s.trim();
    let hex = if s.starts_with('#') {
        &s[1..]
    } else if s.starts_with("0x") {
        &s[2..]
    } else {
        s
    };

    let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
    (r, g, b)
}

pub fn gradient_color(ratio: f32, stops: &[(f32, (u8, u8, u8))]) -> (u8, u8, u8) {
    let ratio = ratio.clamp(0.0, 1.0);

    for w in stops.windows(2) {
        let (r0, c0) = w[0];
        let (r1, c1) = w[1];
        if ratio >= r0 && ratio <= r1 {
            let local = (ratio - r0) / (r1 - r0);
            return lerp_rgb(c0, c1, local);
        }
    }

    stops.last().unwrap().1
}

pub fn lerp_rgb(start: (u8, u8, u8), end: (u8, u8, u8), ratio: f32) -> (u8, u8, u8) {
    let r = start.0 as f32 + (end.0 as f32 - start.0 as f32) * ratio;
    let g = start.1 as f32 + (end.1 as f32 - start.1 as f32) * ratio;
    let b = start.2 as f32 + (end.2 as f32 - start.2 as f32) * ratio;
    (r as u8, g as u8, b as u8)
}
