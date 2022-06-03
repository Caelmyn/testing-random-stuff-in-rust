#![deny(missing_docs)]

//! Compute a delta between two values and keep the last
//! value for future updates. The delta computation takes interger
//! overflow into account.
//!
//! ### Exemples
//! ```
//! use delta::Delta;
//!
//! let mut delta = 0u16;
//!
//! assert_eq!(toto.delta(1), 1);
//! assert_eq!(toto.delta(1), 0);
//! assert_eq!(toto.delta(1000), 999);
//! assert_eq!(toto.delta(u16::MAX), u16::MAX - 1000);
//! assert_eq!(toto.delta(u16::MAX), 0);
//!
//! // Overflow!
//! assert_eq!(toto.delta(0), 1);
//! ```

/* ---------- */

use std::ops::{Add, Sub};

/* ---------- */

/// The trait to implement the delta computation.
pub trait Delta
where
    Self: Sized + Copy,
{
    /// The type of the value representing the delta.
    type Type: Copy + Add<Output = Self::Type> + Sub<Output = Self::Type> + PartialOrd;

    /// Max value of the implementor type.
    const MAX: Self::Type;

    /// Min value of the implementor type.
    const MIN: Self::Type;

    /// Identity value of the implementor type, usually `1`.
    const ONE: Self::Type;

    /// Compute the delta between the current value and a new one,
    /// updating the current value in the process.
    fn delta(&mut self, new: Self::Type) -> Self::Type {
        let val = self.get();

        let delta = if new >= val {
            new - val
        } else {
            (new - Self::MIN) + (Self::MAX - val) + Self::ONE
        };

        self.set(new);
        delta
    }

    /// Returns the inner value.
    fn get(&self) -> Self::Type;

    /// Set the inner value.
    fn set(&mut self, _: Self::Type);
}

/* ---------- */

macro_rules! impl_delta {
    ($($t:ty),+) => {
        $(
            impl Delta for $t {
                type Type = Self;
                const MAX: Self::Type = Self::MAX;
                const MIN: Self::Type = Self::MIN;
                const ONE: Self::Type = 1;

                #[inline(always)]
                fn get(&self) -> Self::Type {
                    *self
                }

                #[inline(always)]
                fn set(&mut self, val: Self::Type) {
                    *self = val
                }
            }
        )+
    };
}

impl_delta!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

/* ---------- */

#[cfg(test)]
mod tests {
    use crate::Delta;

    #[test]
    fn basic_delta_test() {
        let mut delta = 0u16;

        assert_eq!(delta.delta(1), 1);
        assert_eq!(delta.delta(1), 0);
        assert_eq!(delta.delta(1000), 999);
        assert_eq!(delta.delta(u16::MAX), u16::MAX - 1000);
        assert_eq!(delta.delta(u16::MAX), 0);
        assert_eq!(delta.delta(0), 1);
    }

    #[test]
    fn custom_type_delta_test() {
        #[derive(Copy, Clone)]
        struct DeltaU48(u64);

        impl From<u64> for DeltaU48 {
            fn from(val: u64) -> Self {
                Self(val)
            }
        }

        impl Delta for DeltaU48 {
            type Type = u64;
            const MAX: Self::Type = 0xFFFF_FFFF_FFFF;
            const MIN: Self::Type = 0;
            const ONE: Self::Type = 1;

            fn get(&self) -> Self::Type {
                self.0
            }

            fn set(&mut self, val: Self::Type) {
                self.0 = val
            }
        }

        let mut delta = DeltaU48::from(0);

        assert_eq!(delta.delta(1), 1);
        assert_eq!(delta.delta(1), 0);
        assert_eq!(delta.delta(1000), 999);
        assert_eq!(delta.delta(DeltaU48::MAX), DeltaU48::MAX - 1000);
        assert_eq!(delta.delta(DeltaU48::MAX), 0);
        assert_eq!(delta.delta(0), 1);
    }
}
