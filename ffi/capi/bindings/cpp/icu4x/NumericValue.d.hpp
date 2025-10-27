#ifndef ICU4X_NumericValue_D_HPP
#define ICU4X_NumericValue_D_HPP

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
struct NumericValue;
} // namespace icu4x



namespace icu4x {
namespace capi {
    struct NumericValue {
      int32_t value;
    };

    typedef struct NumericValue_option {union { NumericValue ok; }; bool is_ok; } NumericValue_option;
} // namespace capi
} // namespace


namespace icu4x {
/**
 * See the [Rust documentation for `NumericValue`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.NumericValue.html) for more information.
 */
struct NumericValue {
    int32_t value;

  /**
   * See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.EnumeratedProperty.html#tymethod.for_char) for more information.
   */
  inline static icu4x::NumericValue for_char(char32_t ch);

  /**
   * See the [Rust documentation for `get`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.NumericValue.html#method.get) for more information.
   */
  inline std::optional<double> get() const;

  /**
   * Convert to an integer value usable with ICU4C and CodePointMapData
   *
   * See the [Rust documentation for `to_icu4c_value`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.NumericValue.html#method.to_icu4c_value) for more information.
   */
  inline int32_t to_integer_value() const;

  /**
   * Convert from an integer value from ICU4C or CodePointMapData
   *
   * See the [Rust documentation for `from_icu4c_value`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.NumericValue.html#method.from_icu4c_value) for more information.
   */
  inline static icu4x::NumericValue from_integer_value(int32_t value);

    inline icu4x::capi::NumericValue AsFFI() const;
    inline static icu4x::NumericValue FromFFI(icu4x::capi::NumericValue c_struct);
};

} // namespace
#endif // ICU4X_NumericValue_D_HPP
