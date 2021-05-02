//! # Luv
//!
//! Tools for converting colours between sRGB, L\*u\*v\* and LCh(uv) colour
//! spaces and comparing differences in colour.
//!
//! sRGB colors, for this crate at least, are considered to be composed of `u8`
//! values from 0 to 255, while L\*u\*v\* colors are represented by its own
//! struct that uses `f32` values.  The crate is biased towards sRGB thus it
//! also assumes that L\*u\*v\* uses D65 reference white point.
//!
//! # Usage
//!
//! ## Converting single values
//!
//! To convert a single value, use one of the functions
//!
//! * `luv::Luv::from_rgb(rgb: &[u8; 3]) -> Luv`
//! * `luv::Luv::from_rgba(rgba: &[u8; 4]) -> Luv` (drops the fourth alpha byte)
//! * `luv::Luv::to_rgb(&self) -> [u8; 3]`
//!
//! ```rust
//! let pink = luv::Luv::from_rgb(&[253, 120, 138]);
//! assert_eq!(luv::Luv { l: 66.637695, u: 93.02938, v: 9.430316 }, pink);
//! ```
//!
//! ## Converting multiple values
//!
//! To convert slices of values
//!
//! * `luv::rgbs_to_luvs(rgbs: &[[u8; 3]]) -> Vec<Luv>`
//! * `luv::luvs_to_rgbs(luvs: &[Luv]) -> Vec<[u8; 3]>`
//! * `luv::rgb_bytes_to_luvs(bytes: &[u8]) -> Vec<Luv>`
//! * `luv::luvs_to_rgb_bytes(luvs: &[Luv]) -> Vec<u8>`
//!
//! ```rust
//! let rgbs = vec![
//!     [0xFF, 0x69, 0xB6],
//!     [0xE7, 0x00, 0x00],
//!     [0xFF, 0x8C, 0x00],
//!     [0xFF, 0xEF, 0x00],
//!     [0x00, 0x81, 0x1F],
//!     [0x00, 0xC1, 0xC1],
//!     [0x00, 0x44, 0xFF],
//!     [0x76, 0x00, 0x89],
//! ];
//!
//! let luvs = luv::rgbs_to_luvs(&rgbs);
//! ```
//!
//! ```rust
//! use luv::rgb_bytes_to_luvs;
//!
//! let rgbs = vec![
//!     0xFF, 0x69, 0xB6,
//!     0xE7, 0x00, 0x00,
//!     0xFF, 0x8C, 0x00,
//!     0xFF, 0xEF, 0x00,
//!     0x00, 0x81, 0x1F,
//!     0x00, 0xC1, 0xC1,
//!     0x00, 0x44, 0xFF,
//!     0x76, 0x00, 0x89,
//! ];
//!
//! let luvs = rgb_bytes_to_luvs(&rgbs);
//! ```
//!
//! # Features
//!
//! The crate defines an `approx` feature.  If enabled, approximate equality as
//! defined by [`approx` crate](https://crates.io/crates/approx) will be
//! implemented for the `Luv` and `LCh` types.
//!
//! # Other crates
//!
//! The design — and to some degree code — of this crate has been based on the
//! [`lab` crate](https://crates.io/crates/lab) which provides routines for
//! converting colours between sRGB, L\*a\*\b and LCh(ab) colour spaces.
//!
//! For conversion between sRGB and XYZ colour spaces this crate relies on the
//! [`srgb` crate](https://crates.io/crates/srgb).

#[cfg(any(test, feature = "approx"))]
mod approx_impl;

