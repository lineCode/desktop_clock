#![no_main]
extern crate libloading as lib;
#[cfg(windows)] extern crate winapi;

use std::env;
use std::fs::{File, create_dir};
use std::io::Write;
use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use winapi::um::winbase::SetDllDirectoryW;

#[cfg(windows)] const DLL1: &'static [u8] = include_bytes!("../libfreetype-6.dll");
#[cfg(windows)] const DLL2: &'static [u8] = include_bytes!("../libpng16-16.dll");
#[cfg(windows)] const DLL3: &'static [u8] = include_bytes!("../SDL2.dll");
#[cfg(windows)] const DLL4: &'static [u8] = include_bytes!("../SDL2_image.dll");
#[cfg(windows)] const DLL5: &'static [u8] = include_bytes!("../SDL2_ttf.dll");
#[cfg(windows)] const DLL6: &'static [u8] = include_bytes!("../zlib1.dll");

#[cfg(windows)] const CLOCK: &'static [u8] = include_bytes!("../clocklib.dll");

fn convert_ostr(txt: &OsStr) -> Vec<u16> {
    txt.encode_wide().chain(once(0)).collect()
}

fn main() {
    let lib = load_library();
    unsafe {
        let func: lib::Symbol<unsafe extern fn() -> u32> = lib.get(b"main").unwrap();
        func();
    }
}

#[cfg(unix)]
fn load_library() -> lib::Library{
    
}

#[cfg(windows)]
fn load_library() -> lib::Library{
    //将dll解压到临时文件夹
    let tmp_dir = { let mut d = env::temp_dir(); d.push("clock_dlls"); d};
    //println!("tmp_dir={:?}", tmp_dir);
    if !tmp_dir.exists(){
        create_dir(tmp_dir.clone()).unwrap();
    }
    let dll1 = { let mut d = tmp_dir.clone(); d.push("libfreetype-6.dll"); d};
    let dll2 = { let mut d = tmp_dir.clone(); d.push("libpng16-16.dll"); d};
    let dll3 = { let mut d = tmp_dir.clone(); d.push("SDL2.dll"); d};
    let dll4 = { let mut d = tmp_dir.clone(); d.push("SDL2_image.dll"); d};
    let dll5 = { let mut d = tmp_dir.clone(); d.push("SDL2_ttf.dll"); d};
    let dll6 = { let mut d = tmp_dir.clone(); d.push("zlib1.dll"); d};
    let clock = { let mut d = tmp_dir.clone(); d.push("clocklib.dll"); d};

    if !dll1.exists(){ File::create(dll1).unwrap().write_all(DLL1).unwrap(); }
    if !dll2.exists(){ File::create(dll2).unwrap().write_all(DLL2).unwrap(); }
    if !dll3.exists(){ File::create(dll3).unwrap().write_all(DLL3).unwrap(); }
    if !dll4.exists(){ File::create(dll4).unwrap().write_all(DLL4).unwrap(); }
    if !dll5.exists(){ File::create(dll5).unwrap().write_all(DLL5).unwrap(); }
    if !dll6.exists(){ File::create(dll6).unwrap().write_all(DLL6).unwrap(); }

    if !clock.exists(){ File::create(clock.clone()).unwrap().write_all(CLOCK).unwrap(); }


    unsafe{
        let _r = SetDllDirectoryW(convert_ostr(tmp_dir.as_os_str()).as_ptr());
        //println!("SetDllDirectoryW = {}", r);
    }
    lib::Library::new(clock).unwrap()
}

#[cfg(windows)]
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn WinMain() -> i32 {
    main();
    0
}