use std::collections::HashMap;


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

    //let contents: String = format!("{} \t {} \n", key, value); 
    //std::fs::write(path: "kv.db", contents).unwrap();//This unwarp handles the Error for us
    
    //Running the above command makes a kv.db file and writes "hello RAJ" to it when we give the commmand "cargo run -- hello RAJ"

    let database = Database::new().expect("Creating db failed");
    database.insert(key, value);

    //Some Pattern matching down
    //match write_result{
    //    Ok(()) => {
      //      
      //  }
        //Err(e:) => {

        //}
    //}

    //Calling a database function new defined below:
    let database: Database = Database::new().expect("Creating DB failed");
    database.insert(key.to_uppercase(), value);
    database.insert(key, value);
}

//Now we are going to make abstractions around our code

//There are no classes, objects in RUST... we only have structs and these Structs sit on the "Stack"
struct Database {
    map: HashMap<String, String>,  
}

//impl is the implementation we want in the previously defined Struct Database
impl Database {
    fn new() -> Result<Database, std::io::Error> {
        // Read the kv.db file and parse the String and populate the map


        let mut map: = HashMap::new();
        let contents: String = match std::fs::read_to_string(path: "kv.sb") {
            Ok(c: String) => c,
            Err(error: Error) => {
                return Result::Err(error);
            }
        };
//The above code can also be written as 
        // let contents: String = std::fs::read_to_string(path: "kv.db")?;
//Notice the Question mark at the end       
        
        for line: &str in contents.lines(){
            let mut chunks: = line.splitn(n, '\t');
            let key: = chunks.next().expect("No Key !");
            let value: = chunks.expect("No Value");
            map.insert(key.to_owned(), value.to_owned());
        }
        
        Ok(Database {map: map})
    }

    // the insert below is a Method which is different from a function
    fn insert(mut self, key: String, value: String){
        self.map.insert(key, value);
    }
}