/// Struct representing a color in CIALuv, a.k.a. L\*u\*v\*, color space
#[derive(Debug, Copy, Clone, Default)]
pub struct Luv {
    /// The L\* value (achromatic luminance) of the colour in 0–100 range.
    pub l: f32,
    /// The u\* value of the colour.
    ///
    /// Together with v\* value, it defines chromacity of the colour.  The u\*
    /// coordinate represents colour’s position on red-green axis with negative
    /// values indicating more red and positive more green colour.  Typical
    /// values are in -100–100 range (but exact range for ‘valid’ colours
    /// depends on luminance and v\* value).
    pub u: f32,
    /// The u\* value of the colour.
    ///
    /// Together with u\* value, it defines chromacity of the colour.  The v\*
    /// coordinate represents colour’s position on blue-yellow axis with
    /// negative values indicating more blue and positive more yellow colour.
    /// Typical values are in -100–100 range (but exact range for ‘valid’
    /// colours depends on luminance and u\* value).
    pub v: f32,
}

/// Struct representing a color in cylindrical CIELCh(uv) color space
#[derive(Debug, Copy, Clone, Default)]
pub struct LCh {
    /// The L\* value (achromatic luminance) of the colour in 0–100 range.
    ///
    /// This is the same value as in the [`Luv`] object.
    pub l: f32,
    /// The C\*_uv value (chroma) of the colour.
    ///
    /// Together with h_uv, it defines chromacity of the colour.  The typical
    /// values of the coordinate go from zero up to around 150 (but exact range
    /// for ‘valid’ colours depends on luminance and hue).  Zero represents
    /// shade of grey.
    pub c: f32,
    /// The h_uv value (hue) of the colour measured in radians.
    ///
    /// Together with C\*_uv, it defines chromacity of the colour.  The value
    /// represents an angle thus it wraps around τ.  Typically, the value will
    /// be in the -π–π range.  The value is undefined if C\*_uv is zero.
    pub h: f32,
}


// κ and ε parameters used in conversion between XYZ and L*u*v*.  See
// http://www.brucelindbloom.com/LContinuity.html for explanation as to why
// those are different values than those provided by CIE standard.
const KAPPA: f32 = 24389.0 / 27.0;
const ONE_OVER_KAPPA: f32 = 27.0 / 24389.0;
const EPSILON: f32 = 216.0 / 24389.0;
const KAPPA_EPSILON: f32 = /* κ * ε = 216 / 27 = 8 */ 8.0;

use srgb::xyz::D65_XYZ;
const WHITE_U_PRIME: f32 =
    4.0 * D65_XYZ[0] / (D65_XYZ[0] + 15.0 * D65_XYZ[1] + 3.0 * D65_XYZ[2]);
const WHITE_V_PRIME: f32 =
    9.0 * D65_XYZ[1] / (D65_XYZ[0] + 15.0 * D65_XYZ[1] + 3.0 * D65_XYZ[2]);

fn luv_from_xyz(xyz: [f32; 3]) -> Luv {
    let [x, y, z] = xyz;

    let l = if y <= 0.0 {
        return Luv::default();
    } else if y <= EPSILON {
        KAPPA * y
    } else {
        y.powf(1.0 / 3.0).mul_add(116.0, -16.0)
    };

    let d = y.mul_add(15.0, z.mul_add(3.0, x));
    let ll = 13.0 * l;
    let u = ll * (x / d).mul_add(4.0, -WHITE_U_PRIME);
    let v = ll * (y / d).mul_add(9.0, -WHITE_V_PRIME);

    Luv { l, u, v }
}

fn xyz_from_luv(luv: &Luv) -> [f32; 3] {
    if luv.l <= 0.0 {
        return [0.0, 0.0, 0.0];
    }
    let ll = 13.0 * luv.l;
    let u_prime = luv.u / ll + WHITE_U_PRIME;
    let v_prime = luv.v / ll + WHITE_V_PRIME;

    let y = if luv.l > KAPPA_EPSILON {
        ((luv.l + 16.0) / 116.0).powi(3)
    } else {
        luv.l * ONE_OVER_KAPPA
    };

    let a = 0.75 * y * u_prime / v_prime;
    let x = 3.0 * a;
    let z = y * (3.0 - 5.0 * v_prime) / v_prime - a;

    [x, y, z]
}


