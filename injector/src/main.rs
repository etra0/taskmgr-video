use memory_rs::process::process_wrapper::Process;
use std::env;
use std::ffi::CString;
use std::mem;
use std::process;
use std::ptr;
use winapi::shared::basetsd::DWORD_PTR;
use winapi::shared::minwindef::LPVOID;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::libloaderapi::{FreeLibrary, GetModuleHandleA, GetProcAddress};
use winapi::um::memoryapi::VirtualAllocEx;
use winapi::um::processthreadsapi::CreateRemoteThread;
use winapi::um::winbase::FormatMessageA;
use winapi::um::winnt::{MEM_COMMIT, PAGE_READWRITE};

pub unsafe fn inject_dll(process: &Process, name: &str) {
    let dll_dir = CString::new(name).unwrap();
    let dll_dir_s = dll_dir.as_bytes_with_nul().len();

    unsafe {
        // Load kernel32 module in order to get LoadLibraryA
        let s_module_handle = CString::new("Kernel32").unwrap();
        let module_handle = GetModuleHandleA(s_module_handle.as_ptr());

        // Load LoadLibraryA function from kernel32 module
        let a_loadlib = CString::new("LoadLibraryA").unwrap();
        let result = GetProcAddress(module_handle, a_loadlib.as_ptr());
        let casted_function: extern "system" fn(LPVOID) -> u32 = mem::transmute(result);
        println!("{:x}", result as u64);

        // Allocate the space to write the dll direction in the target process
        let addr = VirtualAllocEx(
            process.h_process,
            ptr::null_mut(),
            dll_dir_s,
            MEM_COMMIT,
            PAGE_READWRITE,
        ) as DWORD_PTR;

        let _dll_dir = dll_dir.into_bytes_with_nul();
        process.write_aob(addr, &_dll_dir, true);

        println!("DLL address {:x}", addr);

        let a = CreateRemoteThread(
            process.h_process,
            ptr::null_mut(),
            0,
            Some(casted_function),
            addr as LPVOID,
            0,
            ptr::null_mut(),
        );
        println!("handle {:x?}", a);

        let last_err = GetLastError();
        let mut buffer: Vec<i8> = vec![0; 64];
        FormatMessageA(
            0x1000,
            std::ptr::null(),
            last_err,
            0,
            buffer.as_mut_ptr(),
            64,
            std::ptr::null_mut(),
        );
        let buffer: Vec<u8> = mem::transmute(buffer);
        let msg = String::from_utf8(buffer).unwrap();
        println!("Error: {}", msg);

        FreeLibrary(module_handle);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Not enough arguments.");
        process::exit(0);
    }

    let p_name = args.get(1).unwrap();
    let dll_name = args.get(2).unwrap();
    let process = Process::new(&p_name).unwrap();

    unsafe { inject_dll(&process, &dll_name) };
}
