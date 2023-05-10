use lab::Lab;

const JND: f32 = 2.3 * 2.0;
pub fn are_colors_similar(color1: &Lab, color2: &Lab) -> bool {
  ciede2000_distance(*color1, *color2) < JND
}

// Below is a vendored copy of https://github.com/elliotekj/DeltaE.
// This isdue to the original repo using an older version image library.
//
// The MIT License (MIT)
// Copyright (c) 2017 Elliot Jackson
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

fn ciede2000_distance(color_1: Lab, color_2: Lab) -> f32 {
  let ksub_l = 1.0;
  let ksub_c = 1.0;
  let ksub_h = 1.0;

  let delta_l_prime = color_2.l - color_1.l;

  let l_bar = (color_1.l + color_2.l) / 2.0;

  let c1 = (color_1.a.powi(2) + color_1.b.powi(2)).sqrt();
  let c2 = (color_2.a.powi(2) + color_2.b.powi(2)).sqrt();

  let c_bar = (c1 + c2) / 2.0;

  let a_prime_1 = color_1.a
    + (color_1.a / 2.0) * (1.0 - (c_bar.powi(7) / (c_bar.powi(7) + 25f32.powi(7))).sqrt());
  let a_prime_2 = color_2.a
    + (color_2.a / 2.0) * (1.0 - (c_bar.powi(7) / (c_bar.powi(7) + 25f32.powi(7))).sqrt());

  let c_prime_1 = (a_prime_1.powi(2) + color_1.b.powi(2)).sqrt();
  let c_prime_2 = (a_prime_2.powi(2) + color_2.b.powi(2)).sqrt();

  let c_bar_prime = (c_prime_1 + c_prime_2) / 2.0;

  let delta_c_prime = c_prime_2 - c_prime_1;

  let s_sub_l = 1.0 + ((0.015 * (l_bar - 50.0).powi(2)) / (20.0 + (l_bar - 50.0).powi(2)).sqrt());

  let s_sub_c = 1.0 + 0.045 * c_bar_prime;

  let h_prime_1 = get_h_prime_fn(color_1.b, a_prime_1);
  let h_prime_2 = get_h_prime_fn(color_2.b, a_prime_2);

  let delta_h_prime = get_delta_h_prime(c1, c2, h_prime_1, h_prime_2);

  let delta_upcase_h_prime =
    2.0 * (c_prime_1 * c_prime_2).sqrt() * (degrees_to_radians(delta_h_prime) / 2.0).sin();

  let upcase_h_bar_prime = get_upcase_h_bar_prime(h_prime_1, h_prime_2);

  let upcase_t = get_upcase_t(upcase_h_bar_prime);

  let s_sub_upcase_h = 1.0 + 0.015 * c_bar_prime * upcase_t;

  let r_sub_t = get_r_sub_t(c_bar_prime, upcase_h_bar_prime);

  let lightness: f32 = delta_l_prime / (ksub_l * s_sub_l);

  let chroma: f32 = delta_c_prime / (ksub_c * s_sub_c);

  let hue: f32 = delta_upcase_h_prime / (ksub_h * s_sub_upcase_h);

  (lightness.powi(2) + chroma.powi(2) + hue.powi(2) + r_sub_t * chroma * hue).sqrt()
}
/// Returns the difference between two RGB colors.
///
/// ### Example
///
/// ```
/// extern crate delta_e;
///
/// use delta_e::DE2000;
///
/// fn main() {
///     let color_1 = [234, 76, 76];
///     let color_2 = [76, 187, 234];
///
///     let delta_e = DE2000::from_rgb(&color_1, &color_2);
///     println!("The color difference is: {}", delta_e);
/// }
/// ```

//     pub fn from_rgb(color_1: &[u8; 3], color_2: &[u8; 3]) -> f32 {
//         let lab_1 = Lab::from_rgb(color_1);
//         let lab_2 = Lab::from_rgb(color_2);
//
//         DE2000::new(lab_1, lab_2)
//     }
// }

fn get_h_prime_fn(x: f32, y: f32) -> f32 {
  let mut hue_angle;

  if x == 0.0 && y == 0.0 {
    return 0.0;
  }

  hue_angle = radians_to_degrees(x.atan2(y));

  if hue_angle < 0.0 {
    hue_angle += 360.0;
  }

  hue_angle
}

fn get_delta_h_prime(c1: f32, c2: f32, h_prime_1: f32, h_prime_2: f32) -> f32 {
  if 0.0 == c1 || 0.0 == c2 {
    return 0.0;
  }

  if (h_prime_1 - h_prime_2).abs() <= 180.0 {
    return h_prime_2 - h_prime_1;
  }

  if h_prime_2 <= h_prime_1 {
    h_prime_2 - h_prime_1 + 360.0
  } else {
    h_prime_2 - h_prime_1 - 360.0
  }
}

fn get_upcase_h_bar_prime(h_prime_1: f32, h_prime_2: f32) -> f32 {
  if (h_prime_1 - h_prime_2).abs() > 180.0 {
    return (h_prime_1 + h_prime_2 + 360.0) / 2.0;
  }

  (h_prime_1 + h_prime_2) / 2.0
}

fn get_upcase_t(upcase_h_bar_prime: f32) -> f32 {
  1.0 - 0.17 * (degrees_to_radians(upcase_h_bar_prime - 30.0)).cos()
    + 0.24 * (degrees_to_radians(2.0 * upcase_h_bar_prime)).cos()
    + 0.32 * (degrees_to_radians(3.0 * upcase_h_bar_prime + 6.0)).cos()
    - 0.20 * (degrees_to_radians(4.0 * upcase_h_bar_prime - 63.0)).cos()
}

fn get_r_sub_t(c_bar_prime: f32, upcase_h_bar_prime: f32) -> f32 {
  -2.0
    * (c_bar_prime.powi(7) / (c_bar_prime.powi(7) + 25f32.powi(7))).sqrt()
    * (degrees_to_radians(60.0 * (-(((upcase_h_bar_prime - 275.0) / 25.0).powi(2))).exp())).sin()
}

fn radians_to_degrees(radians: f32) -> f32 {
  radians * (180.0 / std::f32::consts::PI)
}

fn degrees_to_radians(degrees: f32) -> f32 {
  degrees * (std::f32::consts::PI / 180.0)
}
