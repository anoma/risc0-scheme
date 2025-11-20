#include <stdint.h>
#include "../target/scheme.h"

extern uintptr_t scm_main();
// (function scm_main () (function sum (acc pos vec) (if (== (vector_length vec) pos) acc (sum (+ acc (vector_ref vec pos)) (+ pos 1) vec))) (commit_integer (sum 0 0 (read_vector))))
extern uintptr_t scm_main() {
  __label__ tmp0;
  auto uintptr_t sum(uintptr_t acc, uintptr_t pos, uintptr_t vec);
 tmp0:
  uintptr_t tmp1;
  // (function sum (acc pos vec) (if (== (vector_length vec) pos) acc (sum (+ acc (vector_ref vec pos)) (+ pos 1) vec)))
  auto uintptr_t sum(uintptr_t acc, uintptr_t pos, uintptr_t vec) {
    __label__ tmp2;
  tmp2:
    uintptr_t tmp3;
    // (if (== (vector_length vec) pos) acc (sum (+ acc (vector_ref vec pos)) (+ pos 1) vec))
    uintptr_t tmp4;
    // (== (vector_length vec) pos)
    uintptr_t tmp5;
    // (vector_length vec)
    uintptr_t tmp6;
    (tmp6 = (uintptr_t ) vec);
    (tmp5 = (uintptr_t ) vector_length(tmp6));
    uintptr_t tmp7;
    (tmp7 = (uintptr_t ) pos);
    (tmp4 = (uintptr_t ) (tmp5 == tmp7));
    if(tmp4) {
      (tmp3 = (uintptr_t ) acc);
    } else {
      // (sum (+ acc (vector_ref vec pos)) (+ pos 1) vec)
      uintptr_t tmp8;
      // (+ acc (vector_ref vec pos))
      uintptr_t tmp9;
      (tmp9 = (uintptr_t ) acc);
      uintptr_t tmp10;
      // (vector_ref vec pos)
      uintptr_t tmp11;
      (tmp11 = (uintptr_t ) vec);
      uintptr_t tmp12;
      (tmp12 = (uintptr_t ) pos);
      (tmp10 = (uintptr_t ) vector_ref(tmp11, tmp12));
      (tmp8 = (uintptr_t ) (tmp9 + tmp10));
      uintptr_t tmp13;
      // (+ pos 1)
      uintptr_t tmp14;
      (tmp14 = (uintptr_t ) pos);
      uintptr_t tmp15;
      (tmp15 = (uintptr_t ) 1);
      (tmp13 = (uintptr_t ) (tmp14 + tmp15));
      uintptr_t tmp16;
      (tmp16 = (uintptr_t ) vec);
      (acc = tmp8);
      (pos = tmp13);
      (vec = tmp16);
      goto tmp2;
    }
    return tmp3;
  }
  (tmp1 = (uintptr_t ) &sum);
  // (commit_integer (sum 0 0 (read_vector)))
  uintptr_t tmp17;
  // (sum 0 0 (read_vector))
  uintptr_t tmp18;
  (tmp18 = (uintptr_t ) 0);
  uintptr_t tmp19;
  (tmp19 = (uintptr_t ) 0);
  uintptr_t tmp20;
  // (read_vector)
  (tmp20 = (uintptr_t ) read_vector());
  (tmp17 = (uintptr_t ) sum(tmp18, tmp19, tmp20));
  (tmp1 = (uintptr_t ) commit_integer(tmp17));
  return tmp1;
}
