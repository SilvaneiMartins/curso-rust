#[allow(dead_code)]
pub fn shadowing() {
    let x = 5;
    println!("O valor de x é: {}, {:p}", x, &x);

    let x = x + 1;
    println!("O valor de x é: {}, {:p}", x, &x);
    
    let x = x * 2;
    println!("O valor de x é: {}, {:p}", x, &x);

    println!("O valor de x é: {}", x);
}