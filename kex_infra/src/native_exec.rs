use std::mem;
use std::os::raw::c_void;
#[cfg(target_os = "windows")]
use windows::Win32::System::Memory::{VirtualAlloc, VirtualFree, MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE,MEM_RELEASE};
#[cfg(target_os = "linux")]
use libc::{mmap, munmap, PROT_READ, PROT_WRITE, PROT_EXEC, MAP_PRIVATE, MAP_ANON};

type Func = unsafe extern "C" fn()->i32;

#[cfg(target_os = "windows")]
fn create_executable_memory(size: usize)  -> *mut u8{
    unsafe {
        let addr = VirtualAlloc(
            None,
            size,
            MEM_COMMIT | MEM_RESERVE,
            PAGE_EXECUTE_READWRITE,
        );
        if addr.is_null() {
            panic!("Failed to allocate executable memory");
        }
        addr as *mut u8
    }
}
#[cfg(target_os = "windows")]
pub fn execute_code(code: &[u8]) -> i32{
    let size = code.len();
    let ret: i32;
    unsafe {
        let memory = create_executable_memory(size);
        std::ptr::copy_nonoverlapping(code.as_ptr(), memory as *mut u8, size);
        let func: Func = mem::transmute(memory);
        ret = func();
        let _ = VirtualFree(memory as *mut _, 0, MEM_RELEASE);
    }
    ret
}

#[cfg(target_os = "linux")]
fn create_executable_memory(size: usize) -> *mut u8 {
    unsafe {
        let addr = libc::mmap(
            std::ptr::null_mut(),
            size,
            libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC,
            libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
            -1,
            0,
        );
        if addr == libc::MAP_FAILED {
            panic!("Failed to allocate executable memory");
        }
        addr as *mut u8
    }
}

#[cfg(target_os = "linux")]
pub fn execute_code(code: &[u8]) -> i32 {
    let size = code.len();
    let ret: i32;
    unsafe {
        let memory = create_executable_memory(size);
        std::ptr::copy_nonoverlapping(code.as_ptr(), memory, size);
        let func: extern "C" fn() -> i32 = std::mem::transmute(memory);
        ret = func();
        let _ = libc::munmap(memory as *mut _, size);
    }
    ret
}

#[cfg(target_os = "linux")]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_code() {
        // Este código de bytes representa a seguinte função em assembly:
        // mov eax, 42
        // ret
        let code: &[u8] = &[
            0xb8, 0x2a, 0x00, 0x00, 0x00, // mov eax, 42
            0xc3, // ret
        ];
    
        let ret = execute_code(code);

        assert!(ret == 42, "Expected 42, got {}", ret)
    }
}

#[cfg(target_os = "windows")]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_code() {             
        let code: &[u8] = &[
            0xb8, 0x2a, 0x00, 0x00, 0x00, // mov eax, 42
            0xc3, // ret
        ];        
        let ret = execute_code(code);
        assert_eq!(ret, 42);
    }    
}