// Cargo for DLL support
extern crate libloading;

pub fn test_bytes()
{
    unsafe
    {
        // Loading our DLL
        let pblib: libloading::Library = libloading::Library::new("pblib.dll").unwrap();

        // Declaring our functions from DLL
        let passing_byte_byval: libloading::Symbol<unsafe extern "stdcall" fn(u8)> = pblib.get(b"passing_byte_byval").unwrap();
        let passing_byte_byref: libloading::Symbol<unsafe extern "stdcall" fn(*mut u8)> = pblib.get(b"passing_byte_byref").unwrap();
        let passing_byte_byref_with_change: libloading::Symbol<unsafe extern "stdcall" fn(*mut u8)> = pblib.get(b"passing_byte_byref_with_change").unwrap();
        let returning_byte: libloading::Symbol<unsafe extern "stdcall" fn() -> u8> = pblib.get(b"returning_byte").unwrap();

        // Calling the functions        
        let mut u8_var:u8 = 128;

        println!("Passing u8_var={} 'passing_byte_byval'", u8_var);
        passing_byte_byval(u8_var);
        println!();

        println!("Passing u8_var={} 'passing_byte_byref'", u8_var);
        passing_byte_byref(&mut u8_var);
        println!();

        println!("Passing u8_var={} 'passing_byte_byref_with_change'", u8_var);
        passing_byte_byref_with_change(&mut u8_var);
        println!("u8_var changed to {}", u8_var);
        println!();

        println!("Calling 'returning_byte'");
        let u8_result = returning_byte();
        println!("u8_result={}", u8_result);
        println!();
    }    
}

pub fn test_words()
{
    unsafe
    {
        // Loading our DLL
        let pblib: libloading::Library = libloading::Library::new("pblib.dll").unwrap();

        // Declaring our functions from DLL
        let passing_word_byval: libloading::Symbol<unsafe extern "stdcall" fn(u16)> = pblib.get(b"passing_word_byval").unwrap();
        let passing_word_byref: libloading::Symbol<unsafe extern "stdcall" fn(*mut u16)> = pblib.get(b"passing_word_byref").unwrap();
        let passing_word_byref_with_change: libloading::Symbol<unsafe extern "stdcall" fn(*mut u16)> = pblib.get(b"passing_word_byref_with_change").unwrap();
        let returning_word: libloading::Symbol<unsafe extern "stdcall" fn() -> u16> = pblib.get(b"returning_word").unwrap();

        // Calling the functions        
        let mut u16_var:u16 = 39000;

        println!("Passing u16_var={} 'passing_word_byval'", u16_var);
        passing_word_byval(u16_var);
        println!();

        println!("Passing u16_var={} 'passing_word_byref'", u16_var);
        passing_word_byref(&mut u16_var);
        println!();

        println!("Passing u16_var={} 'passing_word_byref_with_change'", u16_var);
        passing_word_byref_with_change(&mut u16_var);
        println!("u16_var changed to {}", u16_var);
        println!();

        println!("Calling 'returning_word'");
        let u16_result = returning_word();
        println!("u16_result={}", u16_result);
        println!();
    }    
}

pub fn test_dwords()
{
    unsafe
    {
        // Loading our DLL
        let pblib: libloading::Library = libloading::Library::new("pblib.dll").unwrap();

        // Declaring our functions from DLL
        let passing_dword_byval: libloading::Symbol<unsafe extern "stdcall" fn(u32)> = pblib.get(b"passing_dword_byval").unwrap();
        let passing_dword_byref: libloading::Symbol<unsafe extern "stdcall" fn(*mut u32)> = pblib.get(b"passing_dword_byref").unwrap();
        let passing_dword_byref_with_change: libloading::Symbol<unsafe extern "stdcall" fn(*mut u32)> = pblib.get(b"passing_dword_byref_with_change").unwrap();
        let returning_dword: libloading::Symbol<unsafe extern "stdcall" fn() -> u32> = pblib.get(b"returning_dword").unwrap();

        // Calling the functions        
        let mut u32_var:u32 = 3900000;

        println!("Passing u32_var={} 'passing_dword_byval'", u32_var);
        passing_dword_byval(u32_var);
        println!();

        println!("Passing u32_var={} 'passing_dword_byref'", u32_var);
        passing_dword_byref(&mut u32_var);
        println!();

        println!("Passing u32_var={} 'passing_dword_byref_with_change'", u32_var);
        passing_dword_byref_with_change(&mut u32_var);
        println!("u32_var changed to {}", u32_var);
        println!();

        println!("Calling 'returning_dword'");
        let u32_result = returning_dword();
        println!("u32_result={}", u32_result);
        println!();        
    }    
}
