// fizz buzz

fn main() {
    fizz_buzz();
    fizz_buzz2();
    fizz_buzz3();
}


pub fn fizz_buzz() {
    println!("---------------");
    println!("Fizz Buzz: fallthru plus concat");
    println!("---------------");
    for i in 1..100 {
        let multiple_of_3 = i % 3 == 0;
        let multiple_of_5 = i % 5 == 0;
        if !multiple_of_3 && !multiple_of_5 {
            println!("{}", i);
            continue;
        }
        let mut fizz: &str = "";
        let mut buzz: &str = "";
        if multiple_of_3 {
            fizz = "Fizz";
        }
        if multiple_of_5 {
            buzz = "Buzz";
        }
        println!("{}{}", fizz, buzz);
    }
}


pub fn fizz_buzz2() {
    println!("---------------");
    println!("Fizz Buzz 2: simple if-else");
    println!("---------------");
    for i in 1..100 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }
}


pub fn fizz_buzz3() {
    println!("---------------");
    println!("Fizz Buzz 3: match");
    println!("---------------");
    for i in 1..100 {
        let multiple_of_3 = i % 3 == 0;
        let multiple_of_5 = i % 5 == 0;
        let fizzbuzz = multiple_of_3 && multiple_of_5;
        let answer: String;
        match i {
            _ if fizzbuzz => {
                answer = "FizzBuzz".to_string();
            },
            _ if multiple_of_3 => {
                answer = "Fizz".to_string();
            },
            _ if multiple_of_5 => {
                answer = "Buzz".to_string();
            }
            _ => answer = i.to_string(),
        }
        println!("{}", &answer);
    }
}
