#[allow(unused_imports)]
use method::*;
#[link(name = "main")]
unsafe extern "C" { fn scm_main() -> u32; }

fn main() {
    unsafe { scm_main(); }
}
