use memory_rs::external::process::Process;
use simple_injector::inject_dll;
use std::env::current_exe;

fn main() {
    let p = Process::new("Taskmgr.exe").unwrap();
    let mut path = current_exe().unwrap();
    path.pop();
    let path_string = path.to_string_lossy();

    let dll_path = format!("{}/cpu_hijack.dll", path_string).to_string();

    inject_dll(&p, &dll_path);
}
