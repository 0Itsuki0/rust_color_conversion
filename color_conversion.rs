use anyhow::bail;

fn main() -> anyhow::Result<()> {
    // rgba to hex
    let rgba = (134, 131, 213, Some(0.94));
    let hex_string = rgba_to_hex(rgba, Some(false))?;
    println!("rgba to hex: {}", hex_string);

    // rgba to hsla
    let hsla = rgba_to_hsla(rgba)?;
    println!("rgba to hsla: {:?}", hsla);

    // rgba to hsva
    let hsva = rgba_to_hsva(rgba)?;
    println!("rgba to hsva: {:?}", hsva);

    // rgba to cmyka
    let cmyka = rgba_to_cmyka(rgba)?;
    println!("rgba to cmyka: {:?}", cmyka);

    // hex to rgba
    let rgba = hex_to_rgba("8683D5f0", Some(false))?;
    println!("hex to rgba: {:?}", rgba);

    // hsla to rgba
    let hsla = (242.0, 49.0, 67.0, Some(0.94));
    let rgba = hsla_to_rgba(hsla)?;
    println!("hsla to rgba: {:?}", rgba);

    // hsva to rgba
    let hsva = (242.0, 39.0, 83.0, Some(0.94));
    let rgba = hsva_to_rgba(hsva)?;
    println!("hsva to rgba: {:?}", rgba);

    // cmyka to rgba
    let cmyka = (47.0, 0.0, 16.0, 0.0, Some(0.94));
    let rgba = cmyka_to_rgba(cmyka)?;
    println!("cmyka to rgba: {:?}", rgba);

    // complementary color
    let rgba = (87, 60, 250, Some(1.0));
    let complementary = complementary(rgba)?;
    println!("complementary: {:?}", complementary);
    Ok(())
}

/// Complementary Color
///
/// RGBA to RGBA
///
/// Ranges:
/// * R: 0 - 255
/// * G: 0 - 255
/// * B: 0 - 255
/// * A: 0.0 - 1.0
pub fn complementary(rgba: (u32, u32, u32, Option<f64>)) -> anyhow::Result<(u32, u32, u32, f64)> {
    let (mut h, s, v, a) = rgba_to_hsva(rgba)?;
    fn shift_hue(h: f64, s: f64) -> f64 {
        let mut h = h + s;
        if h >= 360.0 {
            h = h - 360.0
        }
        if h < 0.0 {
            h = h + 360.0
        }
        return h;
    }
    h = shift_hue(h, 180.0);
    return hsva_to_rgba((h, s, v, Some(a)));
}

/// hex string to RGBA
///
/// Ranges:
/// * R: 0 - 255
/// * G: 0 - 255
/// * B: 0 - 255
/// * A: 0.0 - 1.0
pub fn hex_to_rgba(
    hex_str: &str,
    alpha_first: Option<bool>,
) -> anyhow::Result<(u32, u32, u32, f64)> {
    let mut s = hex_str;
    if s.starts_with("#") {
        s = s.trim_start_matches("#");
    }
    if s.len() != 6 && s.len() != 8 {
        bail!("invalid hex.")
    }

    // hex without alpha
    if s.len() == 6 {
        let num = u32::from_str_radix(s, 16)?;
        let r = (num & 0xFF0000) >> 16;
        let g = (num & 0x00FF00) >> 8;
        let b = (num & 0x0000FF) >> 0;
        return Ok((r, g, b, 1.0));
    }

    // alpha first
    if alpha_first.unwrap_or(false) {
        let num = u32::from_str_radix(s, 16)?;
        let a = (num & 0xFF000000) >> 24;
        let r = (num & 0x00FF0000) >> 16;
        let g = (num & 0x0000FF00) >> 8;
        let b = (num & 0x000000FF) >> 0;
        return Ok((r, g, b, (a as f64) / 255.0));
    }

    // alpha last
    let num = u32::from_str_radix(s, 16)?;
    let r = (num & 0xFF000000) >> 24;
    let g = (num & 0x00FF0000) >> 16;
    let b = (num & 0x0000FF00) >> 8;
    let a = (num & 0x000000FF) >> 0;
    return Ok((r, g, b, (a as f64) / 255.0));
}