/// Convenience function to map a slice of RGB values to Luv values in serial
///
/// # Example
/// ```
/// let rgbs = &[[255u8, 0, 0], [255, 0, 255], [0, 255, 255]];
/// let luvs = luv::rgbs_to_luvs(rgbs);
/// assert_eq!(vec![
///     luv::Luv { l: 53.238235, u: 175.01141, v: 37.758636 },
///     luv::Luv { l: 60.32269, u: 84.06383, v: -108.690346 },
///     luv::Luv { l: 91.11428, u: -70.46933, v: -15.2037325 },
/// ], luvs);
/// ```
#[inline]
pub fn rgbs_to_luvs(rgbs: &[[u8; 3]]) -> Vec<Luv> {
    rgbs.iter().map(Luv::from_rgb).collect()
}

/// RGB to Luv conversion that operates on a flat `&[u8]` of consecutive RGB
/// triples.
///
/// # Example
/// ```
/// let rgbs = &[255u8, 0, 0, 255, 0, 255, 0, 255, 255];
/// let luvs = luv::rgb_bytes_to_luvs(rgbs);
/// assert_eq!(vec![
///     luv::Luv { l: 53.238235, u: 175.01141, v: 37.758636 },
///     luv::Luv { l: 60.32269, u: 84.06383, v: -108.690346 },
///     luv::Luv { l: 91.11428, u: -70.46933, v: -15.2037325 },
/// ], luvs);
/// ```
pub fn rgb_bytes_to_luvs(bytes: &[u8]) -> Vec<Luv> {
    use std::convert::TryInto;
    bytes
        .chunks_exact(3)
        .map(|rgb| Luv::from_rgb(rgb.try_into().unwrap()))
        .collect()
}

/// Convenience function to map a slice of Luv values to RGB values in serial
///
/// # Example
/// ```
/// let luvs = &[
///     luv::Luv { l: 53.238235, u: 175.01141, v: 37.75865 },
///     luv::Luv { l: 60.322693, u: 84.063835, v: -108.69038 },
///     luv::Luv { l: 91.11428, u: -70.46933, v: -15.203715 }
/// ];
/// let rgbs = luv::luvs_to_rgbs(luvs);
/// assert_eq!(vec![[255u8, 0, 0], [255, 0, 255], [0, 255, 255]], rgbs);
/// ```
#[inline]
pub fn luvs_to_rgbs(luvs: &[Luv]) -> Vec<[u8; 3]> {
    luvs.iter().map(Luv::to_rgb).collect()
}

/// Luv to RGB conversion that returns RGB triples flattened into a `Vec<u8>`
///
/// # Example
/// ```
/// let luvs = &[
///     luv::Luv { l: 53.238235, u: 175.01141, v: 37.75865 },
///     luv::Luv { l: 60.322693, u: 84.063835, v: -108.69038 },
///     luv::Luv { l: 91.11428, u: -70.46933, v: -15.203715 }
/// ];
/// let rgb_bytes = luv::luvs_to_rgb_bytes(luvs);
/// assert_eq!(vec![255u8, 0, 0, 255, 0, 255, 0, 255, 255], rgb_bytes);
/// ```
#[inline]
pub fn luvs_to_rgb_bytes(luvs: &[Luv]) -> Vec<u8> {
    luvs.iter().map(Luv::to_rgb).fold(
        Vec::with_capacity(luvs.len() * 3),
        |mut acc, rgb| {
            acc.extend_from_slice(&rgb);
            acc
        },
    )
}


fn subarray<T>(arr: &[T; 4]) -> &[T; 3] {
    std::convert::TryInto::try_into(&arr[..3]).unwrap()
}


impl Luv {
    /// Constructs a new `Luv` from a three-element array of `u8`s
    ///
    /// # Examples
    ///
    /// ```
    /// let luv = luv::Luv::from_rgb(&[240, 33, 95]);
    /// assert_eq!(luv::Luv { l: 52.334686, u: 138.98636, v: 7.8476834 }, luv);
    /// ```
    pub fn from_rgb(rgb: &[u8; 3]) -> Self {
        luv_from_xyz(srgb::xyz_from_u8(*rgb))
    }

