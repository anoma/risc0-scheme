use risc0_zkvm::guest::env;
use scheme_core::Sexpr;

#[unsafe(no_mangle)]
pub extern "C" fn read_sexpr() -> Box<Sexpr> {
    env::read()
}

#[unsafe(no_mangle)]
pub extern "C" fn commit_sexpr(sexpr: Box<Sexpr>) {
    env::commit(&sexpr)
}

fn main() {
    // TODO: Implement your guest code here

    // read the input
    let input: u32 = env::read();

    // TODO: do something with the input

    // write public output to the journal
    env::commit(&input);
}
