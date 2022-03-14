fn main() {
    println!("Hello, world!");

    some_function();
    some_fn_param(4);

    let statement = {
        let x = 30;
        x*five()
    };
    some_fn_param(statement);

    let x = 5;
    let y = 6;
    let result = add(x,y);
    some_fn_param(result);


    let x = 10;
    if x < 10 {
        println!("bigger");
    } else {
        println!("smaller");
    }

    let x = if y < 10 { 5 } else { 6 };
    println!("The value of x is: {}", x);


    let mut counter = 0;
    let result = loop {
        counter = counter + 1;
        if counter >= 10 {
            break counter;
        }
    };
    println!("The result is {}", result);


    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {}", element);
    }

}

fn some_function() {
    println!("some_function called")
}

fn some_fn_param(x: i32) {
    println!("Print: {}",x)
}

fn five() -> i32 {
    // this returns 5 without a return statment!
    5
}

fn add(a:i32, b:i32) -> i32 {
    return a + b;
}