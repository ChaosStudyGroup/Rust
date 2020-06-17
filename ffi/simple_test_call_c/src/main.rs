use libc::c_char;
use std::ffi::CString;
use std::ffi::CStr;

extern {
    fn show(input: c_char);
    fn show_str(intput: *const c_char);
    fn show_str_with_free(input:* const c_char); 
    fn get_str()->*const c_char;
}


fn main() {

    let input = 2;
    let c_str = CString::new("123").unwrap();

    //let c_raw = c_str.into_raw();
    unsafe { 
        show(input);
        
        //read only
       // show_str(c_str.as_ptr()); 
        
        //mem_leak
        //show_str(c_str.into_raw()); 
       
        //c free no mem leak
       // show_str_with_free(c_str.into_raw()); 
       
       // show_str(c_raw);
       // let _ = CString::from_raw(c_raw);
       
        //println!("get str {:?}", CStr::from_ptr(get_str()));
        println!("get str {}", CStr::from_ptr(get_str()).to_str().unwrap());

                
    };


}
