use std::fs;
mod file_read;
mod string_divider;
mod reader;
use crate::string_divider::line;
   
fn main() {
    
    
    //getting Directory File Names

    let mut paths = fs::read_dir("./").unwrap();
    let mut filenames:Vec<String>=Vec::new();
    for path in &mut paths {
        let mut path_=path.iter();
        if let Some(a)=path_.next(){
            let fname = a.file_name();
            let name_=fname.to_str().clone();
             let mut name=String::new();
            if let Some(string)=name_{
                name = String::from(string).clone();
            }
            filenames.push(name);

        }

    }


    //  Reading All Files
    let mut files_data:Vec<Vec<line>>=Vec::new();
    

    for elem in filenames{
        if elem==String::from(".git") {
            continue;
        }
        let file_data = file_read::get_data(elem.clone());
        let mut chunks=string_divider::get_chunks(file_data,elem.clone());
        files_data.push(chunks);
        // for chunk in chunks {
        //     println!("{}",chunk.get_print_data());
        // }

    }

    // Asking User for Input
    println!("\nEnter word to search \n");
    let input  = reader::read_console_line();

    let mut matched :Vec<line> = Vec::new();
    for a_file in files_data{
        for a_line in a_file{
            if a_line.contains_data(input.clone()){
                matched.push(a_line);
            }

        }
    }

    for a_match in matched {
        println!("{}",a_match.get_print_data());
    }














}