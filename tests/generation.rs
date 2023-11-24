use vec_utilities::generation::arange;

#[test]
fn test_arange() {
    let x = arange(-1.0, 10.0, 0.25).unwrap();
    println!("x: {:?}", x);

    println!("{}", x.len());
}
