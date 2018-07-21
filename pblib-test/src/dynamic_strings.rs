// Cargo for DLL support
extern crate libloading;
extern crate ascii;
extern crate winapi;

// Adjusting the signatures for our use
extern "system" {
    pub fn SysAllocStringByteLen(psz: *const u8, len: usize) -> *const u8;

    pub fn SysFreeString(bstrString: *const u8);

    pub fn SysStringByteLen(bstr: *const u8) -> u32;    
}

// Custom PowerString
pub struct PowerString(*const u8);

// For creating from Rust str
impl<'a> From<&'a str> for PowerString {
    
    fn from(str_text: &'a str) -> Self {
        unsafe {
            let ascii_str = ascii::AsciiStr::from_ascii(str_text).unwrap();
            let byte_slice = ascii_str.as_bytes();
            let ptr = SysAllocStringByteLen(&byte_slice[0], ascii_str.len());

            PowerString(ptr)
        }
    }
}

// For releasing
impl Drop for PowerString {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            SysFreeString(self.0);
        }
    }
}

// Convenient, custom functions
impl PowerString
{
    pub fn len(&self) -> u32 {
        unsafe {
            SysStringByteLen(self.0)
        }
    }
    
    pub fn to_string<'v>(&self) -> String {
        unsafe {
            let len = self.len();            
            let slice: &[u8] = ::std::slice::from_raw_parts(self.0, len as usize);

            String::from(::std::str::from_utf8(slice).unwrap())
        }
    }      
}

pub fn test_strings() {
    unsafe {
        // Loading our DLL
        let pblib: libloading::Library = libloading::Library::new("pblib.dll").unwrap();

        // Declaring our functions from DLL
        let passing_string_byval: libloading::Symbol<unsafe extern "stdcall" fn(PowerString)> = pblib.get(b"passing_string_byval").unwrap();
        let passing_string_byref: libloading::Symbol<unsafe extern "stdcall" fn(*mut PowerString)> = pblib.get(b"passing_string_byref").unwrap();
        let passing_string_byref_with_change: libloading::Symbol<unsafe extern "stdcall" fn(*mut PowerString)> = pblib.get(b"passing_string_byref_with_change").unwrap();

        let text1 = PowerString::from("Ciao PowerBASIC, sending something byVal");
        println!("Passing text={:?} 'passing_string_byval'", text1.to_string());
        passing_string_byval(text1);
        println!();

        let mut text2 = PowerString::from("Ciao PowerBASIC, sending something byRef");
        println!("Passing text={:?} 'passing_string_byref'", text2.to_string());
        passing_string_byref(&mut text2);
        println!();

        let mut text3 = PowerString::from("Ciao PowerBASIC, change this byRef please");
        println!("Passing text={:?} 'passing_string_byref_with_change'", text3.to_string());        
        passing_string_byref_with_change(&mut text3);
        println!("text changed to {:?}", text3.to_string());
        println!();
    }    
}
