// Структура Product, которая будет построена
#[derive(Debug, PartialEq)]
struct Product {
    part_a: String,
    part_b: i32,
    part_c: bool,
}

// Builder для структуры Product
struct ProductBuilder {
    part_a: String,
    part_b: i32,
    part_c: bool,
}

impl ProductBuilder {
    // Значения по умолчанию
    fn new() -> Self {
        ProductBuilder {
            part_a: String::from("default_part_a"),
            part_b: 0,
            part_c: false,
        }
    }

    // Методы для установки значений частей продукта

    fn with_part_a(mut self, part_a: &str) -> Self {
        self.part_a = part_a.to_string();
        self
    }

    fn with_part_b(mut self, part_b: i32) -> Self {
        self.part_b = part_b;
        self
    }

    fn with_part_c(mut self, part_c: bool) -> Self {
        self.part_c = part_c;
        self
    }

    // Метод для создания Product
    fn build(self) -> Product {
        Product {
            part_a: self.part_a,
            part_b: self.part_b,
            part_c: self.part_c,
        }
    }
}

fn main() {
    let product = ProductBuilder::new()
        .with_part_a("custom_part_a")
        .with_part_b(42)
        .with_part_c(true)
        .build();

    println!("{:?}", product);
}

// Тесты
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_product() {
        let product = ProductBuilder::new().build();
        assert_eq!(
            product,
            Product {
                part_a: String::from("default_part_a"),
                part_b: 0,
                part_c: false,
            }
        );
    }

    #[test]
    fn test_custom_product() {
        let product = ProductBuilder::new()
            .with_part_a("custom_part_a")
            .with_part_b(42)
            .with_part_c(true)
            .build();
        assert_eq!(
            product,
            Product {
                part_a: String::from("custom_part_a"),
                part_b: 42,
                part_c: true,
            }
        );
    }

    #[test]
    fn test_partial_custom_product() {
        let product = ProductBuilder::new().with_part_b(100).build();
        assert_eq!(
            product,
            Product {
                part_a: String::from("default_part_a"),
                part_b: 100,
                part_c: false,
            }
        );
    }
}
