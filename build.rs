extern crate gl_generator;

use gl_generator::{Api,Fallbacks,GlobalGenerator,Profile,Registry};
use std::fs::File;
use std::path::Path;

fn main(){
    let path = Path::new("src/");
    let mut file = File::create(
        &path.join("gl_bindings.rs")
    ).unwrap();

    Registry::new(
        Api::Gl,
        (4,6),
        Profile::Compatibility,
        Fallbacks::All,
        [])
        .write_bindings(GlobalGenerator,&mut file)
        .unwrap();

}