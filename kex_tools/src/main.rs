fn main() {
    let bytes = generate_bytes_exec();
    let hex_string: String = bytes.iter().map(|b| format!("{:02x}", b)).collect();
    println!("{}", hex_string);

    unsafe {
        let func_ptr = usize::from_be_bytes(bytes.try_into().unwrap()) as *const ();
        let func: fn() = std::mem::transmute(func_ptr);
        func();
    }
}

fn print_hello() {
    println!("Hello, world!");
    println!("Hello, world!");
    println!("Hello, world!");
    println!("Hello, world!");
}

fn generate_bytes_exec() -> Vec<u8> {
    let mut bytes = vec![];
    let func_ptr = print_hello as *const ();
    let func_ptr = func_ptr as usize;
    bytes.extend_from_slice(&func_ptr.to_be_bytes());
    bytes
}