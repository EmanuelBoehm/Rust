use curl::easy::Easy;
use serde_json::Deserializer;
pub fn main() {

let mut handle = Easy::new();
handle.url("https://www.rust-lang.org/").unwrap();
{
    let mut transfer = handle.transfer();
    println!("{:?}",transfer.unpause_write());
    //transfer.write_function(|new_data| data.push(std::str::from_utf8(new_data)));
    transfer.perform();
}
}
