// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::props::EnumeratedProperty;

/// ```
/// use icu_properties::{props::NumericValue, CodePointMapData};
///
/// assert_eq!(
///     CodePointMapData::<NumericValue>::new().get('4').get(),
///     Some(4.0)
/// );
/// assert_eq!(
///     CodePointMapData::<NumericValue>::new().get('⅕').get(),
///     Some(1.0 / 5.0)
/// );
/// assert_eq!(CodePointMapData::<NumericValue>::new().get('t').get(), None);
/// ```
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(clippy::exhaustive_structs)] // newtype
#[repr(transparent)]
pub struct NumericValue(pub(crate) i32);

impl NumericValue {
    /// Returns an ICU4C numeric type value.
    // TODO: i am not sure how useful this is tbh
    pub const fn to_icu4c_value(self) -> i32 {
        self.0
    }
    /// Constructor from an ICU4C numeric type value.
    // TODO: i am not sure how useful this is tbh
    pub const fn from_icu4c_value(value: i32) -> Self {
        Self(value)
    }
}

/// Helper constants for the `Numeric_Value` representation from icu4c, as
/// exported in the nv.toml icuexportdata file.
///
/// The original definitions for this can be found at
/// <https://github.com/unicode-org/icu/blob/main/icu4c/source/common/uprops.h>
mod ntv {
    /// No numeric value.
    pub const UPROPS_NTV_NONE: i32 = 0;
    /// Decimal digits: nv=0..9
    pub const UPROPS_NTV_DECIMAL_START: i32 = 1;
    /// Other digits: nv=0..9
    pub const UPROPS_NTV_DIGIT_START: i32 = 11;
    /// Small integers: nv=0..154
    pub const UPROPS_NTV_NUMERIC_START: i32 = 21;
    /// Fractions: ((ntv>>4)-12) / ((ntv&0xf)+1) = -1..17 / 1..16
    pub const UPROPS_NTV_FRACTION_START: i32 = 0xb0;
    /// Large integers:
    /// ((ntv>>5)-14) * 10^((ntv&0x1f)+2) = (1..9)*(10^2..10^33)
    /// (only one significant decimal digit)
    pub const UPROPS_NTV_LARGE_START: i32 = 0x1e0;
    /// Sexagesimal numbers:
    /// ((ntv>>2)-0xbf) * 60^((ntv&3)+1) = (1..9)*(60^1..60^4)
    pub const UPROPS_NTV_BASE60_START: i32 = 0x300;
    /// Fraction-20 values:
    /// frac20 = ntv-0x324 = 0..0x17 -> 1|3|5|7 / 20|40|80|160|320|640
    /// numerator: num = 2*(frac20&3)+1
    /// denominator: den = 20<<(frac20>>2)
    pub const UPROPS_NTV_FRACTION20_START: i32 = UPROPS_NTV_BASE60_START + 36;
    /// Fraction-32 values:
    /// frac32 = ntv-0x34c = 0..15 -> 1|3|5|7 / 32|64|128|256
    /// numerator: num = 2*(frac32&3)+1
    /// denominator: den = 32<<(frac32>>2)
    pub const UPROPS_NTV_FRACTION32_START: i32 = UPROPS_NTV_FRACTION20_START + 24;
    /// No numeric value (yet).
    pub const UPROPS_NTV_RESERVED_START: i32 = UPROPS_NTV_FRACTION32_START + 16;
}

