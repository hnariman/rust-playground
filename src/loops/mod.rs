pub fn for_loop(limit: i8) {
    for index in 0..limit {
        println!(" for index of {}", index);
    }
    // index will get cleaned & we can even reuse names
    // for inclusive range we can use = operator
    for index in 0..=limit {
        println!(" inclusive index of {}", index);
    }
}

pub fn while_loop(limit: i8) {
    println!("Classic While loop:");
    let mut counter: i8 = 0;
    while counter < limit {
        println!("{}", counter);
        // we don't have ++ or -- either as prefix || postfix in rust
        counter += 1;
    }
}

pub fn just_loop(limit: i8) {
    println!("Using Loop of rust");
    let mut counter: i8 = 0;
    let result = loop {
        counter += 1;
        println! {"{}", counter};

        if counter == limit {
            break;
        }
    };
    println!("And we break");
}

pub fn labeled_loop(limit: i8) {
    let mut count = limit;
    'decay_counter: loop {
        println!("cycles remaining: {}", count);
        count -= 1;
        if count == 0 {
            break;
        }
    }
}

pub fn for_array_iter(limit: usize) {
    // since array length is defined at compile time
    // we need to use vector for dynamic length setting of iterable
    let array = vec![0; limit];
    for item in array.iter() {
        // println!("index:{} item: {}", index, item);
        println!("item: {}", item);
    }
}

pub fn reverse_for(limit: i8) {
    for count in (1..=limit).rev() {
        println!("{} potatoes left", count);
    }
}
