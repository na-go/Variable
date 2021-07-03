fn print_function(x: i32){
    println!("The value of x is: {}", x);
}

fn variable_and_mutable_1(){
    let mut x:u32 = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

fn variable_and_mutable_2(){
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is:{}", x);
}

fn variable_and_mutable_3(){
    /*
    ## Bad example ##
    ## we’re not allowed to mutate a variable’s type by "let mut spaces"## 
    
    let mut spaces = "   ";
    spaces = spaces.len();
    */
    let spaces = "   ";
    println!("spaces:{}", spaces);
    let spaces = spaces.len();
    println!("spaces:{}",spaces);
}

fn data_type_1(){
    let guess: u32 = "42".parse().expect("Not a number"); // int type 32
    println!("guess:{}",guess);
}

fn data_type_2(){
    let x = 2.0;        // float type 64
    let y: f32 = 3.0;   // float type 32
    println!("x:{}, y:{}", x,y);
}

fn data_type_3(){
    // addition
    let sum:u32 = 5 + 10;

    // subtraction
    let difference:f64 = 95.5 - 4.3;

    // multiplication
    let product:i32 = 4 * 30;

    // division
    let quotient:f64 = 56.7 / 32.2;

    // remainder
    let remainder:u32 = 43 % 5;
    println!("sum:{},differ:{},product:{},quotient:{},remainder:{}", sum, difference, product, quotient, remainder);
}

fn data_type_4(){
    let t = true;
    let f: bool = false; // bool type

    println!("t:{}, f:{}", t, f);
}

fn data_type_5(){
    let c = "z";
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("str:{}, special_str:{}, pictograph:{}", c, z, heart_eyed_cat);
}

fn data_type_6(){
    /*
    ## Bad example ##
    ## we’re not allowed to choose a tuple type variable by "println!" ## 
    
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tuple:{}", tup);
    */

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x,y,z) = tup;
    println!("tuple:({},{},{})", x, y, z);
}

fn data_type_7(){
    let _x: (i32, f64, u8) = (500, 6.4, 1); // underscore is required if not used in println!
    let five_hundred = _x.0;
    let six_point_four = _x.1;
    let one = _x.2;
    println!("x1:{}, x2:{}, x3:{}", five_hundred, six_point_four, one);
}

fn data_type_8(){
    /*
    ## Bad example ##
    ## Apparently "array" cannot be output as it is. ##
    
    let a = [1,2,3,4,5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
            "August", "September", "October", "November", "December"];
    println!("a:{}, months:{}", a, months);

    */
    let a = [1,2,3,4,5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
            "August", "September", "October", "November", "December"];
    println!("a:{}, months:{}", a[0], months[0]);
}

fn data_type_9(){
    // Can compile, but gets angry at runtime.

    /*
    let a = [1,2,3,4,5];
    let index = 10;

    let element = a[index];

    println!("The value of element is: {}", element);
    */
}

fn function_1(x: i32){
    println!("The value of x is:{}", x);
}

fn function_2(x: i32, y: i32){
    println!("The value of x is:{}", x);
    println!("The value of y is:{}", y);
}

fn function_3(){
    /*
    ## Bad example ##
    ## lines doesn't return value. ##
    let x = (let y = 6);
    */

    let _x = 5;

    let y = {
        let _x = 3;
        _x + 1       // Expressions do not include ending semicolons.
    };

    println!("The value of y is: {}", y);

}

fn function_4() -> i32{
    5
}

fn function_5(x:i32) -> i32{
    x + 1
}

fn control_flow_1(){
    let number:i32 = 3;
    if number < 5 {
        println!("condition was true.");
    } else {
        println!("condition was false.");

    }
}

fn control_flow_2(){
    let number = 3;

    if number != 0 {
        println!("number was something other than 0.");
    }
}

fn control_flow_3(){
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4,3, or 2");
    }
}

fn control_flow_4() {
    /*
    ## Bad example ##
    ## The if and else arms have value types that are incompatible.        ##

    let condition = true;

    let number = if condition {
        5
    } else {
        "six"
    };
    
    println!("The value of number is: {}", number);

    */
    let condition = true;
    let number = if condition {
        100
    } else {
        44
    };

    println!("The value of number is: {}", number)
}

fn control_flow_5() {
    /*
    ## "const" must be upper case.
    */
    const BREAK_NUMBER:u8 = 2;
    let mut i:u8 = 0; // iterable needs to be mutable because we want to increase it.
    loop{
        println!("again!");
        i = i + 1;
        if i > BREAK_NUMBER {
            break;
        }
    }
}

fn control_flow_6(){
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");
}

fn control_flow_7() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index = index + 1;
    }
}

fn control_flow_8() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn practice_problem_1(celsius:bool, temperatures:f64){
    // Convert temperatures between Fahrenheit and Celsius.
    if celsius {
        println!("Fahrenheit is {}.",temperatures * 1.8 + 32.0);
    } else {
        println!("celsius is {}.",(temperatures - 32.0) / 1.8);
    }
}

fn practice_problem_2(n_term:u32) -> u32{
    // Generate the nth Fibonacci number.
    
    if n_term == 0 {
        1
    } else if n_term == 1{
        1
    } else {
        practice_problem_2(n_term - 1) + practice_problem_2(n_term - 2)
    }
}

fn main() {
    variable_and_mutable_1();
    variable_and_mutable_2();
    variable_and_mutable_3();
    data_type_1();
    data_type_2();
    data_type_3();
    data_type_4();
    data_type_5();
    data_type_6();
    data_type_7();
    data_type_8();
    data_type_9();
    function_1(5);
    function_2(32, 14);
    function_3();
    let x = function_4();
    print_function(x);
    let x = function_5(5);
    print_function(x);
    control_flow_1();
    control_flow_2();
    control_flow_3();
    control_flow_4();
    control_flow_5();
    control_flow_6();
    control_flow_7();
    control_flow_8();
    practice_problem_1(false, 212.0);
    let x:u32 = practice_problem_2(6);
    println!("fibonacci number is: {}",x);
}
