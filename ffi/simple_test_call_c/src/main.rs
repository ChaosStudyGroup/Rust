  

use libc;



extern {

    fn show(input:  libc::c_char);

}



fn main() {

    let input = 2;

    unsafe { show(input) };


}
