use vec_utilities::generation::arange;

#[test]
fn test_arange(){
    let x = arange(2, 1000, 2);
    println!("x: {:?}",x)
}