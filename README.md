# Luv

Tools for converting colours between sRGB, L\*u\*v\* and LCh(uv) colour
spaces and comparing differences in colour.

sRGB colors, for this crate at least, are considered to be composed of `u8`
values from 0 to 255, while L\*u\*v\* colors are represented by its own
struct that uses `f32` values.  The crate is biased towards sRGB thus it
also assumes that L\*u\*v\* uses D65 reference white point.

# Usage

## Converting single values

To convert a single value, use one of the functions

* `luv::Luv::from_rgb(rgb: &[u8; 3]) -> Luv`
* `luv::Luv::from_rgba(rgba: &[u8; 4]) -> Luv` (drops the fourth alpha byte)
* `luv::Luv::to_rgb(&self) -> [u8; 3]`

```rust
let pink = luv::Luv::from_rgb(&[253, 120, 138]);
assert_eq!(luv::Luv { l: 66.637695, u: 93.02938, v: 9.430316 }, pink);
```

## Converting multiple values

To convert slices of values

* `luv::rgbs_to_luvs(rgbs: &[[u8; 3]]) -> Vec<Luv>`
* `luv::luvs_to_rgbs(luvs: &[Luv]) -> Vec<[u8; 3]>`
* `luv::rgb_bytes_to_luvs(bytes: &[u8]) -> Vec<Luv>`
* `luv::luvs_to_rgb_bytes(luvs: &[Luv]) -> Vec<u8>`

```rust
let rgbs = vec![
    [0xFF, 0x69, 0xB6],
    [0xE7, 0x00, 0x00],
    [0xFF, 0x8C, 0x00],
    [0xFF, 0xEF, 0x00],
    [0x00, 0x81, 0x1F],
    [0x00, 0xC1, 0xC1],
    [0x00, 0x44, 0xFF],
    [0x76, 0x00, 0x89],
];

let luvs = luv::rgbs_to_luvs(&rgbs);
```

```rust
use luv::rgb_bytes_to_luvs;

let rgbs = vec![
    0xFF, 0x69, 0xB6,
    0xE7, 0x00, 0x00,
    0xFF, 0x8C, 0x00,
    0xFF, 0xEF, 0x00,
    0x00, 0x81, 0x1F,
    0x00, 0xC1, 0xC1,
    0x00, 0x44, 0xFF,
    0x76, 0x00, 0x89,
];

let luvs = rgb_bytes_to_luvs(&rgbs);
```

# Features

The crate defines an `approx` feature.  If enabled, approximate
equality as defined by [`approx`
crate](https://crates.io/crates/approx) will be implemented for the
`Luv` and `LCh` types.

# Other crates

The design — and to some degree code — of this crate has been based on the
[`lab` crate](https://crates.io/crates/lab) which provides routines for
converting colours between sRGB, L\*a\*\b and LCh(ab) colour spaces.

For conversion between sRGB and XYZ colour spaces this crate relies on the
[`srgb` crate](https://crates.io/crates/srgb).
