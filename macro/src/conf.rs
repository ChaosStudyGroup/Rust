use serde::Deserialize;
use toml::map::Map;
use toml::Value;
use std::fs::File;
use std::io::Read;
use crate::CONF_FILE_PATH;

#[derive(Deserialize)]
#[derive(Debug)]
pub struct Basic{
    pub derive:String
}

#[derive(Deserialize)]
#[derive(Debug)]
pub struct StructEntry{
    pub name:String,
    pub tagged:String,
    pub fields:Option<Map<String,Value>>
}


#[derive(Deserialize)]
#[derive(Debug)]
pub struct Conf{
    pub basic:Basic,
    pub structs:Vec<StructEntry>
}

impl Conf{
    pub fn read_config()->Self{
        let file_path = CONF_FILE_PATH;
        let mut file = match File::open(file_path){
            Ok(f)=>f,
            Err(e)=>{panic!("no such file");}
        };

        let mut str_val = String::new();
        match file.read_to_string(&mut str_val) {
            Ok(s)=>s,
            Err(e)=>panic!("error reading file {}",e)
        };

        toml::from_str::<Conf>(&str_val).unwrap()
    }
}

