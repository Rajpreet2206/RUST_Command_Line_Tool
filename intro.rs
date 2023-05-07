fn main(){
    let x = 5;
    //Here we have defined a variable x and assigned a value 5 to it
    println!("The value of variable x = {}", x);
}

//To understand the mutability of a particular variable

fn main(){
    let x = 5;
    println!("The value of variable x is = {}", x);
    x = 10;
    println!("The updated value of the variable x is = {}", x);
} //This will give an error bcz, variables are immutable by default

//So, we need to do something more

fn main(){
    let mut x = 5;
    println!("The value of variable x is = {}",x);
    let x = 10;
    println!("The updated value of the variable x is = {}", x);
}//Now, this should work, bcz we have updated the variable declaration as "mut"

//Consider the binary sequence 01000001. This is number 65 as well as the characted A
//We need to tell the compiler about the type of this binary string
//Being a statically typed language, RUST must know the type of variables you are defining

fn main(){// Here we are trying to confuse the compiler
    let y = "10".parse().unwrap();
    println!("{ }", y);
}

fn main(){
    let y:u32 = "10".parse().unwrap();
    println!("{ }", y);
}




