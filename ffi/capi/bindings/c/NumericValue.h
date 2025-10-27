#ifndef NumericValue_H
#define NumericValue_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "NumericValue.d.h"






NumericValue icu4x_NumericValue_for_char_mv1(char32_t ch);

typedef struct icu4x_NumericValue_get_mv1_result {union {double ok; }; bool is_ok;} icu4x_NumericValue_get_mv1_result;
icu4x_NumericValue_get_mv1_result icu4x_NumericValue_get_mv1(NumericValue self);

int32_t icu4x_NumericValue_to_integer_value_mv1(NumericValue self);

NumericValue icu4x_NumericValue_from_integer_value_mv1(int32_t value);





#endif // NumericValue_H