    #[doc(hidden)]
    pub fn from_rgb_normalized(rgb: &[f32; 3]) -> Self {
        luv_from_xyz(srgb::xyz_from_normalised(*rgb))
    }

    /// Constructs a new `Luv` from a four-element array of `u8`s
    ///
    /// The `Luv` struct does not store alpha channel information, so the last
    /// `u8` representing alpha is discarded. This convenience method exists
    /// in order to easily measure colors already stored in an RGBA array.
    ///
    /// # Examples
    ///
    /// ```
    /// let luv = luv::Luv::from_rgba(&[240, 33, 95, 255]);
    /// assert_eq!(luv::Luv { l: 52.334686, u: 138.98636, v: 7.8476834 }, luv);
    /// ```
    pub fn from_rgba(rgba: &[u8; 4]) -> Self { Luv::from_rgb(subarray(rgba)) }

    #[doc(hidden)]
    pub fn from_rgba_normalized(rgba: &[f32; 4]) -> Self {
        Luv::from_rgb_normalized(subarray(rgba))
    }

    /// Returns the `Luv`'s color in RGB, in a 3-element array.
    ///
    /// # Examples
    ///
    /// ```
    /// let luv = luv::Luv { l: 52.334686, u: 138.98636, v: 7.8476787 };
    /// assert_eq!([240, 33, 95], luv.to_rgb());
    /// ```
    pub fn to_rgb(&self) -> [u8; 3] { srgb::u8_from_xyz(xyz_from_luv(self)) }

    #[doc(hidden)]
    pub fn to_rgb_normalized(&self) -> [f32; 3] {
        srgb::normalised_from_xyz(xyz_from_luv(self))
    }

    /// Measures the perceptual distance between the colors of one `Luv`
    /// and an `other`.
    ///
    /// # Examples
    ///
    /// ```
    /// let pink = luv::Luv { l: 52.334686, u: 138.98636, v: 7.8476787 };
    /// let websafe_pink = luv::Luv { l: 56.675262, u: 142.3089, v: 10.548637 };
    /// assert_eq!(37.175053, pink.squared_distance(&websafe_pink));
    /// ```
    pub fn squared_distance(&self, other: &Luv) -> f32 {
        (self.l - other.l).powi(2) +
            (self.u - other.u).powi(2) +
            (self.v - other.v).powi(2)
    }
}


impl LCh {
    /// Constructs a new `LCh` from a three-element array of `u8`s
    ///
    /// # Examples
    ///
    /// ```
    /// let rgb = [240, 33, 95];
    /// let lch = luv::LCh::from_rgb(&rgb);
    /// assert_eq!(luv::LCh {l: 52.334686, c: 139.20773, h: 0.056403805}, lch);
    /// assert_eq!(lch, luv::LCh::from_luv(luv::Luv::from_rgb(&rgb)));
    /// ```
    pub fn from_rgb(rgb: &[u8; 3]) -> Self {
        LCh::from_luv(Luv::from_rgb(&rgb))
    }

    /// Constructs a new `LCh` from a four-element array of `u8`s
    ///
    /// The `LCh` struct does not store alpha channel information, so the last
    /// `u8` representing alpha is discarded. This convenience method exists
    /// in order to easily measure colors already stored in an RGBA array.
    ///
    /// # Examples
    ///
    /// ```
    /// let rgba = [240, 33, 95, 255];
    /// let lch = luv::LCh::from_rgba(&rgba);
    /// assert_eq!(luv::LCh {l: 52.334686, c: 139.20773, h: 0.056403805}, lch);
    /// assert_eq!(lch, luv::LCh::from_luv(luv::Luv::from_rgba(&rgba)));
    /// ```
    pub fn from_rgba(rgba: &[u8; 4]) -> Self {
        LCh::from_luv(Luv::from_rgba(&rgba))
    }

