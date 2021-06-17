use std::collections::HashMap;

fn main()
{

    let mut scores = HashMap::new();

    scores.insert(String::from("Red"),10);
    scores.insert(String::from("Blue"),30);

    let score = scores.get(&String::from("Blue"));

    if let Some(score) = score {
        println!("{:?}",score);
    }

    for (key, value) in &scores { 
        println!("{} : {}",key, value);

    }

    scores.entry(String::from("Yellow")).or_insert(100);
    println!("blue...{}",scores.entry(String::from("Blue")).or_insert(1000));
    println!("{:?}",scores);

}
