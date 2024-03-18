
// RUST DON'T HAVE CLASSES AND OBJECT . RUST ONLY HAVE struct  

struct  User{
    name:String,
    company : String,
    age:i32
}

fn main() {
    let mut u1 = User{
        name: String::from("Abhsihek"),
        company :String::from("Abc"),
        age: 100 
    };

    u1.age=200;
    println!("My name is {}  My company is {}  my age is {}",u1.name,u1.company,u1.age);

}
