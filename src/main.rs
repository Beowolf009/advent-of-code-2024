use std::fs::File;
use std::io::{self, prelude::*};

fn main() -> io::Result<()> {
    // Open the file
    let mut file = File::open("list_day1.txt")?;

    // Initialize a new string to hold the file contents
    let mut contents = String::new();

    // Read the file contents into the string
    file.read_to_string(&mut contents)?;

    // Initialize vectors to hold the left and right values
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    // Split the contents by whitespace and sort into left and right vectors
    let mut split = contents.split_whitespace();

    // Loop through the split contents and push the values into the vectors
    while let (Some(left), Some(right)) = (split.next(), split.next()) {
        left_list.push(left.to_string());
        right_list.push(right.to_string());
    }

    //sort the vecctors
    left_list.sort();
    right_list.sort();

    //convert the vectors into integers
    let left_list: Vec<i32> = left_list.iter().map(|x| x.parse::<i32>().unwrap()).collect();
    let right_list: Vec<i32> = right_list.iter().map(|x| x.parse::<i32>().unwrap()).collect();

    // Print the contents of the vectors
    println!("Left list: {:?}", left_list);
    println!("Right list: {:?}", right_list);

    //compare each left and right list indexes and add the running total
    let mut total: i32 = 0;
    let mut diff: i32 = 0;
    for i in 0..left_list.len() {
      //compare the differences between the left and right list indexes
      if left_list[i] > right_list[i] {
        let diff = left_list[i] - right_list[i];
        //add the differences to the total
        total += diff;
      } else if left_list[i] < right_list[i] {
        let diff = right_list[i] - left_list[i];
        //add the differences to the total
        total += diff;
      }
       total += diff;

    }
    println!("Total: {:?}", total);





    




    Ok(())
}