/// RGBA value to hex string
///
/// Ranges:
/// * R: 0 - 255
/// * G: 0 - 255
/// * B: 0 - 255
/// * A: 0.0 - 1.0
pub fn rgba_to_hex(
    color: (u32, u32, u32, Option<f64>),
    alpha_first: Option<bool>,
) -> anyhow::Result<String> {
    let (r, g, b, a) = color;
    if !check_rgb(&r) || !check_rgb(&g) || !check_rgb(&b) {
        bail!("invalid rgb value.")
    }

    fn num_to_hex(n: u32) -> String {
        let s = format!("{:x}", n);
        if s.len() == 1 {
            String::from("0") + &s
        } else {
            s
        }
    }

    if a.is_none() {
        return Ok(format!(
            "#{}{}{}",
            num_to_hex(r),
            num_to_hex(g),
            num_to_hex(b)
        ));
    }
    let a = a.unwrap();
    if !check_alpha(&a) {
        bail!("invalid alpha value")
    }

    let a_u32 = (a * 255.0).round() as u32;

    // alpha first
    if alpha_first.unwrap_or(false) {
        return Ok(format!(
            "#{}{}{}{}",
            num_to_hex(a_u32),
            num_to_hex(r),
            num_to_hex(g),
            num_to_hex(b)
        ));
    }

    // alpha last
    return Ok(format!(
        "#{}{}{}{}",
        num_to_hex(r),
        num_to_hex(g),
        num_to_hex(b),
        num_to_hex(a_u32)
    ));
}

/// HSLA to RGBA
///
/// Ranges:
/// * H: 0.0 - 360.0 (exclusive)
/// * S: 0.0 - 100.0
/// * L: 0.0 - 100.0
///
/// * R: 0 - 255
/// * G: 0 - 255
/// * B: 0 - 255
///
/// * A: 0.0 - 1.0
pub fn hsla_to_rgba(hsla: (f64, f64, f64, Option<f64>)) -> anyhow::Result<(u32, u32, u32, f64)> {
    if !check_hue(&hsla.0) || !check_percentage(&hsla.1) || !check_percentage(&hsla.2) {
        bail!("invalid hsl value.")
    }
    let (h, s, l, a) = (
        hsla.0 / 360.0,
        hsla.1 / 100.0,
        hsla.2 / 100.0,
        hsla.3.unwrap_or(1.0),
    );

    if !check_alpha(&a) {
        bail!("invalid alpha value")
    }
    if s == 0.0 {
        return Ok((255, 255, 255, a));
    }

    let q = if l < 0.5 {
        l * (1.0 + s)
    } else {
        l + s - l * s
    };

    let p = 2.0 * l - q;

    fn calc(p: f64, q: f64, t: f64) -> f64 {
        let mut t = t;
        if t < 0.0 {
            t += 1.0
        };
        if t > 1.0 {
            t -= 1.0
        };
        if t < 1.0 / 6.0 {
            return p + (q - p) * 6.0 * t;
        };
        if t < 1.0 / 2.0 {
            return q;
        };
        if t < 2.0 / 3.0 {
            return p + (q - p) * (2.0 / 3.0 - t) * 6.0;
        };
        return p;
    }
    let r = calc(p, q, h + 1.0 / 3.0);
    let g = calc(p, q, h);
    let b = calc(p, q, h - 1.0 / 3.0);

    return Ok((
        (r * 255.0) as u32,
        (g * 255.0) as u32,
        (b * 255.0) as u32,
        a,
    ));
}

/// RGBA to HSLA
///
/// Ranges:
/// * H: 0.0 - 360.0 (exclusive)
/// * S: 0.0 - 100.0
/// * L: 0.0 - 100.0
///
/// * R: 0 - 255
/// * G: 0 - 255
/// * B: 0 - 255
///
/// * A: 0.0 - 1.0
pub fn rgba_to_hsla(rgba: (u32, u32, u32, Option<f64>)) -> anyhow::Result<(f64, f64, f64, f64)> {
    if !check_rgb(&rgba.0) || !check_rgb(&rgba.1) || !check_rgb(&rgba.2) {
        bail!("invalid rgb value.")
    }
    let (r, g, b, a) = (
        (rgba.0 as f64) / 255.0,
        (rgba.1 as f64) / 255.0,
        (rgba.2 as f64) / 255.0,
        rgba.3.unwrap_or(1.0),
    );

    if !check_alpha(&a) {
        bail!("invalid alpha value")
    }

    let (max, max_index) = max(vec![r, g, b]).unwrap_or((1.0, 0));
    let (min, _) = min(vec![r, g, b]).unwrap_or((0.0, 0));

    let lum = (max + min) / 2.0;
    if max == min {
        return Ok((0.0, 0.0, lum, a));
    }

    let chroma = max - min;
    let sat = chroma / (1.0 - (2.0 * lum - 1.0).abs());

    let hue = match max_index {
        // r
        0 => {
            let x = if g < b { 6.0 } else { 0.0 };
            (g - b) / chroma + x
        }
        // g
        1 => (b - r) / chroma + 2.0,
        // b
        2 => (r - g) / chroma + 4.0,
        _ => unreachable!(),
    };

    let mut hue = hue * 60.0;
    if hue < 0.0 {
        hue = hue + 360.0
    }

    return Ok((hue, sat * 100.0, lum * 100.0, a));
}

