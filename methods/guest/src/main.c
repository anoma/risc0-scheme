#include <stdint.h>
#include "../target/scheme.h"

extern uintptr_t scm_main();
// (function scm_main () (function sum (acc list) (if (is_null list) acc (sum (+ acc (as_integer (car list))) (cdr list)))) (commit_sexpr (integer (sum 0 (read_sexpr)))))
extern uintptr_t scm_main() {
auto uintptr_t sum(uintptr_t acc, uintptr_t list);
uintptr_t tmp0;
// (function sum (acc list) (if (is_null list) acc (sum (+ acc (as_integer (car list))) (cdr list))))
auto uintptr_t sum(uintptr_t acc, uintptr_t list) {
uintptr_t tmp1;
// (if (is_null list) acc (sum (+ acc (as_integer (car list))) (cdr list)))
uintptr_t tmp2;
// (is_null list)
uintptr_t tmp3;
(tmp3 = (uintptr_t ) list);
(tmp2 = (uintptr_t ) is_null(tmp3));
if(tmp2) {
(tmp1 = (uintptr_t ) acc);
} else {
// (sum (+ acc (as_integer (car list))) (cdr list))
uintptr_t tmp4;
// (+ acc (as_integer (car list)))
uintptr_t tmp5;
(tmp5 = (uintptr_t ) acc);
uintptr_t tmp6;
// (as_integer (car list))
uintptr_t tmp7;
// (car list)
uintptr_t tmp8;
(tmp8 = (uintptr_t ) list);
(tmp7 = (uintptr_t ) car(tmp8));
(tmp6 = (uintptr_t ) as_integer(tmp7));
(tmp4 = (uintptr_t ) (tmp5 + tmp6));
uintptr_t tmp9;
// (cdr list)
uintptr_t tmp10;
(tmp10 = (uintptr_t ) list);
(tmp9 = (uintptr_t ) cdr(tmp10));
acc = tmp4;
list = tmp9;
(tmp1 = (uintptr_t ) sum(tmp4, tmp9));
}
return tmp1;
}
(tmp0 = (uintptr_t ) &sum);
// (commit_sexpr (integer (sum 0 (read_sexpr))))
uintptr_t tmp11;
// (integer (sum 0 (read_sexpr)))
uintptr_t tmp12;
// (sum 0 (read_sexpr))
uintptr_t tmp13;
(tmp13 = (uintptr_t ) 0);
uintptr_t tmp14;
// (read_sexpr)
(tmp14 = (uintptr_t ) read_sexpr());
(tmp12 = (uintptr_t ) sum(tmp13, tmp14));
(tmp11 = (uintptr_t ) integer(tmp12));
(tmp0 = (uintptr_t ) commit_sexpr(tmp11));
return tmp0;
}
