#ifndef ICU4X_NumericValue_HPP
#define ICU4X_NumericValue_HPP

#include "NumericValue.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "diplomat_runtime.hpp"


namespace icu4x {
namespace capi {
    extern "C" {

    icu4x::capi::NumericValue icu4x_NumericValue_for_char_mv1(char32_t ch);

    typedef struct icu4x_NumericValue_get_mv1_result {union {double ok; }; bool is_ok;} icu4x_NumericValue_get_mv1_result;
    icu4x_NumericValue_get_mv1_result icu4x_NumericValue_get_mv1(icu4x::capi::NumericValue self);

    int32_t icu4x_NumericValue_to_integer_value_mv1(icu4x::capi::NumericValue self);

    icu4x::capi::NumericValue icu4x_NumericValue_from_integer_value_mv1(int32_t value);

    } // extern "C"
} // namespace capi
} // namespace

inline icu4x::NumericValue icu4x::NumericValue::for_char(char32_t ch) {
    auto result = icu4x::capi::icu4x_NumericValue_for_char_mv1(ch);
    return icu4x::NumericValue::FromFFI(result);
}

inline std::optional<double> icu4x::NumericValue::get() const {
    auto result = icu4x::capi::icu4x_NumericValue_get_mv1(this->AsFFI());
    return result.is_ok ? std::optional<double>(result.ok) : std::nullopt;
}

inline int32_t icu4x::NumericValue::to_integer_value() const {
    auto result = icu4x::capi::icu4x_NumericValue_to_integer_value_mv1(this->AsFFI());
    return result;
}

inline icu4x::NumericValue icu4x::NumericValue::from_integer_value(int32_t value) {
    auto result = icu4x::capi::icu4x_NumericValue_from_integer_value_mv1(value);
    return icu4x::NumericValue::FromFFI(result);
}


inline icu4x::capi::NumericValue icu4x::NumericValue::AsFFI() const {
    return icu4x::capi::NumericValue {
        /* .value = */ value,
    };
}

inline icu4x::NumericValue icu4x::NumericValue::FromFFI(icu4x::capi::NumericValue c_struct) {
    return icu4x::NumericValue {
        /* .value = */ c_struct.value,
    };
}


#endif // ICU4X_NumericValue_HPP