    /// Constructs a new `LCh` from a `Luv`
    ///
    /// # Examples
    ///
    /// ```
    /// let luv = luv::Luv { l: 52.33686, u: 75.5516, v: 19.998878 };
    /// let lch = luv::LCh::from_luv(luv);
    /// assert_eq!(luv::LCh { l: 52.33686, c: 78.15369, h: 0.25877 }, lch);
    ///
    /// let luv = luv::Luv { l: 52.33686, u: 0.0, v: 0.0 };
    /// let lch = luv::LCh::from_luv(luv);
    /// assert_eq!(luv::LCh { l: 52.33686, c: 0.0, h: 0.0 }, lch);
    /// ```
    pub fn from_luv(luv: Luv) -> Self {
        LCh {
            l: luv.l,
            c: luv.u.hypot(luv.v),
            h: luv.v.atan2(luv.u),
        }
    }

    /// Returns the `LCh`'s color in RGB, in a 3-element array
    ///
    /// # Examples
    ///
    /// ```
    /// let mut lch = luv::LCh { l: 52.334686, c: 139.20773, h: 0.05640377 };
    /// assert_eq!([240, 33, 95], lch.to_rgb());
    ///
    /// lch.h += std::f32::consts::TAU;
    /// assert_eq!([240, 33, 95], lch.to_rgb());
    /// ```
    pub fn to_rgb(&self) -> [u8; 3] { self.to_luv().to_rgb() }

    /// Returns the `LCh`'s color in `Luv`
    ///
    /// Note that due to imprecision of floating point arithmetic, conversions
    /// between Luv and LCh are not stable.  A chain of Luv→LCh→Luv or
    /// LCh→Luv→LCh operations isn’t guaranteed to give back the source colour.
    ///
    /// # Examples
    ///
    /// ```
    /// let lch = luv::LCh { l: 52.33686, c: 78.15369, h: 0.25877 };
    /// let luv = lch.to_luv();
    /// assert_eq!(luv::Luv { l: 52.33686, u: 75.5516, v: 19.998878 }, luv);
    ///
    /// let lch = luv::LCh { l: 52.33686, c: 0.0, h: 0.25877 };
    /// let luv = lch.to_luv();
    /// assert_eq!(luv::Luv { l: 52.33686, u: 0.0, v: 0.0 }, luv);
    ///
    /// let inp = luv::Luv { l: 29.52658, u: 58.595745, v: -36.281406 };
    /// let lch = luv::LCh { l: 29.52658, c: 68.91881,  h: -0.5544043 };
    /// let out = luv::Luv { l: 29.52658, u: 58.59575,  v: -36.281406 };
    /// assert_eq!(lch, luv::LCh::from_luv(inp));
    /// assert_eq!(out, lch.to_luv());
    /// ```
    pub fn to_luv(&self) -> Luv {
        Luv {
            l: self.l,
            u: self.c * self.h.cos(),
            v: self.c * self.h.sin(),
        }
    }
}


impl std::cmp::PartialEq<Luv> for Luv {
    /// Compares two colours ignoring chromacity if L\* is zero.
    fn eq(&self, other: &Self) -> bool {
        if self.l != other.l {
            false
        } else if self.l == 0.0 {
            true
        } else {
            self.u == other.u && self.v == other.v
        }
    }
}

