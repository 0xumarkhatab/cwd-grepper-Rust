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

    for elem in filenames{
        if elem==String::from(".git") {
            continue;
        }
        let file_data = file_read::get_data(elem.clone());
        println!("\n\n\t\t ---  Data of {} ---\n{} ",elem,file_data);

    }


}