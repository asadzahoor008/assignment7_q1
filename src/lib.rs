mod person{
    pub fn print_person_name(name:String){
        println!("Person name is from main module {}",name);
    }
    //inner module
    pub mod users {
        pub fn print_user_name(name:String){
        println!("User name is from inner module {}",name);
    }
    }
}