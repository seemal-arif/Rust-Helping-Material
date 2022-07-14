pub fn conversion(x:String)->u32{

    let mut num:u32=0; 
       
    match x.trim().parse::<u32>(){
        Ok(i)=>num=i,
        Err(_)=>()
    }

    return num;

}