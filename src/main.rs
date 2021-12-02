mod macros;
mod dict;
use dictmacro::conjugate;

fn main() {
    let w = conjugate!(ruverb, 4, "a", "B");
}
