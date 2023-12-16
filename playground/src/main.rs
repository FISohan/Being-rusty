fn main() {
    let mut s1 = String::from("sohan");
    let c1 = s1;
    {
    let c2 = s1;
    }
    println!("{c2}")
}

/*
brrowing

*/