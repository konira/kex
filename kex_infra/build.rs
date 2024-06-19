fn main() {    
    deps();
}


#[cfg(target_os = "linux")]
fn deps() ->bool {
    true
}
#[cfg(target_os = "windows")]
fn deps()->bool{
    
    println!("cargo:rustc-link-search=native=*******/WpdPack/Lib/x64");    
    println!("cargo:rustc-link-lib=static=Packet");
    bool
}