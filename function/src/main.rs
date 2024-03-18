fn main() { 
    println!("Function");
    first_function();
    second_funtion(23);
    third_function(23,'H');
    ex();

    // calling funtion 
    let x =return_value();
    println!("The value of the funtion is {x}");
}

// default funtion..

fn first_function(){
    println!("I am abhishk");
}

// parameterised function


fn second_funtion(x: i32){
    println!("The value of x  is  {}",x);
}

fn third_function( x:i32, y: char){
    println!("The value of x is  {x}, The value of y is  {y} ");

}

//expressions

// EXPRESSIONS DON'T RETURN ANY VALUE

fn ex(){
    let y ={
        let x = 9;
        x+1

    };
    println!("Value of y is {y}");
}

// return value from function 
fn return_value() -> i32 {
    23 + 45
}