// Трейт Strategy с методом process, который будет реализован каждым алгоритмом.
trait Strategy {
    fn process(&self, data: &str) -> String;
}

// Конкретные стратегии, которые реализуют трейт Strategy.

struct UpperCaseStrategy;

impl Strategy for UpperCaseStrategy {
    fn process(&self, data: &str) -> String {
        data.to_uppercase()
    }
}

struct LowerCaseStrategy;

impl Strategy for LowerCaseStrategy {
    fn process(&self, data: &str) -> String {
        data.to_lowercase()
    }
}

struct ReverseStrategy;

impl Strategy for ReverseStrategy {
    fn process(&self, data: &str) -> String {
        data.chars().rev().collect()
    }
}

// Контекстная структура, которая использует стратегию.
struct Context {
    strategy: Box<dyn Strategy>,
}

impl Context {
    // Метод для установки новой стратегии
    fn set_strategy(&mut self, strategy: Box<dyn Strategy>) {
        self.strategy = strategy;
    }

    // Метод для выполнения операции по текущей стратегии
    fn execute(&self, data: &str) -> String {
        self.strategy.process(data)
    }
}

fn main() {
    let mut context = Context {
        strategy: Box::new(UpperCaseStrategy),
    };

    let data = "Hello, world!";

    println!("Using UpperCaseStrategy: {}", context.execute(data));

    context.set_strategy(Box::new(LowerCaseStrategy));
    println!("Using LowerCaseStrategy: {}", context.execute(data));

    context.set_strategy(Box::new(ReverseStrategy));
    println!("Using ReverseStrategy: {}", context.execute(data));
}

// Тесты
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upper_case_strategy() {
        let upper_case_strategy = Box::new(UpperCaseStrategy);
        let context = Context {
            strategy: upper_case_strategy,
        };
        assert_eq!(context.execute("hello, world!"), "HELLO, WORLD!");
    }

    #[test]
    fn test_lower_case_strategy() {
        let lower_case_strategy = Box::new(LowerCaseStrategy);
        let context = Context {
            strategy: lower_case_strategy,
        };
        assert_eq!(context.execute("HELLO, WORLD!"), "hello, world!");
    }

    #[test]
    fn test_reverse_strategy() {
        let reverse_strategy = Box::new(ReverseStrategy);
        let context = Context {
            strategy: reverse_strategy,
        };
        assert_eq!(context.execute("hello, world!"), "!dlrow ,olleh");
    }

    #[test]
    fn test_dynamic_strategy_change() {
        let mut context = Context {
            strategy: Box::new(UpperCaseStrategy),
        };
        assert_eq!(context.execute("hello, world!"), "HELLO, WORLD!");

        context.set_strategy(Box::new(LowerCaseStrategy));
        assert_eq!(context.execute("HELLO, WORLD!"), "hello, world!");

        context.set_strategy(Box::new(ReverseStrategy));
        assert_eq!(context.execute("Hello, world!"), "!dlrow ,olleH");
    }
}
