fn main() {
    for i in (0..10) {
    let f = nth_fiboncci(i);
    println!("{i}th {f}");
    }

}

fn nth_fiboncci(n:i32) -> i32 {
    let mut a:i32 = 0;
    let mut b:i32 = 1;
    let mut c = 0;
    let mut cnt = 0;

    if n == 0 { return a;}
    if n == 1 {return  b;}
    while n - 2  >= cnt {
        c = a + b;
        a = b;
        b = c;
        cnt += 1;
    }
    c
}

// 0 1 1 2 3 5 8