// Cargo for DLL support
extern crate libloading;

pub fn test_singles()
{
    unsafe
    {
        // Loading our DLL
        let pblib: libloading::Library = libloading::Library::new("pblib.dll").unwrap();

        // Declaring our functions from DLL
        let passing_single_byval: libloading::Symbol<unsafe extern fn(f32)> = pblib.get(b"passing_single_byval").unwrap();
        let passing_single_byref: libloading::Symbol<unsafe extern fn(*mut f32)> = pblib.get(b"passing_single_byref").unwrap();
        let passing_single_byref_with_change: libloading::Symbol<unsafe extern fn(*mut f32)> = pblib.get(b"passing_single_byref_with_change").unwrap();

        // Calling the functions        
        let mut f32_var:f32 = 1.111111;

        println!("Passing f32_var={} 'passing_single_byval'", f32_var);
        passing_single_byval(f32_var);
        println!();

        println!("Passing f32_var={} 'passing_single_byref'", f32_var);
        passing_single_byref(&mut f32_var);
        println!();

        println!("Passing f32_var={} 'passing_single_byref_with_change'", f32_var);
        passing_single_byref_with_change(&mut f32_var);
        println!("f32_var changed to {}", f32_var);
        println!();
    }    
}

pub fn test_doubles()
{
    unsafe
    {
        // Loading our DLL
        let pblib: libloading::Library = libloading::Library::new("pblib.dll").unwrap();

        // Declaring our functions from DLL
        let passing_double_byval: libloading::Symbol<unsafe extern fn(f64)> = pblib.get(b"passing_double_byval").unwrap();
        let passing_double_byref: libloading::Symbol<unsafe extern fn(*mut f64)> = pblib.get(b"passing_double_byref").unwrap();
        let passing_double_byref_with_change: libloading::Symbol<unsafe extern fn(*mut f64)> = pblib.get(b"passing_double_byref_with_change").unwrap();

        // Calling the functions        
        let mut f64_var:f64 = 1.111111111111;

        println!("Passing f64_var={} 'passing_double_byval'", f64_var);
        passing_double_byval(f64_var);
        println!();

        println!("Passing f64_var={} 'passing_double_byref'", f64_var);
        passing_double_byref(&mut f64_var);
        println!();

        println!("Passing f64_var={} 'passing_double_byref_with_change'", f64_var);
        passing_double_byref_with_change(&mut f64_var);
        println!("f64_var changed to {}", f64_var);
        println!();
    }    
}
