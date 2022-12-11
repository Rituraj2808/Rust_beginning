//variables hold primitive data or reference to data{use & to reference}
//variables are immutable by default
//rust is block - scoped language

pub fn run(){
    let name:&str  = "Brad";
    let  mut age:i16 = 22;
    print!("My name is {}, I am {}\n", name,age);
    age = 32;
    print!("My name is {}, I am {}", name,age);  
    //Assign multiple variables
    let ( my_name, my_age ) = ("Brad", 37);
  println!("{} is {}", my_name, my_age );
}