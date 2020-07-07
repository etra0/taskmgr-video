use std::ffi::CString;
use std::ptr;
use winapi;
use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID};
use winapi::um::consoleapi::AllocConsole;

const GLOBAL_SETTINGS: usize = 0xFB550;
const GLOBAL_SETTINGS_CPU_OFFSET: usize = 0x944;

const PATH: &'static str = "FULL_PATH_TO_TXT";

// nasty globals
static mut TASKMGR: usize = 0;
// Thanks to the user @fourtyseven for the heads up of this two functions.
static mut _GetBlockColours: usize = 0xaacbc;
static mut _SetBlockData: usize = 0xab614;
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

// Hook a function using jmp far (very nasty)
fn hook_fun(dest: usize, f: *const u8, len: usize) {
    let nops = vec![0x90; len];
    let mut _t: DWORD = 0;

    unsafe { 
        let result = winapi::um::memoryapi::VirtualProtect(
            dest as LPVOID,
            len,
            winapi::um::winnt::PAGE_EXECUTE_READWRITE,
            &mut _t
        );


    }

    let mut ptr = dest as *mut u8;

    let mut ptr = dest as *mut u8;
    // far jmp
    unsafe {
        // mov eax, 
        *ptr = 0x48;
        ptr = (dest + 1) as *mut u8;
        *ptr = 0xB8;

        // val
        let ptr = (dest + 2) as *mut usize;
        *ptr = f as usize;

        let ptr = (dest + 10) as *mut u16;
        *ptr = 0xE0FF;
    }
}

fn parse_arr() -> Vec<Vec<u32>> {
    use std::fs;

    let contents = fs::read_to_string(PATH).expect("Couldn't read the file");

    let out: Vec<Vec<u32>> = contents
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|x| x
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|x| x.parse().unwrap())
            .collect()
        ).collect();

    return out;
}

#[no_mangle]
unsafe extern "system" fn update_func(handle: LPVOID) -> DWORD {
    let mut v10: u32 = 0;
    let mut v11: u32 = 0;
    let mut w: [u16; 5] = unsafe { std::mem::zeroed() } ;


    let GetBlockColours = gen_func!(TASKMGR + _GetBlockColours, "system", [LPVOID, u32, *mut u32, *mut u32]);
    let SetBlockData = gen_func!(TASKMGR + _SetBlockData, "system", [LPVOID, u32, _, u32, u32]);

    // let img = parse_arr();
    for i in 1..1024 {
        (GetBlockColours)(handle, 0, &mut v10, &mut v11);
        v11 = IMG[i][INDEX];
        (SetBlockData)(handle, i as u32, &w, v11, v10);
    }

    INDEX = (INDEX + 1) % IMG[0].len();
    
    return 1;
}

#[no_mangle]
pub unsafe extern "system" fn intercept_input(_: LPVOID) -> DWORD {
    use winapi::um;

    let _name = CString::new("CHARTV.dll").unwrap();
    let chartv = um::libloaderapi::GetModuleHandleA(_name.as_ptr()) as usize;
    let target_addr = chartv + 0x312E;

    let _name = CString::new("Taskmgr.exe").unwrap();
    let taskmgr = um::libloaderapi::GetModuleHandleA(_name.as_ptr()) as usize;

    TASKMGR = taskmgr;

    let pf = update_func;

    AllocConsole();
    IMG = parse_arr();
    println!("{}", IMG[0].len());
    println!("pf {:x}", pf as usize);
    let p = (taskmgr + GLOBAL_SETTINGS + GLOBAL_SETTINGS_CPU_OFFSET) as *mut u32;
    *p = 1024;
    
    hook_fun(taskmgr + 0xab738, pf as *const u8, 5);

    return 1;
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn DllMain(_: HINSTANCE, reason: DWORD, _: LPVOID) -> BOOL {
    unsafe {
        match reason {
            winapi::um::winnt::DLL_PROCESS_ATTACH => {
                winapi::um::processthreadsapi::CreateThread(
                    ptr::null_mut(),
                    0,
                    Some(intercept_input),
                    ptr::null_mut(),
                    0,
                    ptr::null_mut(),
                );
            }
            _ => (),
        };
    }

    return true as BOOL;
}
