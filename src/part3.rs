/*
    CIS198 Homework 1
    Part 3: Ownership, move semantics, and lifetimes

    Complete and write at least one unit test for each function you implement.
    If it already has a unit test, either add assertions to it or add a new one.
    Also answer the questions in text.
*/

// Remove these once you are done editing the file!
#![allow(dead_code)]
#![allow(unused_variables)]

/*
    Problem 1: Swap ints

    Implement the function that swaps two integers, and write unit tests.

    The Rust borrow checker may help avoid some possible bugs.

    Then answer this question:
    Q: A common source of error in swap implementations is failing to work if
       the two references are the same. Why don't you need to worry about this
       case in Rust?

    (Try writing a unit test where they are both
    the same, i.e. swap_ints(&mut x, &mut x).)
*/
pub fn swap_ints<'a>(x1: &'a mut i32, x2: &'a mut i32) {
    let tmp: i32;

    tmp = *x1;
    *x1 = *x2;
    *x2 = tmp;
}

#[test]
fn test_swap_ints() {
    let mut x = 0;
    let mut y = 1;

    swap_ints(&mut x, &mut y);

    assert_eq!(x, 1);
    assert_eq!(y, 0);

    // swap_ints(&mut x, &mut x);
    // The line above fails because more than one variable borrows the same mutable object.
    // Rust doesn't allow a mutable object access by two variables simultaneously.
}

/*
    Problem 2: String duplication
*/
#[test]
fn copy_string_test() {
    let str1 = String::from("foo");
    let str2 = str1.clone();
    assert_eq!(str1, str2);
}
// This test doesn't work. Fix it by copying strings properly.
// Q1. What went wrong?
// String does not implement the Copy trait, and therefore the ownership of String will be moved by the assignment.
// Clone a new String, and move the ownership of the cloned String to str2.

// Q2. How come it works fine here?
// i32 does implement the Copy trait
#[test]
fn copy_int_test() {
    let i1 = 1;
    let i2 = i1;
    assert_eq!(i1, i2);
}

// Now implement the following function that duplicates a string n times.
fn duplicate_string(s: &str, times: usize) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for _ in 0..times {
        result.push(s.to_string());
    }
    result
}

#[test]
fn test_duplicate_string() {
    let ans = vec!["str", "str", "str"];

    assert_eq!(duplicate_string("str", 3), ans);
}

/*
    Problem 3: String duplication continued

    These two don't work either. Fix by changing the type of "string" in the
    function copy_me ONLY, and by adjusting the parameter to "copy_me" where
    it's called.
*/

fn copy_me(string: /* Change in here only*/ &String) -> String {
    string.clone()
}

#[test]
fn copy_me_test() {
    let str1 = String::from("foo");
    assert_eq!(str1, copy_me(/* Change in here only*/ &str1));
}

#[test]
fn copy_me_test2() {
    let str1 = String::from("foo");
    let str2 = copy_me(&str1 /* Change in here only*/);
    assert_eq!(str1, str2);
}

/*
    Problem 4: Lifetime specifiers

    For each of the following three functions, either implement it by adding
    lifetime specifiers, or explain why this is not possible.

    (It's not truly impossible -- we will see later on that advanced features
    such as "unsafe code" can be used to turn off Rust's safety and lifetime
    checks.)
*/
// The function below is impossible to work
// fn new_ref_string() -> &'static String {
//     return &String::from("hello");
// }

fn new_ref_str() -> &'static str {
    "hello"
}

#[test]
fn test_new_ref_str() {
    assert_eq!(new_ref_str(), "hello");
}

// The same function from part2
fn pick_longest2<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    match s1.len() >= s2.len() {
        true => s1,
        false => s2,
    }
}

#[test]
fn test_pick_longest2() {
    assert_eq!(pick_longest2("hello", "world666"), "world666");
}

/*
    Problem 5: Using functions with lifetimes

    Write two versions of a function which returns the longest string in a
    vector, using pick_longest2 as a helper function.

    If the vector is empty, return "".

    Q1. In pick_longest_in_v2, if you were to explicitly specify the lifetime
        of the input and output, what should it be?

    Q2. What are the pros and cons of v1 and v2?
*/

