extern crate cc;

fn main(){
    cc::Build::new()
        .file("c_test/test.c")
        .compile("libtest.a");
}