impl std::cmp::PartialEq<LCh> for LCh {
    /// Compares two colours ignoring chromacity if L\* is zero and hue if C\*
    /// is zero.  Hues which are τ apart are compared equal.
    fn eq(&self, other: &Self) -> bool {
        if self.l != other.l {
            false
        } else if self.l == 0.0 {
            true
        } else if self.c != other.c {
            false
        } else if self.c == 0.0 {
            true
        } else {
            use std::f32::consts::TAU;
            self.h.rem_euclid(TAU) == other.h.rem_euclid(TAU)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::{LCh, Luv};

    struct Cases {
        rgb: [[u8; 3]; 17],
        xyz: [[f32; 3]; 17],
        luv: [Luv; 17],
        lch: [LCh; 17],
    }

    #[rustfmt::skip]
    static CASES: Cases = Cases {
        rgb: [
            [253, 120, 138],
            [127, 0, 0],
            [0, 127, 0],
            [0, 0, 127],
            [0, 127, 127],
            [127, 0, 127],
            [255, 0, 0],
            [0, 255, 0],
            [0, 0, 255],
            [0, 255, 255],
            [255, 0, 255],
            [255, 255, 0],
            [0, 0, 0],
            [64, 64, 64],
            [127, 127, 127],
            [196, 196, 196],
            [255, 255, 255],
        ],
        xyz: [
            [0.5181153, 0.36154357, 0.2829196],
            [0.08752622, 0.045130707, 0.004102787],
            [0.07589042, 0.15178084, 0.025296807],
            [0.038297836, 0.015319134, 0.20170195],
            [0.11418824, 0.16709997, 0.22699866],
            [0.1258241, 0.060449857, 0.20580474],
            [0.4124108, 0.21264932, 0.019331753],
            [0.35758454, 0.71516913, 0.11919485],
            [0.18045378, 0.072181515, 0.9503897],
            [0.5380384, 0.7873506, 1.0695845],
            [0.59286475, 0.28483093, 0.9697217],
            [0.76999545, 0.92781854, 0.13852677],
            [0.0, 0.0, 0.0],
            [0.04872901, 0.051269446, 0.055828184],
            [0.20171452, 0.21223073, 0.23110169],
            [0.52465874, 0.5520114, 0.6010947],
            [0.95044917, 1.0, 1.0889173],
        ],
        luv: [
            Luv { l: 66.637695, u: 93.02938, v: 9.430316 },
            Luv { l: 25.299875, u: 83.16892, v: 17.94366 },
            Luv { l: 45.87715, u: -43.437836, v: 56.162983 },
            Luv { l: 12.809523, u: -3.7292058, v: -51.694935 },
            Luv { l: 47.892532, u: -37.04091, v: -7.9915605 },
            Luv { l: 29.525677, u: 41.14607, v: -53.19983 },
            Luv { l: 53.238235, u: 175.01141, v: 37.758636 },
            Luv { l: 87.73554, u: -83.07059, v: 107.40619 },
            Luv { l: 32.298466, u: -9.40297, v: -130.34576 },
            Luv { l: 91.11428, u: -70.46933, v: -15.2037325 },
            Luv { l: 60.32269, u: 84.06383, v: -108.690346 },
            Luv { l: 97.139, u: 7.7040625, v: 106.79492 },
            Luv { l: 0.0, u: 0.0, v: 0.0 },
            Luv { l: 27.09341, u: 0.0, v: -0.0000052484024 },
            Luv { l: 53.192772, u: 0.0, v: -0.000010304243 },
            Luv { l: 79.15698, u: -0.000015333902, v: -0.000015333902 },
            Luv { l: 100.0, u: 0.0, v: -0.00001937151 }
        ],
        lch: [
            LCh { l: 66.637695, c: 93.506134, h: 0.101024136 },
            LCh { l: 25.299875, c: 85.08257, h: 0.21249253 },
            LCh { l: 45.87715, c: 71.00089, h: 2.2291214 },
            LCh { l: 12.809523, c: 51.82927, h: -1.6428102 },
            LCh { l: 47.892532, c: 37.893192, h: -2.9291 },
            LCh { l: 29.525677, c: 67.25489, h: -0.9124711 },
            LCh { l: 53.238235, c: 179.03828, h: 0.2124925 },
            LCh { l: 87.73554, c: 135.78223, h: 2.2291214 },
            LCh { l: 32.298466, c: 130.68448, h: -1.6428102 },
            LCh { l: 91.11428, c: 72.090775, h: -2.9291 },
            LCh { l: 60.32269, c: 137.40567, h: -0.91247106 },
            LCh { l: 97.139, c: 107.07244, h: 1.4987823 },
            LCh { l: 0.0, c: 0.0, h: 0.0 },
            LCh { l: 27.09341, c: 0.0000052484024, h: -1.5707964 },
            LCh { l: 53.192772, c: 0.000010304243, h: -1.5707964 },
            LCh { l: 79.15698, c: 0.000021685413, h: -2.3561945 },
            LCh { l: 100.0, c: 0.00001937151, h: -1.5707964 }
        ],
    };

    fn run_test<T, U>(want: &[T], f: impl Fn(&U) -> T, input: &[U])
    where
        T: PartialEq + std::fmt::Debug, {
        let actual: Vec<_> = input.iter().map(f).collect();
        assert_eq!(want, &actual[..]);
    }

    fn run_test_approx<T, U>(want: &[T], f: impl Fn(&U) -> T, input: &[U])
    where
        T: PartialEq + std::fmt::Debug + approx::RelativeEq<Epsilon = f32>,
    {
        let actual: Vec<_> = input.iter().map(f).collect();
        approx::assert_abs_diff_eq!(want, &actual[..], epsilon = 0.0001);
    }

    #[test]
    fn test_luv_from_xyz() {
        run_test_approx(
            &CASES.luv[..],
            |xyz: &[f32; 3]| super::luv_from_xyz(*xyz),
            &CASES.xyz[..],
        );
    }

    #[test]
    fn test_xyz_from_luv() {
        run_test(
            &CASES.xyz[..],
            |luv: &Luv| super::xyz_from_luv(luv),
            &CASES.luv[..],
        );
    }

    #[test]
    fn test_luv_from_rgb() {
        run_test(&CASES.luv[..], Luv::from_rgb, &CASES.rgb[..]);
    }

    #[test]
    fn test_rgb_from_luv() {
        run_test(&CASES.rgb[..], Luv::to_rgb, &CASES.luv[..]);
    }

    #[test]
    fn test_lch_from_luv() {
        run_test(
            &CASES.lch[..],
            |luv: &Luv| LCh::from_luv(*luv),
            &CASES.luv[..],
        );
    }

    #[test]
    fn test_luv_from_lch() {
        run_test_approx(&CASES.luv[..], LCh::to_luv, &CASES.lch[..]);
    }

    fn get_rgb_bytes() -> Vec<u8> {
        CASES.rgb.iter().fold(
            Vec::with_capacity(CASES.rgb.len() * 3),
            |mut acc, rgb| {
                acc.extend_from_slice(&rgb[..]);
                acc
            },
        )
    }

    #[test]
    fn test_rgbs_to_luvs() {
        let got = super::rgbs_to_luvs(&CASES.rgb);
        assert_eq!(&CASES.luv[..], &got[..]);
    }

    #[test]
    fn test_rgb_bytes_to_luvs() {
        let input = get_rgb_bytes();
        let got = super::rgb_bytes_to_luvs(&input[..]);
        assert_eq!(&CASES.luv[..], &got[..]);
    }

    #[test]
    fn test_luvs_to_rgbs() {
        let got = super::luvs_to_rgbs(&CASES.luv);
        assert_eq!(&CASES.rgb[..], &got[..]);
    }

    #[test]
    fn test_luvs_to_rgb_bytes() {
        let want = get_rgb_bytes();
        let got = super::luvs_to_rgb_bytes(&CASES.luv);
        assert_eq!(&want[..], &got[..]);
    }

    #[test]
    fn test_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Luv>();
        assert_send::<LCh>();
    }

    #[test]
    fn test_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Luv>();
        assert_sync::<LCh>();
    }

