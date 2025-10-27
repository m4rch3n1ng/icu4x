#ifndef NumericValue_D_H
#define NumericValue_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct NumericValue {
  int32_t value;
} NumericValue;

typedef struct NumericValue_option {union { NumericValue ok; }; bool is_ok; } NumericValue_option;



#endif // NumericValue_D_H
