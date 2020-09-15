fn main() {
    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();

    // notice a string slice is used
    let mut s = String::from("foo");
    s.push_str("bar");

    // the push_str method takes a string slice because we
    // don't want it to take ownership in case we wish to
    // use s2 later
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    // the push method takes a single character as a parameter
    // and adds it to the string which called it
    let mut s = String::from("lo");
    s.push('l');    // s contains "lol"

    // string concatenation with the + operator
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;  // note s1 has been moved here and can no longer be used
    // s1 is no longer valid because + uses the add method, which takes ownership
    // of s1, and then adds the reference of s2 to return the new string s3.
    // This is more efficient than copying the values of s1 and s2 and then doing
    // the concatenation.

    // Concatenating multiple strings get a little wierd
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    // For more complicated string concatenation, we use the format! macro
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    // Accessing individual characters in rust isn't so easy (in fact, it's impossible)

    let s1 = String::from("hello");
    let h = s1[0];

    // This is not allowed because Rust stores strings as vectors of unsigned 8 bit
    // integers
    let hello = "Здравствуйте"; // the first letter of this string is "Ze" (cyrillic)
    let answer = &hello[0];     // though the string appears to be 12 characters long
                                // it is stored as 24 bytes because each letter requires
                                // two u8 bytes to be UTF-8 encoded.
                                // If this did compile and return, you would get 208, which
                                // isn't very useful

    // Bytes, Scalar Values and Grapheme Clusters
    // the Hindi word “नमस्ते” is represented in three ways in rust
    // the Vec<u8>: [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
    // The unicode scalar values: ['न', 'म', 'स', '्', 'त', 'े']
    // and the grapheme clusters: ["न", "म", "स्", "ते"]
    // Each program can use the interpretation it needs to work with that word, which is why
    // accessing individual characters from a string will error out.

    // slicing strings can be done, but you need to grab a range
    let hello = "Здравствуйте";

    let s = &hello[0..4];   // this would make s equal to "Зд"

    // however, trying to grab [0..1] would cause the rust to freak out at run time

    // If you need to access elements in a string, you can use the chars method
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    /* this would print:
    न
    म
    स
     ्
    त
     े
    */

    // similarly you can print out the raw bytes using the bytes function
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
    /*
    224
    164
    --snip--
    165
    135
    */
}