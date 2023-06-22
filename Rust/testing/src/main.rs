fn main(){
    let mut string = String::from("hola");
    
    let r1 = &string;
    let r2 = &string;
    println!("{} - {}", r1, r2);
    
    let r3 = &mut string;
    println!("r3: {}", r3)
}