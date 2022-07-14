use std::io:: Write;
use std::io;

pub fn user_input()->String {

    print!("Enter value : ");
    io::stdout().flush().unwrap();
    let mut value:String=String::new();
    let _res=io::stdin().read_line(&mut value).unwrap();
    return value;

}
