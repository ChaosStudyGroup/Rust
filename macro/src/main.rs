mod conf;
const CONF_FILE_PATH : &'static str = "confs/method.toml";
use codegen::{gen_struct, gen_struct_by_conf};
gen_struct! {
     Person -> {name: String, age: u32}
}
gen_struct_by_conf!{ confs / method.toml }
fn main() {
    let c = Person::new("Alex".to_string(), 18);
    println!("{:?}",c);

    let c = Person1::new("Alex".to_string(), 18);
    println!("{:?}",c);
    let c = Person2::new();
    println!("{:?}",c);
    let c = Person3::new("Alex".to_string(), 18);
    println!("{:?}",c);
}
