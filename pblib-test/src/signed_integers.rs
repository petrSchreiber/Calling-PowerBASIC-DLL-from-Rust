// Cargo for DLL support
extern crate libloading;

pub fn test_integers()
{
    unsafe
    {
        // Loading our DLL
        let pblib: libloading::Library = libloading::Library::new("pblib.dll").unwrap();

        // Declaring our functions from DLL
        let passing_integer_byval: libloading::Symbol<unsafe extern fn(i16)> = pblib.get(b"passing_integer_byval").unwrap();
        let passing_integer_byref: libloading::Symbol<unsafe extern fn(*mut i16)> = pblib.get(b"passing_integer_byref").unwrap();
        let passing_integer_byref_with_change: libloading::Symbol<unsafe extern fn(*mut i16)> = pblib.get(b"passing_integer_byref_with_change").unwrap();

        // Calling the functions        
        let mut i16_var:i16 = 20000;

        println!("Passing i16_var={} 'passing_integer_byval'", i16_var);
        passing_integer_byval(i16_var);
        println!();

        println!("Passing i16_var={} 'passing_integer_byref'", i16_var);
        passing_integer_byref(&mut i16_var);
        println!();

        println!("Passing i16_var={} 'passing_integer_byref_with_change'", i16_var);
        passing_integer_byref_with_change(&mut i16_var);
        println!("i16_var changed to {}", i16_var);
        println!();
    }    
}

pub fn test_longs()
{
    unsafe
    {
        // Loading our DLL
        let pblib: libloading::Library = libloading::Library::new("pblib.dll").unwrap();

        // Declaring our functions from DLL
        let passing_long_byval: libloading::Symbol<unsafe extern fn(i32)> = pblib.get(b"passing_long_byval").unwrap();
        let passing_long_byref: libloading::Symbol<unsafe extern fn(*mut i32)> = pblib.get(b"passing_long_byref").unwrap();
        let passing_long_byref_with_change: libloading::Symbol<unsafe extern fn(*mut i32)> = pblib.get(b"passing_long_byref_with_change").unwrap();

        // Calling the functions        
        let mut i32_var:i32 = 1500000000;

        println!("Passing i32_var={} 'passing_long_byval'", i32_var);
        passing_long_byval(i32_var);
        println!();

        println!("Passing i32_var={} 'passing_long_byref'", i32_var);
        passing_long_byref(&mut i32_var);
        println!();

        println!("Passing i32_var={} 'passing_long_byref_with_change'", i32_var);
        passing_long_byref_with_change(&mut i32_var);
        println!("i32_var changed to {}", i32_var);
        println!();
    }    
}

pub fn test_quads()
{
    unsafe
    {
        // Loading our DLL
        let pblib: libloading::Library = libloading::Library::new("pblib.dll").unwrap();

        // Declaring our functions from DLL
        let passing_quad_byval: libloading::Symbol<unsafe extern fn(i64)> = pblib.get(b"passing_quad_byval").unwrap();
        let passing_quad_byref: libloading::Symbol<unsafe extern fn(*mut i64)> = pblib.get(b"passing_quad_byref").unwrap();
        let passing_quad_byref_with_change: libloading::Symbol<unsafe extern fn(*mut i64)> = pblib.get(b"passing_quad_byref_with_change").unwrap();

        // Calling the functions        
        let mut i64_var:i64 = 4900000;

        println!("Passing i64_var={} 'passing_quad_byval'", i64_var);
        passing_quad_byval(i64_var);
        println!();

        println!("Passing i64_var={} 'passing_quad_byref'", i64_var);
        passing_quad_byref(&mut i64_var);
        println!();

        println!("Passing i64_var={} 'passing_quad_byref_with_change'", i64_var);
        passing_quad_byref_with_change(&mut i64_var);
        println!("i64_var changed to {}", i64_var);
        println!();
    }    
}