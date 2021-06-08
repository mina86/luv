/* This file is part of luv crate.
 * Copyright (c) 2020 🐝🐝🐝
 * Copyright (c) 2021 Michał Nazarewicz <mina86@mina86.com>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE. */

fn luv_eq(
    lhs: &crate::Luv,
    rhs: &crate::Luv,
    eq: impl Fn(f32, f32) -> bool,
) -> bool {
    if !eq(lhs.l, rhs.l) {
        false
    } else if eq(lhs.l, 0.0) || eq(rhs.l, 0.0) {
        true
    } else {
        eq(lhs.u, rhs.u) && eq(lhs.v, rhs.v)
    }
}

fn lch_eq(
    lhs: &crate::LCh,
    rhs: &crate::LCh,
    eq: impl Fn(f32, f32) -> bool,
) -> bool {
    if !eq(lhs.l, rhs.l) {
        false
    } else if eq(lhs.l, 0.0) || eq(rhs.l, 0.0) {
        true
    } else if !eq(lhs.c, rhs.c) {
        false
    } else if eq(rhs.c, 0.0) || eq(rhs.c, 0.0) {
        true
    } else {
        use std::f32::consts::TAU;
        eq(lhs.h.rem_euclid(TAU), rhs.h.rem_euclid(TAU))
    }
}

macro_rules! approx_impl {
    ($t:ty, $eq:ident) => {
        impl approx::AbsDiffEq<$t> for $t {
            type Epsilon = f32;

            fn default_epsilon() -> Self::Epsilon { f32::default_epsilon() }

            fn abs_diff_eq(&self, other: &$t, epsilon: Self::Epsilon) -> bool {
                $eq(self, other, |a, b| a.abs_diff_eq(&b, epsilon))
            }
        }

        impl approx::RelativeEq<$t> for $t {
            fn default_max_relative() -> Self::Epsilon {
                f32::default_max_relative()
            }

            fn relative_eq(
                &self,
                other: &$t,
                epsilon: Self::Epsilon,
                max_relative: Self::Epsilon,
            ) -> bool {
                $eq(self, other, |a, b| {
                    a.relative_eq(&b, epsilon, max_relative)
                })
            }
        }

        impl approx::UlpsEq<$t> for $t {
            fn default_max_ulps() -> u32 { f32::default_max_ulps() }

            fn ulps_eq(&self, other: &$t, epsilon: f32, max_ulps: u32) -> bool {
                $eq(self, other, |a, b| a.ulps_eq(&b, epsilon, max_ulps))
            }
        }
    };
}

approx_impl!(crate::Luv, luv_eq);
approx_impl!(crate::LCh, lch_eq);
