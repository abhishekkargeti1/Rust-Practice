fn main() {
    println!("Owership & reference in rust");
    first();
}
fn first(){
    let x =20;
    println!("{}",x);
    let y =x; 
    println!("{}",y);
    
    
    let a =String::from("Abhishek Kareti");
    let b = a.clone();    
    println!("{}---{}",a,b);
    
    //. reference in rust 

    let mut x =10;
    let y=&mut x;
    *y+=20;
    println!("{}",y);
    println!("{}",x);


}