// struct Test {
//     a: i8,
//     b: f32,
// }
// let t1 = Test { a: 1, b: 2.2 };
// println!("t1: {:#?}", t1);

pub fn cheat_sheat() {
    println!("---------------------------------------------------------------------------");
    println!("|||||||||| Tuple");
    println!("---------------------------------------------------------------------------");

    let tuple: (&str, bool, i32) = ("This is string", false, 10_000);
    println!("tuple 0: {}", tuple.0);
    println!("tuple 1: {}", tuple.1);
    println!("tuple 2: {}", tuple.2);

    println!("---------------------------------------------------------------------------");
    println!("|||||||||| Loops");
    println!("---------------------------------------------------------------------------");

    let mut counter = 0;
    loop {
        println!("Loop: {}", counter);
        counter += 1;

        if counter == 5 {
            println!("Counter: {}", counter);
            break;
        }
    }

    println!("----------------------------");

    let mut counter = 5;
    while counter != 0 {
        println!("While: {}", counter);
        counter -= 1;
    }

    println!("----------------------------");

    let arr = [1, 2, 4, 5];
    for i in arr {
        println!("Arr loop: {}", i);
    }
    for i in arr.iter() {
        println!("Arr loop + iter(): {}", i);
    }

    println!("----------------------------");

    for i in 1..8 {
        println!("Arr loop: {}", i);
    }

    println!("---------------------------------------------------------------------------");
    println!("|||||||||| Element comparison");
    println!("---------------------------------------------------------------------------");

    let a: &str = "true";
    let b: bool = false;
    println!("variables_a_and_b: {}, {}", a, b);

    println!("---------------------------------------------------------------------------");
    println!("|||||||||| Statement");
    println!("---------------------------------------------------------------------------");

    let this_is_statement: i32 = 333;
    println!("this_is_statement: {}", this_is_statement);

    println!("---------------------------------------------------------------------------");
    println!("|||||||||| Statement with Expresion");
    println!("---------------------------------------------------------------------------");

    let return_number = {
        let x: i32 = 3;
        x + 1 // Don't add ';' because then it does not return value, Expresion
    };
    println!("return_number: {}", return_number);

    println!("---------------------------------------------------------------------------");
    println!("|||||||||| Return from Function is Expresion");
    println!("---------------------------------------------------------------------------");

    println!("return_from_function: {}", return_from_function(1));
    println!("return_from_function: {}", return_from_function(2));
    fn return_from_function(val: i32) -> i32 {
        if val == 2 {
            return 4; // Expresion
        };
        // * 'val' or 'return val;'
        val // Expresion
    }

    println!("---------------------------------------------------------------------------");
    println!("|||||||||| Ownership and Borrowing");
    println!("---------------------------------------------------------------------------");

    let owner_1: String = String::from("Hello");
    println!("owner_1: {}", owner_1);
    let owner_2: String = owner_1;
    let owner_3: String = owner_2.clone();
    // println!("owner_1: {}", owner_1); // ! this will fail because owner has been transfered to 'owner_2'
    // println!("owner_2: {}", owner_1.clone()); // ! this will fail because owner has been transfered to 'owner_2'
    println!("owner_2: {}", owner_2);
    println!("owner_3: {}", owner_3); // * this works because i cloned from the owner that has the value

    println!("---------------------------------------------------------------------------");
    println!("|||||||||| Debug print");
    println!("---------------------------------------------------------------------------");

    let val: Vec<i32> = vec![1, 2, 3];
    println!("debug print: {:?}", val); // * will use this 'std::fmt::Debug' instead of 'std::fmt::Display'

    println!("---------------------------------------------------------------------------");
    println!("|||||||||| Raw string literals");
    println!("---------------------------------------------------------------------------");

    let json_string: &str = r#"{
        "name": "John",
        "number": ["one", "two"]
    }"#;
    println!("json_string: {}", json_string);
    println!("json_string :? : {:?}", json_string);

    println!("---------------------------------------------------------------------------");
    println!("|||||||||| Reference");
    println!("---------------------------------------------------------------------------");

    // Stored in Stack
    let mut t1 = "Test 1";
    let t2 = t1;
    println!("t2: {}", t2);
    println!("t1: {}", t1);
    t1 = "Test 1 Changed";
    println!("t1: {}", t1);
    println!("t2: {}", t2);

    // Stored in Heap
    let mut t3 = vec!["Test 2"];
    let t4 = t3.clone();
    println!("t3: {:?}", t3);
    println!("t4: {:?}", t4);
    t3 = vec!["Test 2 Changed"];
    println!("t3: {:?}", t3);
    println!("t4: {:?}", t4);

    println!("---------------------------------------------------------------------------");

    let example_str: &str = "String: &str";
    let example_string: String = String::from("String: String");
    let example_from_str: String = example_str.to_string();
    let example_from_str2: String = "Hard coded string".to_string();
    println!("example_str: {:?}", example_str);
    println!("example_string: {:?}", example_string);
    println!("example_from_str: {:?}", example_from_str);
    println!("example_from_str2: {:?}", example_from_str2);
}
