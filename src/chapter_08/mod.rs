#[allow(unused_variables)]
#[allow(clippy::vec_init_then_push)]
pub fn run() {
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
    let mut borrow_vec = vec![1, 2, 3, 4, 5]; // macro with initial types
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
