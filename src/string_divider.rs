#![allow(dead_code)]
#![allow(unused_variables)]
#[cfg(feature = "dead_code")]

use std::fmt::format;
extern crate colored;
use colored::*;

pub struct Line{
    number:u32,
    data:String,
    filename:String,

}


pub fn get_chunks(string:String,fname:String )->Vec<Line>{
    let chunk_size = 40;
    let str_length = string.len();
    let split=string.split("\n");
    let mut pieces:Vec<Line>=Vec::new();
    let mut line_number=1;
    for s in split {
        let data =s.to_string();
        let the_line=Line{filename:fname.clone(),number:line_number,data:data.clone()};
        pieces.push(the_line);
        line_number+=1;

    }
    return pieces;

}

impl Line {
pub fn get_print_data(&self)->String{

    let fname=self.filename.as_str().green();
    let line_number=(self.number.to_string()).as_str().red().bold();
    
    let new_data=self.data.clone().cyan();

    return format!("{} :: {} => {}",fname,line_number,new_data);

    
}
pub fn contains_data(&self,substr:String)->bool{
    if self.data.contains(&substr)==true{
        return true;
    }
    else {
        return false;
    }
}

}