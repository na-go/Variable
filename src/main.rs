fn question_1(){
    let mut x:u32 = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

fn question_2(){
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is:{}", x);
}

fn question_3(){
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

fn main() {
    question_1();
    question_2();
    question_3();
}
