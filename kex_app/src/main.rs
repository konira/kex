use std::env;

use kex_bootstrap::init;

fn main() {
    init(env::args().collect());
}