/// HSVA to RGBA
///
/// Ranges:
/// * H: 0.0 - 360.0 (exclusive)
/// * S: 0.0 - 100.0
/// * V: 0.0 - 100.0
///
/// * R: 0 - 255
/// * G: 0 - 255
/// * B: 0 - 255
///
/// * A: 0.0 - 1.0
pub fn hsva_to_rgba(hsva: (f64, f64, f64, Option<f64>)) -> anyhow::Result<(u32, u32, u32, f64)> {
    if !check_hue(&hsva.0) || !check_percentage(&hsva.1) || !check_percentage(&hsva.2) {
        bail!("invalid hsv value.")
    }
    let (h, s, v, a) = (
        hsva.0 / 360.0,
        hsva.1 / 100.0,
        hsva.2 / 100.0,
        hsva.3.unwrap_or(1.0),
    );

    if !check_alpha(&a) {
        bail!("invalid alpha value")
    }

    let i = (h * 6.0).floor();
    let f = h * 6.0 - i;
    let p = v * (1.0 - s);
    let q = v * (1.0 - f * s);
    let t = v * (1.0 - (1.0 - f) * s);

    if s == 0.0 {
        return Ok((255, 255, 255, a));
    }

    let r: f64;
    let g: f64;
    let b: f64;

    match (i as u32) % 6 {
        0 => {
            r = v;
            g = t;
            b = p;
        }
        1 => {
            r = q;
            g = v;
            b = p;
        }
        2 => {
            r = p;
            g = v;
            b = t;
        }
        3 => {
            r = p;
            g = q;
            b = v;
        }
        4 => {
            r = t;
            g = p;
            b = v;
        }
        5 => {
            r = v;
            g = p;
            b = q;
        }
        _ => unreachable!(),
    }

    return Ok((
        (r * 255.0) as u32,
        (g * 255.0) as u32,
        (b * 255.0) as u32,
        a,
    ));
}

/// RGBA to HSVA
///
/// Ranges:
/// * H: 0.0 - 360.0 (exclusive)
/// * S: 0.0 - 100.0
/// * V: 0.0 - 100.0
///
/// * R: 0 - 255
/// * G: 0 - 255
/// * B: 0 - 255
///
/// * A: 0.0 - 1.0
pub fn rgba_to_hsva(rgba: (u32, u32, u32, Option<f64>)) -> anyhow::Result<(f64, f64, f64, f64)> {
    if !check_rgb(&rgba.0) || !check_rgb(&rgba.1) || !check_rgb(&rgba.2) {
        bail!("invalid rgb value.")
    }
    let (r, g, b, a) = (
        (rgba.0 as f64) / 255.0,
        (rgba.1 as f64) / 255.0,
        (rgba.2 as f64) / 255.0,
        rgba.3.unwrap_or(1.0),
    );

    if !check_alpha(&a) {
        bail!("invalid alpha value")
    }

    let (max, max_index) = max(vec![r, g, b]).unwrap_or((1.0, 0));
    let (min, _) = min(vec![r, g, b]).unwrap_or((0.0, 0));
    let val = max;

    if max == min {
        return Ok((0.0, 0.0, val, a));
    }

    let diff = max - min;
    let hue: f64;

    let sat = diff / val;

    match max_index {
        0 => hue = (g - b) / diff,
        1 => hue = 2.0 + (b - r) / diff,
        2 => hue = 4.0 + (r - g) / diff,
        _ => unreachable!(),
    }
    let mut hue = hue * 60.0;
    if hue < 0.0 {
        hue = hue + 360.0
    }

    return Ok((hue, sat * 100.0, val * 100.0, a));
}

