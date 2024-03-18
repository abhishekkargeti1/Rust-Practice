fn main() {
    println!("loops in rust");
    first();
    second();
    third();
}
// type of loops in rust 
    // 1 loops
    // 2 while
    // 3 for
fn first(){
    loop{

        // this loop is infinite loop
        println!("i AM ABhishek kargeti");
        break;
    }
}
fn second (){
    let x = 1;
    while x>0{
        println!("We are in while loop");
        break;
    }
}

fn third(){
    for i in 1..90{
        println!("{}",i);
        
    }

}
