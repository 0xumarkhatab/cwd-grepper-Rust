use std::fmt::format;


pub struct line{
    number:u32,
    data:String,

}

impl line {
pub fn get_print_data(&self)->String{
    format!("{} ::  {}",self.number,self.data)
    
}

}

pub fn get_chunks(string:String )->Vec<line>{
    let mut chunkSize = 40;
    let stringLength = string.len();
    let mut i=0;
    // let mut pieces:Vec<String>=Vec::new();
    // let s: String = string.to_owned();
    // let s_slice: &str = &s[..];  // take a full slice of the string
    // while  i < stringLength {
    //     if i+chunkSize >stringLength{
    //         chunkSize = stringLength-i;
    //     }
    //     let data =String::from( &s_slice[i..(i+chunkSize)]);
    //     pieces.push(data.clone());
    //     i+=chunkSize;
    // }
 
    // return pieces;

    let mut split=string.split("\n");
    let mut pieces:Vec<line>=Vec::new();
    let mut line_number=1;
    for s in split {
        let data =s.to_string();
        let theLine=line{number:line_number,data:data.clone()};
        pieces.push(theLine);
        
        line_number+=1;


    }
    return pieces;

}