    #[test]
    fn test_rgb_to_luv_to_rgb() {
        use rand::Rng;
        let rgbs: Vec<[u8; 3]> = {
            let rng: rand::rngs::StdRng =
                rand::SeedableRng::from_seed([1u8; 32]);
            rng.sample_iter(&rand::distributions::Standard)
                .take(2048)
                .collect()
        };
        assert_eq!(rgbs, super::luvs_to_rgbs(&super::rgbs_to_luvs(&rgbs)));
    }

    #[test]
    fn test_grey_error() {
        // Grey colours have u* and v* components equal to zero.  This test goes
        // through all 8-bit greys and calculates squared error.  If it goes up,
        // a change might have worsen the precision of the calculations.  If it
        // goes down, calculations got better.
        let mut error: f64 = 0.0;
        let mut count: usize = 0;
        for i in 0..=255_u32 {
            let luv = Luv::from_rgb(&[i as u8, i as u8, i as u8]);
            if luv.u != 0.0 || luv.v != 0.0 {
                let u = (luv.u as f64).mul_add(luv.u as f64, error);
                error = (luv.v as f64).mul_add(luv.v as f64, u);
                count += 1;
            }
        }
        assert_eq!((255, 70.59474231878698), (count, error * 1e9));
    }

    fn square_error(a: Luv, b: Luv, error: f64) -> f64 {
        let a = (a.l as f64, a.u as f64, a.v as f64);
        let b = (b.l as f64, b.u as f64, b.v as f64);
        let (l, u, v) = (a.0 - b.0, a.1 - b.1, a.2 - b.2);
        l.mul_add(l, u.mul_add(u, v.mul_add(v, error)))
    }

