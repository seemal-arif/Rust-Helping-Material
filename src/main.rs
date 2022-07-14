mod user_input;
mod time_delay;
mod integer_string;




fn main(){
    let  value:String=String::from(user_input::user_input());
    println!("value is {}",value);
    time_delay::time_delay();
    println!("Convert string -> integer ");
    let mut num:u32=integer_string::conversion(value.clone());
    num=num+10; 
    println!("Value entered by user is {}",num);
  
}