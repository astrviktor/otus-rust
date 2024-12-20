fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn log<F>(f: F) -> impl Fn(i32, i32) -> i32
where
    F: Fn(i32, i32) -> i32,
{
    move |a: i32, b: i32| {
        println!("Logging for func");
        println!("Calling func with arguments: a = {}, b = {}", a, b);
        let result = f(a, b);
        println!("func returned result: {}", result);
        result
    }
}

fn main() {
    let a = 5;
    let b = 7;

    // Напрямую вызываем функцию add
    let sum_direct = add(a, b);
    println!("Direct call to add: {}", sum_direct);

    // Создаем декорированную версию функции add
    let decorated_add = log(add);

    // Вызываем декорированную функцию
    let sum_decorated = decorated_add(a, b);
    println!("Decorated call to add: {}", sum_decorated);
}