impl NumericValue {
    /// Get the numeric value as a floating point number, returning [`None`], when the
    /// character does not have an associated numeric value.
    ///
    /// ```
    /// use icu_properties::{props::NumericValue, CodePointMapData};
    ///
    /// assert_eq!(
    ///     CodePointMapData::<NumericValue>::new().get('4').get(),
    ///     Some(4.0)
    /// );
    /// assert_eq!(
    ///     CodePointMapData::<NumericValue>::new().get('⅕').get(),
    ///     Some(1.0 / 5.0)
    /// );
    /// assert_eq!(CodePointMapData::<NumericValue>::new().get('t').get(), None);
    /// ```
    pub fn get(&self) -> Option<f64> {
        if self.0 == ntv::UPROPS_NTV_NONE {
            None
        } else if self.0 < ntv::UPROPS_NTV_DIGIT_START {
            // decimal digit
            let value = self.0 - ntv::UPROPS_NTV_DECIMAL_START;
            Some(value as f64)
        } else if self.0 < ntv::UPROPS_NTV_NUMERIC_START {
            // other digit
            let value = self.0 - ntv::UPROPS_NTV_DIGIT_START;
            Some(value as f64)
        } else if self.0 < ntv::UPROPS_NTV_FRACTION_START {
            // small integer
            let value = self.0 - ntv::UPROPS_NTV_NUMERIC_START;
            Some(value as f64)
        } else if self.0 < ntv::UPROPS_NTV_LARGE_START {
            // fraction
            let numerator = (self.0 >> 4) - 12;
            let denominator = (self.0 & 0xf) + 1;
            Some(numerator as f64 / denominator as f64)
        } else if self.0 < ntv::UPROPS_NTV_BASE60_START {
            // large, single-significant-digit integer
            let mant = (self.0 >> 5) - 14;
            let exp = (self.0 & 0x1f) + 2;

            let mut value = mant as f64;
            value *= 10i32.pow(exp as u32) as f64;

            Some(value)
        } else if self.0 < ntv::UPROPS_NTV_FRACTION20_START {
            // sexagesimal (base 60) integer
            let mant = (self.0 >> 2) - 0xbf;
            let exp = (self.0 & 3) + 1;

            let mut value = mant as f64;
            value *= 60u32.pow(exp as u32) as f64;

            Some(value)
        } else if self.0 < ntv::UPROPS_NTV_FRACTION32_START {
            // fraction-20 e.g. 3/80
            let frac20 = self.0 - ntv::UPROPS_NTV_FRACTION20_START; // 0..0x17

            let numerator = 2 * (frac20 & 3) + 1;
            let denominator = 20 << (frac20 >> 2);

            Some(numerator as f64 / denominator as f64)
        } else if self.0 < ntv::UPROPS_NTV_RESERVED_START {
            // fraction-32 e.g. 3/64
            let frac32 = self.0 - ntv::UPROPS_NTV_FRACTION32_START; // 0..15
            let numerator = 2 * (frac32 & 3) + 1;
            let denominator = 32 << (frac32 >> 2);

            Some(numerator as f64 / denominator as f64)
        } else {
            // reserved
            None
        }
    }
}

impl EnumeratedProperty for NumericValue {
    type DataMarker = crate::provider::PropertyEnumNumericValueV1;
    #[cfg(feature = "compiled_data")]
    const SINGLETON: &'static crate::provider::PropertyCodePointMap<'static, Self> =
        crate::provider::Baked::SINGLETON_PROPERTY_ENUM_NUMERIC_VALUE_V1;
    const NAME: &'static [u8] = "Numeric_Value".as_bytes();
    const SHORT_NAME: &'static [u8] = "nv".as_bytes();
}

impl crate::private::Sealed for NumericValue {}

impl zerovec::ule::AsULE for NumericValue {
    type ULE = zerovec::ule::RawBytesULE<4>;
    fn to_unaligned(self) -> Self::ULE {
        self.0.to_unaligned()
    }
    fn from_unaligned(unaligned: Self::ULE) -> Self {
        Self(zerovec::ule::AsULE::from_unaligned(unaligned))
    }
}

#[cfg(feature = "datagen")]
impl databake::Bake for NumericValue {
    fn bake(&self, env: &databake::CrateEnv) -> databake::TokenStream {
        env.insert("icu_properties");

        let v = self.0;
        databake::quote!(icu_properties::props::NumericValue::from_icu4c_value(#v))
    }
}
