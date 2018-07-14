// Cargo for DLL support
extern crate libloading;

fn main() {    
    // Interaction with non-rust code is considered unsafe
    unsafe
    {
        // Loading our DLL
        let pblib: libloading::Library = libloading::Library::new("pblib.dll").unwrap();

        // Declaring our functions from DLL
        let passing_byte_byval: libloading::Symbol<unsafe extern fn(u8)> = pblib.get(b"passing_byte_byval").unwrap();
        let passing_byte_byref: libloading::Symbol<unsafe extern fn(*mut u8)> = pblib.get(b"passing_byte_byref").unwrap();
        let passing_byte_byref_with_change: libloading::Symbol<unsafe extern fn(*mut u8)> = pblib.get(b"passing_byte_byref_with_change").unwrap();

        let mut u8_var:u8 = 128;

        println!("Passing u8_var={} 'passing_byte_byval'", u8_var);
        passing_byte_byval(u8_var);

        println!("Passing u8_var={} 'passing_byte_byref'", u8_var);
        passing_byte_byref(&mut u8_var);

        println!("Passing u8_var={} 'passing_byte_byref_with_change'", u8_var);
        passing_byte_byref_with_change(&mut u8_var);
        println!("  u8_var changed to {}", u8_var);
    }
}
