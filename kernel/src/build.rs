// Track "external" changes that should force a rebuild.

use std::env;

fn main() {
    let link_script = env::var("LINK_SCRIPT").unwrap();

    println!("cargo:rerun-if-changed={}", link_script);
}