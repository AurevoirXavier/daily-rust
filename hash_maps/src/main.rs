
use std::collections::HashMap;

fn main() {
    {
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
    }

    {
        let teams = vec![String::from("Blue"), String::from("Yellow")];
        let inital_scores = vec![10, 50];

        // let scores: HashMap<&String, &i32> = teams.iter().zip(inital_scores.iter()).collect();
        let scores: HashMap<_, _> = teams.iter().zip(inital_scores.iter()).collect();
    }

    {
        let field_name = String::from("Favorite color");
        let field_value = String::from("Blue");

        let mut map = HashMap::new();

        map.insert(field_name, field_value);
    }

    {
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        let team_name = String::from("Blue");
        let score = scores.get(&team_name);
    }

    {
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        // random order
        for (k, v) in &scores {
            println!("{}: {}", k, v);
        }
    }

    {
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Blue"), 25);

        println!("{:#?}", scores);
    }

    {
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);

        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(50);

        println!("{:#?}", scores);
    }

    {
        let text = "hello world wonderful world";

        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);

            *count += 1;
        }

        println!("{:#?}", map);
    }
}
