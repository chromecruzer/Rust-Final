#![allow(unused)]
use::std::cmp::Ordering;
use std::collections::HashMap;

fn main() {
    println!("Hello, Janani & Nethra");

let (valx,valy):(i32,i32) = (40 , 50 ); 
let val: i32 = 20;
let val2: i32 = 30;

match valx.cmp(&valy){
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

fn adder(x:i32) ->(i32,i32){
    return (x+1, x+2); 
}

fn array_adder(list: &[i32]) -> i32{
    let mut sum: i32 = 0;
    for &val in list.iter(){
        sum += &val;
    }
    sum
    
}

println!("Normal={:?}, List={}, Generics={}",adder(9), array_adder(&[2,3,4,5]), work_generics(9,9));


// Generics 

fn work_generics<T: std::ops::Add<Output = T>>(x: T,y: T) -> T {
    return  x + y;
}

// Borrow checker and mutable reference

fn nor_check(ishu: String){
    println!("{}", ishu )

}

fn abnor_check(potta:  &mut String)->String{
    potta.push_str(" is to sacrifice that BITCH...!!");
    println!("{:?}", potta);
    potta.to_string()
}

let test = nor_check("Normal Ishu".to_string());
let test2 = abnor_check(&mut "The only way to get outta this".to_string());
println!("this is nor_check = {:?}, this is abnor_check = {}", test, test2);


// Hashmaps 

let mut assmap = HashMap::new();
assmap.insert("kayal", "Vizhi");
assmap.insert("Malar", "Vizhi");
println!("this is hashmap nigga = {:?}", assmap);

for(&k,v) in assmap.iter(){
    println!("{} = {}",k,v);
    if k == "kayal"{
     println!("Kayal is Found")
    }
}

// Struct its a custom datatypes we store all .

struct Customer{
    name: String,
    age: i8,
    ismale: bool,
}

let   mut tyson = Customer{
name: "Bob".to_string(),
age: 49,
ismale: true
};
tyson.age = 56;
println!("the name is {}, age = {}, is Gender is Male? {}", tyson.name,tyson.age, tyson.ismale);

// Structs with trait // structs = all bundle of datatypes variables and trait = fn()s that  can match the struct to perform actions.

struct Motta{
    fav: String,
    age: i8
}

trait LovesTo {
    fn fuck_to(&self);
}

impl  LovesTo for Motta {
    fn fuck_to(&self) {
        println!("He loves to seduce girls..!!, His fav is  {}", &self.fav)
    }
}

let vattam = Motta{
    fav: "Gayu".to_string(),
    age: 25
};

vattam.fuck_to()







} 