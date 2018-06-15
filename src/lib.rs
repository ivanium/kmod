//! A simple kernel module in Rust
#![feature(lang_items, core_str_ext)]
#![allow(unused_features)]
#![feature(lang_items)]
#![no_std]

#[macro_use]
mod print;
pub mod lang_items;

extern {
    fn register_mod_add(f: u64);
    fn unregister_mod_add();    
    // fn register_mod_mul(f: *const u8);
    // fn unregister_mod_mul();
    // fn add(a: i32, b: i32, c: *mut i32);
    // fn mul(a: i32, b: i32, c: *mut i32);
}

#[no_mangle]
pub unsafe extern fn add(a: i32, b: i32, c: *mut i32) {
    println!("in add");
    *c = b + a;
}

// #[no_mangle]
// pub unsafe fn mul(a: i32, b: i32, c: *mut i32) {
//     *c = a * b;
// }

// macro_rules! export_add {
//     ($func: ident) => {
//         unsafe { register_mod_add(&$func as *const _ as *const u8 as u64); }
//     };
// }

#[no_mangle]
/// Entry point for the kernel module
pub fn init_module() -> i32 {
    println!("Module initialized.");
    let a=99;
    let b=98;
    let mut c=97;
    unsafe{
        add(a,b,&c as *const i32 as *mut i32);
        //add(a,b,c as *mut i32);
    }
    println!("reg_ init");
    unsafe { register_mod_add(&(cleanup_module as *const ()) as *const _ as *const u8 as u64); }
    // unsafe { export_add!(add) }
    println!("Mod add registered!");

    return 0;
}

#[no_mangle]
/// Exit point for the kernel module
pub fn cleanup_module() {
    println!("Module exit.");
    unsafe { unregister_mod_add(); }
    println!("finish unregister mod add.");
}
