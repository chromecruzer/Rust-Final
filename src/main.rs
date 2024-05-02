#![allow(unused)]
use ::std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::{default, error, option, string};

fn main() {
    println!("Hello, Janani & Nethra");

    let (valx, valy): (i32, i32) = (40, 50);
    let val: i32 = 20;
    let val2: i32 = 30;

    match valx.cmp(&valy) {
        Ordering::Less => println!("you cannot vote"),
        Ordering::Greater => println!("You can vote"),
        Ordering::Equal => println!("congrates u have the right to vote !!"),
    }

    // loop

    // rust loop and arrays

    let arr1: [i32; 4] = [2, 6, 4, 1];
    let mut loopidx: usize = 0;

    loop {
        if arr1[loopidx] % 2 == 0 {
            println!("the values are {}", arr1[loopidx]);
        }
        loopidx += 1;

        if loopidx >= arr1.len() {
            break; // Break loop if reached end of array
        }
    }

    fn adder(x: i32) -> (i32, i32) {
        return (x + 1, x + 2);
    }

    fn array_adder(list: &[i32]) -> i32 {
        let mut sum: i32 = 0;
        for &val in list.iter() {
            sum += &val;
        }
        sum
    }

    println!(
        "Normal={:?}, List={}, Generics={}",
        adder(9),
        array_adder(&[2, 3, 4, 5]),
        work_generics(9, 9)
    );

    // Generics

    fn work_generics<T: std::ops::Add<Output = T>>(x: T, y: T) -> T {
        return x + y;
    }

    // Borrow checker and mutable reference

    fn nor_check(ishu: String) {
        println!("{}", ishu)
    }

    fn abnor_check(potta: &mut String) -> String {
        potta.push_str(" is to sacrifice that BITCH...!!");
        println!("{:?}", potta);
        potta.to_string()
    }

    let test = nor_check("Normal Ishu".to_string());
    let test2 = abnor_check(&mut "The only way to get outta this".to_string());
    println!(
        "this is nor_check = {:?}, this is abnor_check = {}",
        test, test2
    );

    // Hashmaps

    let mut assmap = HashMap::new();
    assmap.insert("kayal", "Vizhi");
    assmap.insert("Malar", "Vizhi");
    println!("this is hashmap nigga = {:?}", assmap);

    for (&k, v) in assmap.iter() {
        println!("{} = {}", k, v);
        if k == "kayal" {
            println!("Kayal is Found")
        }
    }

    // Struct its a custom datatypes we store all .

    struct Customer {
        name: String,
        age: i8,
        ismale: bool,
    }

    let mut tyson = Customer {
        name: "Bob".to_string(),
        age: 49,
        ismale: true,
    };
    tyson.age = 56;
    println!(
        "the name is {}, age = {}, is Gender is Male? {}",
        tyson.name, tyson.age, tyson.ismale
    );

    // Structs with trait // structs = all bundle of datatypes variables and trait = fn()s that  can match the struct to perform actions.

    struct Motta {
        fav: String,
        age: i8,
    }

    trait LovesTo {
        fn fuck_to(&self);
    }

    impl LovesTo for Motta {
        fn fuck_to(&self) {
            println!("He loves to seduce girls..!!, His fav is  {}", &self.fav)
        }
    }

    let vattam = Motta {
        fav: "Gayu".to_string(),
        age: 25,
    };

    vattam.fuck_to();

    // File creation in RUST

    // let file_name = "sample.txt";
    //     let mut output = match File::create(file_name) {
    //         Ok(file) => file,
    //         Err(error) => panic!("Error creating file: {:?}", error),
    //     };

    //     write!(output, "some random text\nHolla").expect("Failed to write to file");
    //     let input = File::open(file_name).unwrap();

    //////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // let buffered = BufReader::new(input);
    // for i in buffered.lines(){
    //      println!("{:?}",i.unwrap())
    // }

    //  let output2 = File::create("random2.txt");
    //  let mut output2 = match output2{
    //     Ok(file) => file,
    //     Err(error) => panic!("Error creating file: {:?}", error),
    //  }

    // closures in rust

    let can_vote = |age: i32| age >= 18;

    println!("voting age is {}", can_vote(8));

    // bit advanced closures
    fn closure_test<T>(a: i32, b: i32, func: T) -> i32
    where
        T: Fn(i32, i32) -> i32,
    {
        //func(a,b)
        return func(a, b);
        //return panic!("jff");
    }

    let sum = |a, b| a + b;
    let product = |a, b| a * b;
    // let div = |a,b| a/b;
    //
    println!("The sum is = {}", closure_test(5, 4, sum));
    println!("The Product is = {}", closure_test(5, 4, product));

    // smart pointers
    #[derive(Debug)] // Add this derive attribute to automatically implement Debug trait
    struct TreeNode<T> {
        pub left: Option<Box<TreeNode<T>>>,
        pub right: Option<Box<TreeNode<T>>>,
        pub key: T,
    }

    impl<T> TreeNode<T> {
        pub fn new(key: T) -> Self {
            TreeNode {
                left: None,
                right: None,
                key,
            }
        }

        pub fn with_left(mut self, node: TreeNode<T>) -> Self {
            self.left = Some(Box::new(node));
            self
        }

        pub fn with_right(mut self, node: TreeNode<T>) -> Self {
            self.right = Some(Box::new(node));
            self
        }
    }

    let test3 = TreeNode::new(4)
        .with_left(TreeNode::new(9))
        .with_right(TreeNode::new(10));

    println!("tree node is {:#?}", test3);
}
