fn main() {
    println!("Datatypes");


    // Scalar type == types where we can store single type of data.
    // integer ,String , Boolean, floating ,Char.
    // char and integer are same in rust.

    // length (8 bits,16 ,32,64,128,arch) signed(i)-i8,i16 or unsigned(u)-u8,u16,

    let number =2;
    println!("{}",number);

    let found = true;
    println!("{found}");
    
    let char1 ="abs";
    println!("{char1}");
    
    let num= 49.09;
    println!("{num}");

    // Compounds types --- types where we store multiple data at a time .
    // arrays,tuples, dictionary

    // tuples
    let tuples = (23,34,45);
    let mut tuples1 = (23,34,45);
    println!("{:?}",tuples);
    println!("{}",tuples.1);
    tuples1.0 = 90;
    println!("{}",tuples.0);
    println!("{:?}",tuples1);

    //Arrays

    let a =[1,2,3,4,5];
    println!("{:?}",a);
    println!("{}",a[2]);



    
}
