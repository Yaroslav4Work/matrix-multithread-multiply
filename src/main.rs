use std::time::Instant;

use matrix_multihread_multiply::Matrix;

fn main() {
    let a = Matrix::new(vec![vec![1, 2, 2], vec![3, 1, 1]]).unwrap();
    let b = Matrix::new(vec![vec![4, 2], vec![3, 1], vec![1, 5]]).unwrap();

    println!("a size: {:?}", a.size());
    println!("b size: {:?}", b.size());

    println!(
        "Result for {} * {} is {:?}",
        a,
        b,
        Matrix::multiply(&a, &b).unwrap()
    );

    println!("Result for {} * {} is {:?}", a.clone(), b.clone(), a * b);

    let a = Matrix::generate(2, 3).unwrap();
    let b = Matrix::generate(3, 2).unwrap();

    println!(
        "Result for {} * {} is {:?}",
        a,
        b,
        Matrix::multiply(&a, &b).unwrap()
    );

    println!("Result for {} * {} is {:?}", a.clone(), b.clone(), a * b);

    let a = Matrix::generate(5, 7).unwrap();
    let b = Matrix::generate(7, 3).unwrap();

    println!(
        "Result for {} * {} is {:?}",
        a,
        b,
        Matrix::multiply(&a, &b).unwrap()
    );

    println!("Result for {} * {} is {:?}", a.clone(), b.clone(), a * b);

    let a = Matrix::generate(5, 18).unwrap();
    let b = Matrix::generate(18, 9).unwrap();

    println!(
        "Result for {} * {} is {:?}",
        a,
        b,
        Matrix::multiply(&a, &b).unwrap()
    );

    println!("Result for {} * {} is {:?}", a.clone(), b.clone(), a * b);

    let a = Matrix::generate(500, 700).unwrap();
    let b = Matrix::generate(700, 500).unwrap();

    let start = Instant::now();

    println!(
        "Result for {} * {} is {}. Time: {}",
        a,
        b,
        Matrix::multiply(&a, &b).unwrap(),
        start.elapsed().as_millis()
    );

    let start = Instant::now();

    println!(
        "Result for {} * {} is {}. Time: {}",
        a.clone(),
        b.clone(),
        a * b,
        start.elapsed().as_millis()
    );
}
