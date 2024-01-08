use std::io;

fn main() {
    println!("Введите число F1:");

    let mut first_num: f32 = loop {
        let mut first_num= String::new();

        io::stdin()
            .read_line(&mut first_num)
            .expect("Error");

        match first_num.trim().parse() {
            Ok(temp)  => break temp,
            Err(_) => println!("Некоректное число."),
        }
    };

    println!("Введите число F2:");

    let mut second_num: f32 = loop {
        let mut second_num= String::new();

        io::stdin()
            .read_line(&mut second_num)
            .expect("Error");

        match second_num.trim().parse() {
            Ok(temp)  => break temp,
            Err(_) => println!("Некоректное число."),
        }
    };

    println!("Укажите количество чисел:");

    let size: usize = loop {
        let mut size= String::new();

        io::stdin()
            .read_line(&mut size)
            .expect("Error");

        match size.trim().parse() {
            Ok(temp)  => break temp,
            Err(_) => println!("Некоректное число."),
        }
    };

    let mut i: usize = 0;


    println!("\n{}",first_num);
    println!("{}",second_num);

    while i <= size {
        let res: f32 = first_num + second_num;
        first_num = second_num;
        second_num = res;

        println!("{}",res);

        i += 1;
    }
}


