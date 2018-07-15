// Cargo for DLL support
extern crate libloading;

use std::ffi::CString;
use std::ffi::CStr;
use std::os::raw::c_char;

pub fn test_stringzs()
{
    unsafe
    {
        // Loading our DLL
        let pblib: libloading::Library = libloading::Library::new("pblib.dll").unwrap();

        // Declaring our functions from DLL
        let passing_stringz_byref: libloading::Symbol<unsafe extern fn(*const c_char)> = pblib.get(b"passing_stringz_byref").unwrap();
        let passing_stringz_ptr: libloading::Symbol<unsafe extern fn(*const c_char)> = pblib.get(b"passing_stringz_ptr").unwrap();
        let passing_stringz_ptr_with_change: libloading::Symbol<unsafe extern fn(*const c_char)> = pblib.get(b"passing_stringz_ptr_with_change").unwrap();

        // Calling the functions        
        let text = CString::new("Hello, world!").unwrap();

        println!("Passing text={:?} 'passing_stringz'", text);
        passing_stringz_byref(text.as_ptr());
        println!();

        println!("Passing text={:?} 'passing_stringz_ptr'", text);
        passing_stringz_ptr(text.as_ptr());
        println!();

        let ptr:*const c_char = text.as_ptr();
        println!("Passing text={:?} 'passing_stringz_ptr'", text);
        passing_stringz_ptr_with_change(text.as_ptr());
        let result = CStr::from_ptr(ptr).to_string_lossy().into_owned();
        println!("text changed to {}", result);
        println!();        
    }    
}
