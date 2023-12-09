use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}



fn process_line( line: & str ) -> u64 { 

    let parts = line.split(":").collect::<Vec<&str>>();

    let s2 = parts[1];
    let s3 = s2.split("|").collect::<Vec<&str>>();

    let lhs = s3[0];
    let rhs = s3[1];

    let nums_lhs = lhs.trim().split(" ").filter( |x| !x.is_empty() ).map( |x| x.parse::<i32>().unwrap() ).collect::<Vec<i32>>();
    let nums_rhs = rhs.trim().split(" ").filter( |x| !x.is_empty() ).map( |x| x.parse::<i32>().unwrap() ).collect::<Vec<i32>>();

    let mut n_valid = 0_u32;

    for n in nums_lhs{ 
        if nums_rhs.contains(&n) { 
            n_valid += 1; 
        }
    }
    
    let mut result = 0_u64;
    if n_valid>0{ 
        result = 2_u64.pow(n_valid-1 );
    } 
    result

}



fn main() {

    let lines = read_lines( "input.txt");

    let mut result = 0_u64;
    for line in lines {
        result += process_line( &line );
    }
        
    println!( "Result: {} ", result);
    println!("Hello, world!");
}
