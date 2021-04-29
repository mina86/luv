use approx::{AbsDiffEq, RelativeEq};

use crate::{LCh, Luv};

impl AbsDiffEq<Luv> for Luv {
    type Epsilon = f32;

    fn default_epsilon() -> Self::Epsilon { 0.0001 }

    fn abs_diff_eq(&self, other: &Luv, epsilon: Self::Epsilon) -> bool {
        AbsDiffEq::abs_diff_eq(&self.l, &other.l, epsilon) &&
            AbsDiffEq::abs_diff_eq(&self.u, &other.u, epsilon) &&
            AbsDiffEq::abs_diff_eq(&self.v, &other.v, epsilon)
    }
}

impl RelativeEq<Luv> for Luv {
    fn default_max_relative() -> Self::Epsilon { 0.0001 }

    fn relative_eq(
        &self,
        other: &Luv,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        RelativeEq::relative_eq(&self.l, &other.l, epsilon, max_relative) &&
            RelativeEq::relative_eq(&self.u, &other.u, epsilon, max_relative) &&
            RelativeEq::relative_eq(&self.v, &other.v, epsilon, max_relative)
    }
}

impl AbsDiffEq<LCh> for LCh {
    type Epsilon = f32;

    fn default_epsilon() -> Self::Epsilon { 0.0001 }

    fn abs_diff_eq(&self, other: &LCh, epsilon: Self::Epsilon) -> bool {
        AbsDiffEq::abs_diff_eq(&self.l, &other.l, epsilon) &&
            AbsDiffEq::abs_diff_eq(&self.c, &other.c, epsilon) &&
            AbsDiffEq::abs_diff_eq(&self.h, &other.h, epsilon)
    }
}

impl RelativeEq<LCh> for LCh {
    fn default_max_relative() -> Self::Epsilon { 0.0001 }

    fn relative_eq(
        &self,
        other: &LCh,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        RelativeEq::relative_eq(&self.l, &other.l, epsilon, max_relative) &&
            RelativeEq::relative_eq(&self.c, &other.c, epsilon, max_relative) &&
            RelativeEq::relative_eq(&self.h, &other.h, epsilon, max_relative)
    }
}
