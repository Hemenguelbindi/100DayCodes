pub fn result(){
    let s = String::from("Hello");
    let len = calculate(&s);
    println!("Длина строки: {s} = {len}");
}

fn calculate(s: &String) -> usize {
    s.len()
}