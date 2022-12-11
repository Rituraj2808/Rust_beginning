//print to console

pub fn run(){
    //print to console
    println!(r#"Hello from the print.rs file"#);

    //rust can only print strings, anything else use placeholders {}
    println!("#{}",1);

    //Basic Formatting
    println!("{} is from {}","Brad","Mass");

    //Positional Arguments
    println!("{0} is from {2},and likes to {1}","Brad","code","Mass");

    //Basic Formatting
    println!("{name} likes playing {act}",name = "Brad",act = "Mass");

    //Placeholder traits
    println!("Binary:{:b},Hex:{:x},Octal:{:o}",100,100,100);

    //Place holder foe debug traits
    println!("{:?}",(12,true,[1,23,4,5]));
}