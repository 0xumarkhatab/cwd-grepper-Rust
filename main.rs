use std::fs;
mod file_read;

fn main() {
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



}