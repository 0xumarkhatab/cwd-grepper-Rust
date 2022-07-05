

use std::fs::File;
use std::io::prelude::*;

pub fn get_data(name:String) ->String{
 let mut file = File::open(String::from("./") + &name)
  .expect("File not found");
  let mut data = String::new();
 file.read_to_string(&mut data)
  .expect("Error while reading file");
return data;

}