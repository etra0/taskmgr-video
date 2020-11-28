use std::ffi::CString;
use std::ptr;
use winapi;
use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID};
use winapi::um::consoleapi::AllocConsole;
use memory_rs::internal::*;
use memory_rs::internal::injections::*;

const GLOBAL_SETTINGS: usize = 0xFB550;
const GLOBAL_SETTINGS_CPU_OFFSET: usize = 0x944;

const PATH: &'static str = r"C:\Users\Sebastian\Documents\work\rust\taskmanager\out.txt";

// nasty globals
static mut TASKMGR: usize = 0;
// Thanks to the user @fourtyseven for the heads up of this two functions.
static mut SETBLOCKDATA_OFFSET: usize = 0xab614;
static mut INDEX: usize = 0;
static mut IMG: Vec<Vec<u32>> = Vec::new();

/// It should be something like
/// gen_func!(address, "stdcall", [u32], u32)
macro_rules! gen_func {
    ($addr:expr, $function_type:expr, [$($params:ty),*], $return_type:ty) => {
        std::mem::transmute::<usize,
            extern $function_type fn($($params),*) -> $return_type>($addr);
    };

    // void return
    ($addr:expr, $function_type:expr, [$($params:ty),*]) => {
        std::mem::transmute::<usize,
            extern $function_type fn($($params),*)>($addr);
    };
}


fn parse_arr() -> Vec<Vec<u32>> {
    use std::fs;

    let contents = fs::read_to_string(PATH).expect("Couldn't read the file");

    let out: Vec<Vec<u32>> = contents
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|x| {
            x.split(' ')
                .filter(|s| !s.is_empty())
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect();

    return out;
}

#[no_mangle]
#[allow(non_snake_case)]
extern "system" fn update_func(handle: LPVOID) -> DWORD {
    let v10: u32 = 0;
    let w: [u16; 5] = unsafe { std::mem::zeroed() };

    let SetBlockData = unsafe {
        gen_func!(
            TASKMGR + SETBLOCKDATA_OFFSET,
            "system",
            [LPVOID, u32, _, u32, u32]
        )
    };

    for i in 1..1024 {
        let color = unsafe { IMG[i][INDEX] };
        (SetBlockData)(handle, i as u32, &w, color, v10);
    }

    unsafe {
        INDEX = (INDEX + 1) % IMG[0].len();
    }

    return 1;
}

#[no_mangle]
pub unsafe extern "system" fn intercept_input(_: LPVOID) -> DWORD {
    use winapi::um;

    let proc_inf = process_info::ProcessInfo::new(None).unwrap();

    TASKMGR = proc_inf.addr;


    AllocConsole();
    IMG = parse_arr();
    println!("{}", IMG[0].len());
    println!("{:x?}", proc_inf);
    let p = (proc_inf.addr + GLOBAL_SETTINGS + GLOBAL_SETTINGS_CPU_OFFSET) as *mut u32;
    *p = 1024;

    let mut detour = injections::Detour::new(proc_inf.addr + 0xAB738, 14, update_func as usize, None);

    detour.inject();

    loop {}

    return 1;
}

memory_rs::main_dll!(intercept_input);