    #[test]
    fn test_roundtrip_error() {
        let mut error: f64 = 0.0;
        for l in 1..=22 {
            for u in -22..=-22 {
                for v in -22..=-22 {
                    let src = Luv {
                        l: l as f32 / 0.22,
                        u: u as f32 / 0.11,
                        v: v as f32 / 0.11,
                    };
                    let dst = super::luv_from_xyz(super::xyz_from_luv(&src));
                    error = square_error(src, dst, error);
                }
            }
        }
        assert_eq!(42.49181984050665, error * 1e9);
    }

    #[test]
    #[rustfmt::skip]
    fn test_partial_eq() {
        use std::f32::consts::TAU;

        // Chromacity doesn’t matter if L* is zero.
        assert_eq!(Luv { l: 0.0, u: 0.0, v: 0.0 },
                   Luv { l: 0.0, u: 1.0, v: 0.0 });
        assert_eq!(Luv { l: 0.0, u: 0.0, v: 0.0 },
                   Luv { l: 0.0, u: 0.0, v: 1.0 });
        assert_eq!(LCh { l: 0.0, c: 0.0, h: 0.0 },
                   LCh { l: 0.0, c: 1.0, h: 0.0 });
        assert_eq!(LCh { l: 0.0, c: 0.0, h: 0.0 },
                   LCh { l: 0.0, c: 0.0, h: 1.0 });

        // Hue doesn’t matter if C* is zero.
        assert_eq!(LCh { l: 100.0, c: 0.0, h: 0.0 },
                   LCh { l: 100.0, c: 0.0, h: 1.0 });

        // Hues which are τ apart are eqaul.
        assert_eq!(LCh { l: 75.0, c: 50.0, h: 1.0 },
                   LCh { l: 75.0, c: 50.0, h: 1.0 + 2.0 * TAU });
        assert_eq!(LCh { l: 75.0, c: 50.0, h: 1.0 },
                   LCh { l: 75.0, c: 50.0, h: 1.0 - TAU });

        // And a few non-equal test cases.
        assert_ne!(Luv { l: 25.0, u: 100.0, v: 75.0 },
                   Luv { l: 50.0, u: 100.0, v: 75.0 });
        assert_ne!(Luv { l: 50.0, u: 100.0, v: 75.0 },
                   Luv { l: 50.0, u:  50.0, v: 75.0 });
        assert_ne!(Luv { l: 50.0, u: 100.0, v: 75.0 },
                   Luv { l: 50.0, u: 100.0, v: 25.0 });
        assert_ne!(LCh { l: 50.0, c: 100.0, h: 1.0 },
                   LCh { l: 25.0, c: 100.0, h: 1.0 });
        assert_ne!(LCh { l: 50.0, c: 100.0, h: 1.0 },
                   LCh { l: 50.0, c:  50.0, h: 1.0 });
        assert_ne!(LCh { l: 50.0, c: 100.0, h: 1.0 },
                   LCh { l: 50.0, c: 100.0, h: 2.0 });
    }
}
