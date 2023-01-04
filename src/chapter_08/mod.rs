use std::collections::HashMap;

pub fn run() {
    vectors();
    strings();
    hash_maps();

    println!("Exercises");
    let mut numbers = vec![1, 5, 5, 2, 3];
    let med = median(&mut numbers);
    println!("Median {:?}", med);
    let most = most_frequent(&numbers);
    println!("Most {}", most);
}

fn median(numbers: &mut [u8]) -> u8 {
    numbers.sort();
    let mid_index = numbers.len() / 2;
    numbers[mid_index]
}

fn most_frequent(numbers: &[u8]) -> u8 {
    let mut max: u8 = 0;
    let mut num_count = HashMap::new();
    for num in numbers {
        let count = num_count.entry(format!("{}", num)).or_insert(0);
        *count += 1;
        if &max < count {
            max = *num
        }
    }
    max
}

#[allow(unused_variables)]
fn hash_maps() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 50);
    scores.insert(String::from("Red"), 50);

    let team_name = String::from("Blue");
    let mut team_score = 0;
    if let Some(score) = scores.get(&team_name).copied() {
        team_score = score;
    };
    // let score = scores.get(&team_name).copied().unwrap_or(0); // equivalent to above
    println!("Team {}'s Score is {}", team_name, team_score);

    let field_name = String::from("Favorite Color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // println!("{}", field_name); // borrow of moved value. owner is now map
    // println!("{}", field_value); // borrow of moved value.
    //
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 50); // 10 is replaced
    println!("{:?}", scores);

    scores.entry(String::from("Yellow")).or_insert(30); // will be inserted
    scores.entry(String::from("Blue")).or_insert(20); // will not be inserted
    println!("{:?}", scores);

    let text = "Hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}

#[allow(unused_variables)]
fn strings() {
    // the following are equivalent
    let lit: String = "Hello World".to_string();
    let lit2: String = String::from("Hello World");

    let mut s_foo = String::from("Foo");
    let s2 = "bar";
    s_foo.push_str(s2);
    println!("{}", s_foo);

    let mut s_lo = String::from("lo");
    s_lo.push('l');
    println!("{}", s_lo);

    let s1 = "Hello".to_string();
    let s2 = "World!".to_string();
    let s3 = s1 + &s2;
    println!("{}", s3);
    // println!("{}", s1); Invalid as S1 was moved during concat

    let tic = String::from("Tic");
    let tac = String::from("Tac");
    let toe = String::from("Toe");
    let s = format!("{}-{}-{}", tic, tac, toe);
    // format rather than
    // s = tic + "-" + tac + "-" + toe;
    println!("{}", s);

    // let h = lit[0]; // Cannot be index by integer
    let hello = String::from("Здравствуйте");
    println!(
        "Здравствуйте is {} long due to characters taking multiple bytes",
        hello.len()
    );
    // valid unicode scalar values may be made up of more than one byte

    // loops twice
    for c in "Зд".chars() {
        println!("iterated character {}", c)
    }

    // loops 4
    for b in "Зд".bytes() {
        println!("iterated byte {}", b)
    }
}

#[allow(unused_variables)]
#[allow(clippy::vec_init_then_push)]
fn vectors() {
    let v: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3, 4, 5]; // macro with initial types

    let mut inferred_type_vector = Vec::new();
    inferred_type_vector.push(4);
    inferred_type_vector.push(5);

    let third: &i32 = &v2[2];
    println!("The third element is {}", third);

    let opt_third: Option<&i32> = v2.get(2);
    if let Some(opt_third) = opt_third {
        println!("The third element via get is {}", opt_third);
    }
    //let ninth = &v2[9]; // panic - index out of bounds
    let opt_ninth: Option<&i32> = v2.get(9);
    if let Some(opt_ninth) = opt_ninth {
        println!("The Out of Range element via get is {}", opt_ninth); // this doesn't run
    }

    // borrrow checker
    let borrow_vec = vec![1, 2, 3, 4, 5]; // macro with initial types
    let borrowed_element = &borrow_vec[0];
    // borrow_vec.push(6); // this will fail to compile "cannot borrow as mutable because it is also borrowed as immutable"
    // this does not work because vectors store all alements in memory contiguously so any push may relocat all elements
    println!("The borrowed element is {}", borrowed_element);

    // Iterating
    let iter_vec = vec![100, 32, 57];

    for i in &iter_vec {
        println!("Iterating over a reference to {}", i);
    }

    let mut mut_iter_vec = vec![100, 32, 57];

    for i in &mut mut_iter_vec {
        *i += 50;
        println!("Iterating over a dereferenced variable {}", i);
    }

    for i in mut_iter_vec {
        println!("has dereferenced variable remain changed {}", i); // yes
    }

    // storing multiple types
    let multiple_types_vec = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_most_frequest() {
        let numbers = vec![1, 1, 5];
        let most = most_frequent(&numbers);
        assert_eq!(most, 1);
    }

    #[test]
    fn test_median() {
        let mut numbers = vec![1, 2, 3];
        let actual= median(&mut numbers);
        assert_eq!(actual, 2);
    }

    #[test]
    fn test_median_single() {
        let mut numbers = vec![1];
        let actual= median(&mut numbers);
        assert_eq!(actual, 1);
    }

}
