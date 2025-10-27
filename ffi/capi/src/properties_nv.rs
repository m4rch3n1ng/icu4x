// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
#[diplomat::abi_rename = "icu4x_{0}_mv1"]
pub mod ffi {
    use diplomat_runtime::{DiplomatChar, DiplomatOption};
    use icu_properties::props;

    #[diplomat::rust_link(icu::properties::props::NumericValue, Struct)]
    pub struct NumericValue {
        pub value: i32,
    }

    impl NumericValue {
        #[diplomat::rust_link(icu::properties::props::EnumeratedProperty::for_char, FnInTrait)]
        #[cfg(feature = "compiled_data")]
        pub fn for_char(ch: DiplomatChar) -> Self {
            icu_properties::CodePointMapData::<props::NumericValue>::new()
                .get32(ch)
                .into()
        }

        #[diplomat::rust_link(icu::properties::props::NumericValue::get, FnInStruct)]
        pub fn get(self) -> DiplomatOption<f64> {
            props::NumericValue::from_icu4c_value(self.value)
                .get()
                .into()
        }

        #[diplomat::rust_link(icu::properties::props::NumericValue::to_icu4c_value, FnInStruct)]
        #[diplomat::attr(demo_gen, disable)] // semi-internal, also too many of these
        /// Convert to an integer value usable with ICU4C and `CodePointMapData`
        pub fn to_integer_value(self) -> i32 {
            self.value
        }

        #[diplomat::rust_link(icu::properties::props::NumericValue::from_icu4c_value, FnInStruct)]
        #[diplomat::attr(demo_gen, disable)] // semi-internal, also too many of these
        /// Convert from an integer value from ICU4C or `CodePointMapData`
        pub fn from_integer_value(value: i32) -> Self {
            Self { value }
        }
    }
}

impl From<icu_properties::props::NumericValue> for ffi::NumericValue {
    fn from(value: icu_properties::props::NumericValue) -> Self {
        Self {
            value: value.to_icu4c_value(),
        }
    }
}
