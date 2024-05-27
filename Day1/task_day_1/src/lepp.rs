pub fn lpp(){
    let mut sum_for = 0;
    for i in 1..=10 {
        sum_for += i;
    }
    println!("Сумма чисел от 1 до 10 с использованием цикла for: {}", sum_for);   
    // Сумма чисел от 1 до 10 с использованием цикла while
    let mut sum_while = 0;
    let mut i = 1;
    while i <= 10 {
        sum_while += i;
        i += 1;
    }
    println!("Сумма чисел от 1 до 10 с использованием цикла while: {}", sum_while);  
        // Сумма чисел от 1 до 10 с использованием цикла loop
        let mut sum_loop = 0;
        let mut i = 1;
        loop {
            if i > 10 {
                break;
            }
            sum_loop += i;
            i += 1;
        }
    println!("Сумма чисел от 1 до 10 с использованием цикла loop: {}", sum_loop);
}