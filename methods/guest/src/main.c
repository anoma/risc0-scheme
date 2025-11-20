#include <stdint.h>
#include "../target/scheme.h"

extern uintptr_t scm_main();
// (function scm_main () (function sum (acc list) (if (is_null list) acc (sum (+ acc (as_integer (car list))) (cdr list)))) (commit_sexpr (integer (sum 0 (read_sexpr)))))
extern uintptr_t scm_main() {
  __label__ tmp0;
  auto uintptr_t sum(uintptr_t acc, uintptr_t list);
 tmp0:
  uintptr_t tmp1;
  // (function sum (acc list) (if (is_null list) acc (sum (+ acc (as_integer (car list))) (cdr list))))
  auto uintptr_t sum(uintptr_t acc, uintptr_t list) {
    __label__ tmp2;
  tmp2:
    uintptr_t tmp3;
    // (if (is_null list) acc (sum (+ acc (as_integer (car list))) (cdr list)))
    uintptr_t tmp4;
    // (is_null list)
    uintptr_t tmp5;
    (tmp5 = (uintptr_t ) list);
    (tmp4 = (uintptr_t ) is_null(tmp5));
    if(tmp4) {
      (tmp3 = (uintptr_t ) acc);
    } else {
      // (sum (+ acc (as_integer (car list))) (cdr list))
      uintptr_t tmp6;
      // (+ acc (as_integer (car list)))
      uintptr_t tmp7;
      (tmp7 = (uintptr_t ) acc);
      uintptr_t tmp8;
      // (as_integer (car list))
      uintptr_t tmp9;
      // (car list)
      uintptr_t tmp10;
      (tmp10 = (uintptr_t ) list);
      (tmp9 = (uintptr_t ) car(tmp10));
      (tmp8 = (uintptr_t ) as_integer(tmp9));
      (tmp6 = (uintptr_t ) (tmp7 + tmp8));
      uintptr_t tmp11;
      // (cdr list)
      uintptr_t tmp12;
      (tmp12 = (uintptr_t ) list);
      (tmp11 = (uintptr_t ) cdr(tmp12));
      (acc = tmp6);
      (list = tmp11);
      goto tmp2;
    }
    return tmp3;
  }
  (tmp1 = (uintptr_t ) &sum);
  // (commit_sexpr (integer (sum 0 (read_sexpr))))
  uintptr_t tmp13;
  // (integer (sum 0 (read_sexpr)))
  uintptr_t tmp14;
  // (sum 0 (read_sexpr))
  uintptr_t tmp15;
  (tmp15 = (uintptr_t ) 0);
  uintptr_t tmp16;
  // (read_sexpr)
  (tmp16 = (uintptr_t ) read_sexpr());
  (tmp14 = (uintptr_t ) sum(tmp15, tmp16));
  (tmp13 = (uintptr_t ) integer(tmp14));
  (tmp1 = (uintptr_t ) commit_sexpr(tmp13));
  return tmp1;
}
