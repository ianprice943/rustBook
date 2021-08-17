fn main() {
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter(); // not useful on its own, as iterators are lazy. Something has to consume this!

    for val in v1_iter {
        println!("Got: {}", val);
    }

    // methods that produce other iterators
    let v2: Vec<i32> = vec![1,2,3];
    //v2.iter().map(|x| x + 1); // leads to a compiler warning since nothing "consumes" the data generated from map()
    let v3: Vec<_> = v2.iter().map(|x| x + 1).collect();

    assert_eq!(v3, vec![2,3,4]);
}
