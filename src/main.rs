// /*-------------------------ASSIGNMENT # 3-------------------------

// --------------------- Question # 1 & 2 -------------------------------------------------

// Question - 1
// Write a function whcih expects a closure with FnOnce() trait and call that
// closure in the function body.


// Question - 2
// Write a function which expects a closure with FnMut() trait and call that 
// closure in the function body
// */

// #![allow(unused)]
// use std::thread;
// use std::time::Duration;

// fn karachi_hotel<F, T> (dish: F, mut oder: T)
// where T: FnMut(),                              /* FnMut */
//       F: FnOnce() -> String                    /* FnOnce */
// {
//       oder();
//       oder();
//       oder();
//       println!("\nConsumed: {}", dish());
//       println!("Delicious!\n");
// }

// fn main() {
//     let dish = String::from("Karachi Biryani");
//     let mut consume_and_return_dish = move || dish;
//     let mut y: usize = 0;
//     let mut number_of_plates = || y += 1;
//     karachi_hotel(consume_and_return_dish, number_of_plates);
//     println!("Number of Persons {}\nNumber of Biryani Plates {}\n", y, y);
// }


/*--------------------- Question # 3 -------------------------------------------------

create a new thread with spawn and waiting for all threads to finish using join handles.
// */
// #![allow(unused)]
// use std::thread;
// use std::time::Duration;

// fn main(){

//     let handle = thread::spawn(|| {
//         for i in 1..10 {
//         for k in 1..i {
//         if i == 2
//         {
//             print!("*-------Spawn Thread Has Started");
//         }
//         print!("*");
//         if k == 8
//         {
//             print!("-------Spawn Thread Has End");
//         }
//         }
//             println!("");
//             thread::sleep(Duration::from_millis(100));
//         }
//     });

//     handle.join().unwrap();

//     for i in (10..20).rev() {
//         if i == 11
//         {
//             print!("*-------Main Thread Has End");
//         }
//         for k in (10..i).rev()
//         {            
//             print!("*");

//         }
//         if i == 18
//         {
//             print!("-------Main Thread Has Started");
//         }
//         println!("");
//         thread::sleep(Duration::from_millis(100));
//     } 
// }