/// CMYKA to RGBA
///
/// Ranges:
/// * C: 0.0 - 100.0
/// * M: 0.0 - 100.0
/// * Y: 0.0 - 100.0
/// * K: 0.0 - 100.0
///
/// * R: 0 - 255
/// * G: 0 - 255
/// * B: 0 - 255
///
/// * A: 0.0 - 1.0
pub fn cmyka_to_rgba(
    cmyka: (f64, f64, f64, f64, Option<f64>),
) -> anyhow::Result<(u32, u32, u32, f64)> {
    if !check_percentage(&cmyka.0)
        || !check_percentage(&cmyka.1)
        || !check_percentage(&cmyka.2)
        || !check_percentage(&cmyka.3)
    {
        bail!("invalid cmyk value.")
    }
    let (c, m, y, k, a) = (
        cmyka.0 / 100.0,
        cmyka.1 / 100.0,
        cmyka.2 / 100.0,
        cmyka.3 / 100.0,
        cmyka.4.unwrap_or(1.0),
    );

    if !check_alpha(&a) {
        bail!("invalid alpha value")
    }

    let r = (1.0 - c) * (1.0 - k);
    let g = (1.0 - m) * (1.0 - k);
    let b = (1.0 - y) * (1.0 - k);

    return Ok((
        (r * 255.0) as u32,
        (g * 255.0) as u32,
        (b * 255.0) as u32,
        a,
    ));
}

/// RGBA to CMYKA
///
/// Ranges:
/// * C: 0.0 - 100.0
/// * M: 0.0 - 100.0
/// * Y: 0.0 - 100.0
/// * K: 0.0 - 100.0
///
/// * R: 0 - 255
/// * G: 0 - 255
/// * B: 0 - 255
///
/// * A: 0.0 - 1.0
pub fn rgba_to_cmyka(
    rgba: (u32, u32, u32, Option<f64>),
) -> anyhow::Result<(f64, f64, f64, f64, f64)> {
    if !check_rgb(&rgba.0) || !check_rgb(&rgba.1) || !check_rgb(&rgba.2) {
        bail!("invalid rgb value.")
    }
    let (r, g, b, a) = (
        (rgba.0 as f64) / 255.0,
        (rgba.1 as f64) / 255.0,
        (rgba.2 as f64) / 255.0,
        rgba.3.unwrap_or(1.0),
    );

    if !check_alpha(&a) {
        bail!("invalid alpha value")
    }

    let (max, _) = max(vec![r, g, b]).unwrap_or((1.0, 0));
    let k = 1.0 - max;

    let c = (1.0 - r - k) / (1.0 - k);
    let m = (1.0 - g - k) / (1.0 - k);
    let y = (1.0 - b - k) / (1.0 - k);

    return Ok((c * 100.0, m * 100.0, y * 100.0, k * 100.0, a));
}

/********************************/
/****** Helper functions ********/
/********************************/
/// check rgb value
fn check_rgb(num: &u32) -> bool {
    return (0..=255).contains(num);
}

/// check alpha value
fn check_alpha(a: &f64) -> bool {
    return (0.0..=1.0).contains(a);
}

/// check hue value
fn check_hue(h: &f64) -> bool {
    return (0.0..=360.0).contains(h);
}

/// check percentage
fn check_percentage(n: &f64) -> bool {
    return (0.0..=100.0).contains(n);
}

/// min with index of a vector
fn min<T: std::cmp::PartialOrd + Copy>(v: Vec<T>) -> Option<(T, usize)> {
    if v.is_empty() {
        return None;
    }
    let mut result = v[0];
    let mut index: usize = 0;
    if v.len() == 1 {
        return Some((result, index));
    }
    for i in 1..v.len() {
        let v = v[i];
        if v < result {
            result = v;
            index = i;
        }
    }
    Some((result, index))
}

/// min with index of a vector
fn max<T: std::cmp::PartialOrd + Copy>(v: Vec<T>) -> Option<(T, usize)> {
    if v.is_empty() {
        return None;
    }
    let mut result = v[0];
    let mut index: usize = 0;
    if v.len() == 1 {
        return Some((result, index));
    }
    for i in 1..v.len() {
        let v = v[i];
        if v > result {
            result = v;
            index = i;
        }
    }
    Some((result, index))
}