fn pick_longest_in_v1(v: Vec<String>) -> String {
    if v.is_empty() {
        return "".to_owned();
    }

    let mut cur_longest = v[0].clone();
    for s in v {
        cur_longest =
            pick_longest2(cur_longest.as_str(), s.as_str()).to_owned();
    }
    cur_longest
}

#[test]
fn test_pick_longest_in_v1() {
    let v: Vec<String> = vec![];
    assert_eq!(pick_longest_in_v1(v).as_str(), "");

    let v2 =
        vec![String::from("123"), String::from("4567"), String::from("aabbc")];
    assert_eq!(pick_longest_in_v1(v2).as_str(), "aabbc");
}

fn pick_longest_in_v2<'a>(v: Vec<&'a str>) -> &'a str {
    if v.is_empty() {
        return "";
    }

    let mut cur_longest = v[0];
    for s in v {
        cur_longest = pick_longest2(cur_longest, s);
    }
    cur_longest
}

#[test]
fn test_pick_longest_in_v2() {
    let v: Vec<&str> = vec![];
    assert_eq!(pick_longest_in_v2(v), "");

    let v2 = vec!["123", "4567", "aabbc"];
    assert_eq!(pick_longest_in_v2(v2), "aabbc");
}

/*
    Problem 6: Move semantics

    Write three versions of a function that pads a vector with zeros.
    Fail if the vector is larger than the desired length.

    Use .clone() if necessary to make any additional unit tests compile.

    Which of these functions do you prefer? Which is the most efficient?
*/

fn pad_with_zeros_v1(v: Vec<usize>, desired_len: usize) -> Vec<usize> {
    if v.len() > desired_len {
        panic!("the length of vector is larger than desired_len");
    }

    let mut result: Vec<usize> = v;

    for i in result.len()..desired_len {
        result.push(0);
    }
    debug_assert_eq!(result.len(), desired_len);

    result
}

fn pad_with_zeros_v2(slice: &[usize], desired_len: usize) -> Vec<usize> {
    if slice.len() > desired_len {
        panic!("the length of vector is larger than desired_len");
    }

    let mut result: Vec<usize> = slice.to_vec();

    for i in slice.len()..desired_len {
        result.push(0);
    }

    debug_assert_eq!(result.len(), desired_len);

    result
}

fn pad_with_zeros_v3(v: &mut Vec<usize>, desired_len: usize) {
    if v.len() > desired_len {
        panic!("the length of vector is larger than desired_len");
    }

    for i in v.len()..desired_len {
        v.push(0);
    }

    debug_assert_eq!(v.len(), desired_len);
}

#[test]
fn test_pad_twice_v1() {
    let v = vec![1];
    let v = pad_with_zeros_v1(v, 2);
    let v = pad_with_zeros_v1(v, 4);
    assert_eq!(v, vec![1, 0, 0, 0]);
}

#[test]
fn test_pad_twice_v2() {
    let v = vec![1];
    let v = pad_with_zeros_v2(&v, 2);
    let v = pad_with_zeros_v2(&v, 4);
    assert_eq!(v, vec![1, 0, 0, 0]);
}

#[test]
fn test_pad_twice_v3() {
    let mut v = vec![1];
    pad_with_zeros_v3(&mut v, 2);
    pad_with_zeros_v3(&mut v, 4);
    assert_eq!(v, vec![1, 0, 0, 0]);
}

/*
    Problem 7: Move semantics continued

    Write a function which appends a row to a vector of vectors.
    Notice that it takes ownership over the row.
    You shouldn't need to use .clone().

    Why is this more general than being passed a &[bool]
    and cloning it?

    Second, write a function which returns whether
    a row equals the first row in the vector of vectors.
    Notice that it does not take ownership over the row.

    Why is this more general than being passed a Vec<bool>?
*/

fn append_row(grid: &mut Vec<Vec<bool>>, row: Vec<bool>) {
    grid.push(row);
}

