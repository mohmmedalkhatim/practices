pub fn init(s: String, k: usize) {
    let start = k - 1;
    let v: Vec<&str> = s.split("-").collect();
    let re = v.join("");
    let r: Vec<&str> = re.split("").collect();
    let end = v.len();
    for item in start..end {
        print!("  {}", r[item])
    }
    println!("{:?}", r)
}
