pub fn learnings() {
    // L1
    //Immutable by default
    let var1: i32 = 5;
    println!("{var1}");
    //Mutable variable created using mut keyword
    let mut var2: i32 = 6;
    var2 += 5;
    println!("{var2}");

    // L2 Constants
    const V1: i32 = 5;
    println!("{V1}");

    // L3 Shadowing
     //Defined immutable int variable
    let v1 = 5;
    println!("{v1}");

    //Changed int to string
    let v1 = String::from("yes");
    println!("{v1}");

    //Defined immutable to mutable
    let mut v1 = 99;
    v1 += 10;
    println!("{v1}");

    // L4: Compound datatypes
    // Tuples
    let tup = ("String", 12, 'c', "String2");
    println!("{}, {}, {}, {}", tup.0, tup.1, tup.2, tup.3);

    // Arrays
    let arr = [10, 20, 55];
    println!("{}", arr[0]);

    // Array of 0s with 8 elemnts.
    let arr2 = [0; 8];
    println!("{}, {}", arr2[0], arr2[7]);

    for_loops();
}

fn for_loops() {
    let arr = [("String1", 11), ("String2", 12), ("String3", 13)];

    for elem in arr.iter() {
        print!("{} {}", elem.0, elem.1);
    }
}