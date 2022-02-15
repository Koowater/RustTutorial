extern crate rand;
// 외부 의존 crate가 있음을 알린다.

use std::io;
use std::cmp::Ordering;
use rand::Rng;
// trait를 위한 코드. 10장에서 자세히 배울 예정.

fn main() {
    println!("Guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1, 101);
    // rand::thread_rng():
    //      os가 seed를 정하고 현재 스레드에서만 사용되는 특별한 정수생성기를 반환한다.

    // println!("The secret number is: {}", secret_number);


    loop {

        println!("Please input your guess.");

        let mut guess = String::new();
        // mut을 이용하여 가변변수를 선언하였다.
        // 새로운 빈 String 인스턴스와 연결된 가변 변수를 생성하였다.

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
            // 실행의 결과값으로 io::Result를 반환한다. 
            // Result의 variants는 Ok와 Err이다.
            // .expect는io::Result의 메소드 중 하나이다.

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            },                
        };
        // expect 메소드 호출을 match 표현식으로 바꾸는 것은
        // 에러 발생 시 종료에서 처리로 바꾸는 일반적인 방법이다.


        // let guess: u32 = guess.trim().parse()
        //     .expect("Please type a number!");
            // Shadowing이다. 3장에서 자세히 배운다.
            // trim: String 인스턴스의 처음과 끝 부분의 빈칸을 제거한다.
            //      String을 입력 받았을 때, 끝에 있는 개행 문자를 지워야 한다.
            // parse: 문자열을 숫자형으로 파싱한다.
        
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!");
                break;
            }
        }
    }
}
