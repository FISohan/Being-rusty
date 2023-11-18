use std:: io;
fn main() {
    let mut min = 1;
    let mut max = 100;
    let mut sol = (min + max) / 2;

    loop {
        let mut cmd = String::new();
        println!(" I Guess: {sol}");
        io::stdin().read_line(&mut cmd).expect("err");
        cmd = cmd.trim().to_string();
        match cmd.as_str() {
            "<" => {
                min = sol;
                sol = (min + max) / 2;
                continue;
            }
            ">" => {
                max = sol;
                sol = (min + max) / 2;
                continue;
            }
            "=" => {
                println!("Ans = {sol}");
                break;
            }
            _ => continue,
        }
    }
}

//43