/*
    CIS198 Homework 1
    Part 1: Implementing functions

    Complete and write at least one unit test for each function you implement.
    If it already has a unit test, either add assertions to it or add a new one.
    Also answer the questions in text.
*/

// Remove these once you are done editing the file!
// This will result in useful warnings if you missed something.
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(clippy::while_let_on_iterator)]

/*
    Problem 1: Double

    Implement the function that doubles an integer in three different ways.

    What are some differences between them? Can you write unit tests
    which fail (or fail to compile) for some but not others?

    Which of the three do you prefer?
*/

pub fn double_v1(n: i32) -> i32 {
    n + n
}

pub fn double_v2(n: &i32) -> i32 {
    *n + *n
}

pub fn double_v3(n: &mut i32) {
    *n = *n + *n;
}

// Example unit test (so you can recall the syntax)
#[test]
fn test_double_v1() {
    assert_eq!(double_v1(2), 4);
    assert_eq!(double_v1(-3), -6);
}
#[test]
fn test_double_v2() {
    assert_eq!(double_v2(&2), 4);
    assert_eq!(double_v2(&-3), -6);
}

#[test]
fn test_double_v3() {
    let mut n = 2;
    double_v3(&mut n);
    assert_eq!(n, 4);

    n = -3;
    double_v3(&mut n);
    assert_eq!(n, -6)
}

/*
    Problem 2: Integer square root

    Implement the integer square root function: sqrt(n) should return the
    largest m such that m * m <= n. For a 'harder' version, try to do it more
    efficiently than trying every possibility.
*/
pub fn sqrt(n: usize) -> usize {
    let half: usize = n / 2;

    for m in (1..half).rev() {
        if m * m <= n {
            return m;
        }
    }

    1
}

// Remember to write unit tests here (and on all future functions)
#[test]
fn test_sqrt() {
    assert_eq!(sqrt(9), 3);
    assert_eq!(sqrt(17), 4);
    assert_eq!(sqrt(3), 1);
    assert_eq!(sqrt(2), 1);
    assert_eq!(sqrt(1), 1);
}

/*
    Problem 3: Slice sum

    Implement the sum function on slices in two different ways
    (using different for loop patterns).
    Do not use the predefined sum function.
    Also, try to do it without an unnecessary `return` statement at the end --
    Clippy should detect if you mess this up.

    Which of the two ways do you prefer?
*/
pub fn sum_v1(slice: &[i32]) -> i32 {
    // do some initialization...
    let mut sum: i32 = 0;

    for &v in slice {
        sum += v;
        // ...
    }
    sum
}

pub fn sum_v2(slice: &[i32]) -> i32 {
    // do some initialization...
    let mut sum: i32 = 0;
    let mut slice_iter = slice.iter();

    while let Some(val) = slice_iter.next() {
        sum += val;
    }

    sum
}

#[test]
fn test_sum_v1() {
    let slice = &[1, 2, 3];
    assert_eq!(sum_v1(slice), 6);
}

#[test]
fn test_sum_v2() {
    let slice = &[1, 2, 3];
    assert_eq!(sum_v2(slice), 6);
}

/*
    Problem 4: Unique

    Make unique. Create a new vector which contains each item in the vector
    only once! Much like a set would.
    This doesn't need to be efficient; you can use a for loop.
*/

pub fn unique(slice: &[i32]) -> Vec<i32> {
    let mut new_vec = Vec::new();

    for &i in slice {
        if !new_vec.contains(&i) {
            new_vec.push(i);
        }
    }

    new_vec
}

#[test]
fn test_unique() {
    let slice = &[1, 2, 2, 3, 3, 3];
    let answer_slice = &[1, 2, 3];
    assert_eq!(unique(slice), answer_slice);
}

/*
    Problem 5: Filter

    Return a new vector containing only elements that satisfy `pred`.
    This uses some unfamiliar syntax for the type of pred -- all you need
    to know is that pred is a function from i32 to bool.
*/
pub fn filter(slice: &[i32], pred: impl Fn(i32) -> bool) -> Vec<i32> {
    let mut new_vec = Vec::new();
    for &i in slice {
        if pred(i) {
            new_vec.push(i);
        }
    }

    new_vec
}

