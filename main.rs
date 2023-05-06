fn main(){
    //The variable is defined as a binding
    let mut args: Skip<Args>  = std::env::args().skip(1); // Envoking the standard module "env" and the "args" function that we can call for command line args
    //args() is an iterator, i.e. types that allow iteration over them
    //skip(1) skips over the first element
    let key: String  = args.next().unwrap();//We need to make it mutable by using the "mut" argument\
    //In RUST, everything is Immutable by default...You have to ask RUST for making it Mutable. This makes RUST different from other languages
    //unwrap() is a great way to say that if the key is there, then GREAT...but, if it's not, then CRASH the program
    println!("The Key is {}", key);
    let value: String = args.next().unwrap();
    pritnln!("The key is '{}' and the value is '{}'", key, value);//Output should be "The key is 'hello' and the value is 'world'" if you use the command " cargo run -- hello world"
    //We can also show friendly error messages
    let key: String = args.next().expect("Key was not there");

    //Now we are going to store our key and value in the database
    //Our file looks like this
        // mykey\tab_character\myvalue\new_line\mykey2\tab_character\myvalue2

    let contents: String = format!("{} \t {} \n", key, value); 
    std::fs::write(path: "kv.db", contents).unwrap();//This unwarp handles the Error for us
    
    //Running the above command makes a kv.db file and writes "hello RAJ" to it when we give the commmand "cargo run -- hello RAJ"

    //Some Pattern matching down
    //match write_result{
    //    Ok(()) => {
      //      
      //  }
        //Err(e:) => {

        //}
    //}

}