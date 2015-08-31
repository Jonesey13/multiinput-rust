/*
Reads a vector of file addresses and stores the corresponding shaders
into a vector of strings
*/

use std::io::prelude::*;
use std::fs::File;
pub struct Shader{
    pub shaders: Vec<String>
}

impl Shader{
    pub fn new(shaders: Vec<&'static str>)->Shader{
        let mut strings :Vec<String> = Vec::new();
        for shad in &shaders{
            let mut shader_file = match File::open(shad){
                Ok(file) => file,
                Err(_) => match File::open("examples/".to_string() + shad){
                    Ok(file) => file,
                    Err(_) => match File::open("../".to_string() + shad){
                        Ok(file) => file,
                        Err(_) => panic!("Failed to Load Shader {}", shad),
                    }
                }
            };
            let mut string_shader = String::new();
            match shader_file.read_to_string(& mut string_shader){
                Ok(_) => println!(""),
                Err(_) => println!("Failed to Read {}", shad),
                };
            strings.push(string_shader.clone());
            }
        Shader{
            shaders: strings
        }
    }

}
