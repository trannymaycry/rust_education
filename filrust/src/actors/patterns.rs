// module shows concept of patterns
fn examples() {
    let favorite_things: Option<&str> = None;
    let end_of_day = true;
    let age: Result<u8, _> = "27".parse();
    // if let construction pattern
    if let Some(thing) = favorite_things {
        println!("We going to {} today", thing);
    } else if end_of_day {
        if let Ok(age) = age {
            if age < 25 {
                println!("Oh, hack this box today and find useful courses");
            }
        } else {
            println!("Smoke weed hood");
        }
    } else {
        println!("I guess you work for tear yet. You cannot may named gucci")
    }


    let mut my_earnings = Vec::new();
    my_earnings.push(300);
    my_earnings.push(2000);
    my_earnings.push(500);
    my_earnings.push(850);
    // 'while' construction pattern
    while let Some(dollar) = my_earnings.pop() {
        println!("Conversion from dollars to rouble earning: {} RUB.", dollar * 60);
    }


    let numbers_in_letters = vec!["zero", "one", "two", "three"];
    // for pattern example
    for (index, string_number) in numbers_in_letters.iter().enumerate() {
        println!("{} - {}", index, string_number);
    }


    // structs pattern
    struct Shotgun {
        easy: i32,
        gang: i32,
    }
    let glock = (10, 40);
    let Shotgun { easy, gang } = glock;
    match glock {
        // NB! here we bound to real_gang value (using @) with a check matching in particular range (from 30 to 60 at this case)
        Shotgun { easy, gang: real_gang @ 30..=60, } => println!("Oh you are actually going to account for this niggers! Enough accurate for {}", real_gang),
        Shotgun { easy, gang: 20 } => println!("There are enough bullets for everyone"),
        Shotgun { easy: 0, gang: 0 } => println!("Don't you dare drag ur kick ass to those niggers!"),
        Shotgun { easy, gang } => println!("Oh, you have {} bullets in light and {} bullets in extended shotgun", easy, gang),
    }


    // match patterns ability and enum patterns
    let rims_size = 21;
    let on_the_hood = true;
    match rims_size {
        18..=25 if on_the_hood => println!("Oh shiiiiit"),
        16 | 17 if !on_the_hood => println!("If are u my dad that is good"),
        _ => println!("You have to run away from the hood"),
    }
    enum Vehicle {
        None,
        WeedWagon { weight: i32, stain: String },
        PussyScream(bool),
        BrosWheels(String, String, String, String),
    }
    let cadillac = Vehicle::WeedWagon { weight: 420, stain: String::from("Skunk") };
    match cadillac {
        Vehicle::None => {
            println!("lox");
        }
        Vehicle::WeedWagon { weight, stain } => {
            if weight > 100 {
                println!("{} gram shit you have, step on the gas, motherfucker, and fuck with the {} right now!", weight, stain);
            }
        }
        Vehicle::PussyScream(man) if man => {
            println!("Okey no bros today, will engage this pussy man")
        }
        // here we pass first two bros (using ..) and third (using _) and keep just last for using in code below
        Vehicle::BrosWheels(.., _, right_hands_bro) => {
            println!("{}, pass me the blunt man", right_hands_bro);
        }
        _ => println!("Boy get your own wheels before eat those shit"),
    }
    // 'let' and 'function' pattern examples
    let (heaven, hell) = (String::from("tati"), String::from("party"));
    let tuple_prefer_and_not = (String::from("die"), String::from("existence"));
    function_pattern((heaven, hell));
    function_pattern(tuple_prefer_and_not);
    let ((one_tuple_val, and_two), Shotgun { easy, gang }) = (("write first primitive", "and write second"), Shotgun { easy: 5, gang: 10 });
    println!("In first we store '{}' and '{}'; further we store {} and {} from Shotgun to values and did it at once string", one_tuple_val, and_two, easy, gang);
    let _bound = String::from("In is bound but compiler won't warn about it if will not use");

    // This code won't work bcuz _bound get ownership Some value from bound and don't use it in it's body
    // let bound = Some(String::from("Test"));
    // if let Some(_bound) = bound {
    //     println!("found a string");
    // }
    // println!("{:?}", bound);
}

// function declaration pattern
fn function_pattern((heaven, hell): (String, String)) {
    println!("Prefer - {}, avoid - {}", heaven, hell);
}
