extern crate gl_generator;

use gl_generator::{Registry, Api, Profile, Fallbacks, GlobalGenerator};
use std::env;
use std::fs::File;
use std::path::Path;

fn main() {
    
    let dest = env::var("OUT_DIR").unwrap();
    let path = &Path::new(&dest).join("bindings.rs");
    let file = File::create_new(path);
    
    if let Ok(mut f) = file {
        Registry::new(Api::Gl, (4, 5), Profile::Core, Fallbacks::All, []).write_bindings(GlobalGenerator, &mut f).unwrap();
    } 

}