#[test]
fn test_filter() {
    fn is_even(n: i32) -> bool {
        n % 2 == 0
    }
    assert_eq!(filter(&vec![1, 2, 3, 4, 5, 6], &is_even), vec![2, 4, 6]);
}

/*
    Problem 6: Fibonacci

    Given starting fibonacci numbers n1 and n2, compute a vector of
    length 'out_size'
    where v[i] is the ith fibonacci number.
*/
pub fn fibonacci(n1: i32, n2: i32, out_size: usize) -> Vec<i32> {
    let mut new_vec = vec![n1, n2];

    for i in 2..out_size {
        let tmp = new_vec[i - 1] + new_vec[i - 2];
        new_vec.push(tmp);
    }

    new_vec
}

#[test]
fn test_fibonacci() {
    let answer_slice1 = vec![1, 2, 3, 5, 8, 13];

    assert_eq!(fibonacci(1, 2, 6), answer_slice1);
}

/*
    Problem 7: String concatenation

    Create a function which concats 2 &strs and returns a String,
    and a function which concats 2 Strings and returns a String.

    You may use any standard library function you wish.

    What are some reasons the second function is not efficient?
*/
pub fn str_concat(s1: &str, s2: &str) -> String {
    s1.to_string() + s2
}

#[test]
fn test_str_concat() {
    assert_eq!(str_concat("test", "123"), "test123");
}

pub fn string_concat(s1: String, s2: String) -> String {
    s1 + &s2
}

#[test]
fn test_string_concat() {
    let a = String::from("test");
    let b = String::from("123");
    let ans = String::from("test123");
    assert_eq!(string_concat(a, b), ans);
}

/*
    Problem 8: String concatenation continued

    Convert a Vec<String> into a String.
    Your answer to the previous part may help.
*/

pub fn concat_all(v: Vec<String>) -> String {
    let mut new_str = String::new();
    for s in v {
        new_str = string_concat(new_str, s);
    }

    new_str
}

#[test]
fn test_concat_all() {
    let str_vec = vec![String::from("hello"), String::from("world")];

    assert_eq!(concat_all(str_vec), String::from("helloworld"))
}

/*
    Problem 9: Parsing

    Convert a Vec<String> into a Vec<i32> and vice versa.

    Assume all strings are correct numbers! We will do error handling later.
    Use `.expect("ignoring error")` to ignore Result from parse()
    See https://doc.rust-lang.org/std/primitive.str.html#method.parse

    The unit tests check if your functions are inverses of each other.

    A useful macro: format! is like println! but returns a String.
*/

pub fn parse_all(v: Vec<String>) -> Vec<i32> {
    let mut new_vec = Vec::new();

    for s in v {
        new_vec.push(s.parse::<i32>().unwrap());
    }

    new_vec
}

pub fn print_all(v: Vec<i32>) -> Vec<String> {
    let mut new_vec = Vec::new();

    for i in v {
        new_vec.push(i.to_string());
    }

    new_vec
}

#[test]
fn test_print_parse() {
    assert_eq!(parse_all(print_all(vec![1, 2])), vec![1, 2]);
}

#[test]
fn test_parse_print() {
    let v = vec!["1".to_string(), "2".to_string()];
    assert_eq!(print_all(parse_all(v.clone())), v);
}

/*
    Problem 10: Composing functions

    Implement a function which concatenates the even Fibonacci
    numbers out of the first n Fibonacci numbers.

    For example: if n = 6, the first 5 Fibonacci numbers are 1, 1, 2, 3, 5, 8,
    so the function should return the String "28".

    Don't use a for loop! Your previous functions should be sufficient.
*/

pub fn concat_even_fibonaccis(n: usize) -> String {
    let mut fib_vec = fibonacci(1, 1, n);

    fib_vec = filter(&fib_vec, &|n| n % 2 == 0);
    let string_fib_vec = print_all(fib_vec);
    concat_all(string_fib_vec)
}

#[test]
fn test_concat_even_fibonaccis() {
    assert_eq!(&concat_even_fibonaccis(6), "28");
    assert_eq!(&concat_even_fibonaccis(9), "2834");
}
