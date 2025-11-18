#include <stdint.h>
#include "../target/scheme.h"

extern uintptr_t scm_main();
// (function scm_main () (commit_sexpr (read_sexpr)))
extern uintptr_t scm_main() {
uintptr_t tmp0;
// (commit_sexpr (read_sexpr))
uintptr_t tmp1;
// (read_sexpr)
(tmp1 = (uintptr_t ) read_sexpr());
(tmp0 = (uintptr_t ) commit_sexpr((const struct Sexpr *) tmp1));
return tmp0;
}
