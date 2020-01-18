use std::io;
//main module
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
fn main() {

    let mut person_name = String::new();
    println!("Type person name: ");
    io::stdin().read_line(&mut person_name).expect("Failed to read person name");
    person::print_person_name(person_name);

    let mut username = String::new();
    println!("Type user name :");
    io::stdin().read_line(&mut username).expect("Failed to read user name");
    person::users::print_user_name(username);
}
