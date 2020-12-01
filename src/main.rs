use std::collections::HashMap;

fn main() {
    let v = roman_to_int("MMXIII");
    println!("{:?}", v);

    for i in 0..100 {
        println!("{}", fizzbuzz(i))
    }
}

///Transform a Roman number into an integer
///Progress by pair through the string and
/// -ONLY first value if second do not exists
/// -SUM if both values are equal
/// -SUBSTRACT if first value below second
/// -IGNORE the second value if first > second
fn roman_to_int(s:&str)->Option<u32> {
    let mut result = 0;

    let values: HashMap<char, u32> =[
                    ('I', 1),
                    ('V', 5),
                    ('X', 10),
                    ('L', 50),
                    ('C', 100),
                    ('D', 500),
                    ('M', 1000),
                ].iter().cloned().collect();

    let mut iter = s.chars().filter(|k| values.contains_key(k)).peekable();
    
    while iter.peek().is_some() {

        result += match (iter.next(), iter.peek()) {
            (Some(first), None) => *values.get(&first).unwrap_or(&0),
            (Some(first), Some(second)) => match (values.get(&first), values.get(second)) {
                (Some(first), Some(second)) if first == second => {iter.next(); first + second},
                (Some(first), Some(second)) if first < second => {iter.next(); second - first},
                (Some(first), _) => *first,
                _ => 0
            },
            _ => 0
        };

    }

    Some(result)

}

/// Transform an interger into a Roman number
fn int_to_roman(input:u32)->String {

    let library = ["I", "V", "X", "L", "C", "D", "M"];

    let mut vec_value:Vec<String> = Vec::new();
    let input_as_string = input.to_string();
    let length = input_as_string.len();


    for (index, value) in input_as_string.chars().enumerate() {
        match (index==0, length>3) {
            (false, true) if index<length-3 => vec_value[0].push(value),
            _ => vec_value.push(value.to_string())
        }
    }

    vec_value.reverse();

    let mut output = String::new();

    for (index, rank_occ) in vec_value.iter().enumerate() {
        let library_index = index * 2;

        if library_index >= library.len() {continue;}

        match rank_occ.parse::<usize>() {
            Ok(r) => 
                if r<4 || r>9 {
                    output.push_str(&library[library_index].repeat(r as usize));
                } else if r==4 {
                    output.push_str(&library[library_index+1]);
                    output.push_str(&library[library_index]);                 
                } else if r<9 {
                    output.push_str(&library[library_index].repeat(r as usize));
                    output.push_str(&library[library_index+1]);                    
                } else if r==9 {
                    output.push_str(&library[library_index+2]);
                    output.push_str(&library[library_index]);                    
                }                    
            ,
            Err(_) => continue
        }        

    }
    
    output.chars().rev().collect()
}

fn fizzbuzz(n:u8) -> String {
    match (n%3, n%5) {
        (0, 0) => "fizzbuzz".to_string(),
        (0, _) => "fizz".to_string(),
        (_, 0) => "buzz".to_string(),
        _ => n.to_string()
    }
}

#[test]
fn test_int_to_roman() {
    assert_eq!(int_to_roman(0), "".to_string());
    assert_eq!(int_to_roman(2), "II".to_string());
    assert_eq!(int_to_roman(20), "XX".to_string());
    assert_eq!(int_to_roman(4), "IV".to_string());
    assert_eq!(int_to_roman(49), "XLIX".to_string());
    assert_eq!(int_to_roman(999), "CMXCIX".to_string());
    assert_eq!(int_to_roman(2222), "MMCCXXII".to_string()); 
    assert_eq!(int_to_roman(12499), "MMMMMMMMMMMMCDXCIX".to_string());
}

#[test]
fn test_roman_to_int() {
    assert_eq!(roman_to_int(""), Some(0));
    assert_eq!(roman_to_int("I"), Some(1));
    assert_eq!(roman_to_int("II"), Some(2));
    assert_eq!(roman_to_int("III"), Some(3));
    assert_eq!(roman_to_int("IV"), Some(4));
    assert_eq!(roman_to_int("XIV"), Some(14));
    assert_eq!(roman_to_int("XZX"), Some(20));
    assert_eq!(roman_to_int("XLIX"), Some(49));
    assert_eq!(roman_to_int("CDXLVIII"), Some(448));
    assert_eq!(roman_to_int("MMMDCCLXVII"), Some(3767));
}

#[test]
fn test_fizzbuzz() {
    assert_eq!(fizzbuzz(2), "2");
    assert_eq!(fizzbuzz(3), "fizz");
    assert_eq!(fizzbuzz(5), "buzz");
    assert_eq!(fizzbuzz(15), "fizzbuzz");
}