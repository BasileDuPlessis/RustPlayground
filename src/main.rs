use std::{collections::HashMap};

fn main() {

    println!("{:?}", roman_to_arabic("MMXIII"));

    println!("{:?}", arabic_to_roman(22));

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
fn roman_to_arabic(s:&str)->Option<u32> {
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
fn arabic_to_roman(input:usize)->String {

    let mut output = String::new();

    let translate = |u:usize, a:[&str; 3]| {
        match (u==4, u==9, u>=5) {
            (true, _, _) => a[0].to_owned() + a[1],
            (_, true, _) => a[0].to_owned() + a[2],
            (_, _, true) => a[1].to_owned() + &a[0].repeat(u-5),
            _ => a[0].repeat(u)
        }
    };


    if input>=1000 {
        output = "M".repeat((input as f32/1000_f32).floor() as usize);
    }
    if input>=100 {
        output = format!("{}{}", output,
            translate(((input as f32/100_f32).floor() as usize) % 10, ["C", "D", "M"])
        );
    }
    if input>=10 {
        output = format!("{}{}", output,
            translate(((input as f32/10_f32).floor() as usize) % 10, ["X", "L", "C"])
        );
    }
    if input>0 {
        output = format!("{}{}", output,
            translate(input % 10, ["I", "V", "X"])
        );
    } else {
        output = "".to_owned();
    }
   
    output
}

fn fizzbuzz(n:u8) -> String {
    match (n%3, n%5) {
        (0, 0) => "fizzbuzz".to_owned(),
        (0, _) => "fizz".to_owned(),
        (_, 0) => "buzz".to_owned(),
        _ => n.to_string()
    }
}




#[test]
fn test_arabic_to_roman() {
    assert_eq!(arabic_to_roman(0), "");
    assert_eq!(arabic_to_roman(2), "II");    
    assert_eq!(arabic_to_roman(4), "IV");
    assert_eq!(arabic_to_roman(5), "V");
    assert_eq!(arabic_to_roman(9), "IX");
    assert_eq!(arabic_to_roman(10), "X");
    assert_eq!(arabic_to_roman(21), "XXI");
    assert_eq!(arabic_to_roman(49), "XLIX");
    assert_eq!(arabic_to_roman(999), "CMXCIX");
    assert_eq!(arabic_to_roman(2222), "MMCCXXII"); 
    assert_eq!(arabic_to_roman(3687), "MMMDCLXXXVII"); 
}

#[test]
fn test_roman_to_arabic() {
    assert_eq!(roman_to_arabic(""), Some(0));
    assert_eq!(roman_to_arabic("I"), Some(1));
    assert_eq!(roman_to_arabic("II"), Some(2));
    assert_eq!(roman_to_arabic("III"), Some(3));
    assert_eq!(roman_to_arabic("IV"), Some(4));
    assert_eq!(roman_to_arabic("XIV"), Some(14));
    assert_eq!(roman_to_arabic("XZX"), Some(20));
    assert_eq!(roman_to_arabic("XLIX"), Some(49));
    assert_eq!(roman_to_arabic("CDXLVIII"), Some(448));
    assert_eq!(roman_to_arabic("MMMDCCLXVII"), Some(3767));
}

#[test]
fn test_fizzbuzz() {
    assert_eq!(fizzbuzz(2), "2");
    assert_eq!(fizzbuzz(3), "fizz");
    assert_eq!(fizzbuzz(5), "buzz");
    assert_eq!(fizzbuzz(15), "fizzbuzz");
}