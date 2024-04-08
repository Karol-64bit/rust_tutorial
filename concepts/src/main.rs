fn main() {

    // mutable - daje możliwość zmiany wartoścci zmiennej
    let mut x = 5;
    println!("The value of x is: {}", x);
    let x = 6;
    println!("The value of x is: {}", x);
    
    // constants - stała, nie można jej zmienić
    const MAX_VALUE: u32 = 1000;

    // shadow - można nadpisać zmienną, może mieć inny typ
    let y = 5;
    println!("The value of x is: {}", y);
    let y = "six";
    println!("The value of x is: {}", y);


    // Compound types
    let tup = ("one", 2);
    let (number1, number2) = tup;

    println!("{}", number1); 

    println!("{}", tup.1); 

    // array
    let error_codes = [200, 404, 500];
    let error = error_codes[2];

    println!("{}", error); 



    // function
    println!("{}", my_function(11, 22));


    //control flow
    let condition = true;
    let number = if condition { 5 } else { 6 };

    

}

fn my_function(x: i32, y: i32) -> i32 {
    println!("x: {}", x);
    println!("y: {}", y);
    // let sum = x + y;
    // return sum

    // alternative shorter way to return value
    x + y
}
