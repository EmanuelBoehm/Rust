//use std::path::Path;
//mod configs;
//mod prgm;
//mod installer;
//mod git_status;
mod prims;
//mod sandbox;
mod corona;
fn main() {
        //installer::run(Path::new("./sandbox.csv"));
        corona::main();
         prims::main(Some(100));
        //    git_status::main();

}
//fn main() {
    ////git_init::init(Option::from(Path::new("/home/emi/tttt")));
    ////git_init::init(None);
//}
