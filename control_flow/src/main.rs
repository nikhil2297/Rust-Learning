fn main() {
    let mut count: u32 = 4;
    let mut result : u32 = 0;
    'counting_up: loop {
        if count == 0 {
            break;
        }
        let mut num: u32 = 10;
        let mut factorial : u32 = 1; 
        result += loop {
            if num == 1 {
                break factorial;
            }
            if count == 0 {
                continue 'counting_up;
            }
            factorial *= num;
            num -= 1;
        };
        println!("count = {count}, factorial : {factorial}");
        count -= 1;
    
    }
    println!("Result = {result}");    
}