#[test]
fn test_append_row() {
    let mut grid = vec![vec![true, true], vec![false, false]];
    let row = vec![false, true];

    append_row(&mut grid, row);
    assert_eq!(grid[2], vec![false, true]);
}

fn is_first_row(grid: &[Vec<bool>], row: &[bool]) -> bool {
    // Check if row is the first row in grid
    // Remember to handle the case when grid is empty
    if grid.is_empty() {
        return false;
    }

    grid[0] == row
}

#[test]
fn test_is_first_row() {
    let grid = vec![vec![true, true], vec![false, false]];
    let row = vec![true, true];
    let second_row = vec![false, false];

    assert_eq!(is_first_row(&grid, &row), true);
    assert_eq!(is_first_row(&grid, &second_row), false);
}

/*
    Problem 8: Modifying while iterating

    In C and C++, you run into subtle bugs if you try to modify a data
    structure while iterating over it. Rust's move semantics prevents that.
*/

use std::collections::HashMap;

// To familiarize yourself with HashMaps,
// implement the following function which converts pairs from a slice
// into key-value pairs in a hashmap.
// Documentation:
// https://doc.rust-lang.org/std/collections/struct.HashMap.html

fn vector_to_hashmap(v: &[(i32, String)]) -> HashMap<i32, String> {
    let mut map: HashMap<i32, String> = HashMap::new();

    for t in v {
        map.insert(t.0, t.1.clone());
    }

    map
}

#[test]
fn test_vector_to_hashmap() {
    let my_vec = vec![(1, "one".to_owned()), (2, "two".to_owned())];

    let map = vector_to_hashmap(&my_vec);

    assert_eq!(map.get(&1).unwrap().as_str(), "one");
    assert_eq!(map.get(&2).unwrap().as_str(), "two");
}

// Now rewrite this function to delete all entries in hashmap where the keys
// are negative.
fn delete_negative_keys(h: &mut HashMap<i32, i32>) {
    // This fails, uncomment to see error.
    h.retain(|&k, _| k >= 0);
}

#[test]
fn pass_test_delete_negative_keys() {
    let mut map: HashMap<i32, i32> = (-3..8).map(|x| (x, x)).collect();
    delete_negative_keys(&mut map);

    assert_eq!(map.contains_key(&-3), false);
    assert_eq!(map.contains_key(&-2), false);
    assert_eq!(map.contains_key(&-1), false);
}

/*
    Problem 9: The Entry API

    Move semantics present interesting API design choices not found in other
    languages.
    HashMap is an example of such a API.
    Specifically, the Entry API:
    https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html

    This allows for efficient HashMap access because we only access
    the entry in the map (computing an expensive hash function) once.

    Implement a function which does the following:
        For all entries in `add`: (k, v)
        If `k` exists in `merged`, append `v` to the value of `merged[k]`.
        If that `k` doesn't exist in `merged`, add the (k, v) to `merged`.
    Use `or_insert` and `and_modify`.
*/

fn merge_maps(
    merged: &mut HashMap<String, String>,
    add: HashMap<String, String>,
) {
    for (k, v) in add {
        merged.entry(k).and_modify(|e| (*e).push_str(v.as_str())).or_insert(v);
    }
}

#[test]
fn test_merge_maps() {
    let mut merged: HashMap<String, String> = HashMap::new();
    let add: HashMap<String, String> = HashMap::from([
        ("a".to_owned(), "1".to_owned()),
        ("b".to_owned(), "2".to_owned()),
    ]);

    merge_maps(&mut merged, add);

    assert_eq!(merged.get("a").unwrap(), "1");
    assert_eq!(merged.get("b").unwrap(), "2");

    let add2: HashMap<String, String> = HashMap::from([
        ("a".to_owned(), "1".to_owned()),
        ("b".to_owned(), "2".to_owned()),
    ]);
    merge_maps(&mut merged, add2);

    assert_eq!(merged.get("a").unwrap(), "11");
    assert_eq!(merged.get("b").unwrap(), "22");
}
