// build.rs
extern crate bindgen;
extern crate cc;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-search=native=C:/Users/lorib/Downloads/WpdPack_4_1_2/WpdPack/Lib/x64");
    println!("cargo:rustc-link-lib=static=Packet");    
}