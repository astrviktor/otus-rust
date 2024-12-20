trait Coffee {
    fn cost(&self) -> f32;
}

struct SimpleCoffee;

impl Coffee for SimpleCoffee {
    fn cost(&self) -> f32 {
        1.0
    }
}

struct MilkDecorator {
    coffee: Box<dyn Coffee>,
}

impl Coffee for MilkDecorator {
    fn cost(&self) -> f32 {
        self.coffee.cost() + 0.5
    }
}

struct SugarDecorator {
    coffee: Box<dyn Coffee>,
}

impl Coffee for SugarDecorator {
    fn cost(&self) -> f32 {
        self.coffee.cost() + 0.3
    }
}

fn main() {
    let simple_coffee = SimpleCoffee {};
    println!("Cost of simple coffee: {}", simple_coffee.cost());

    let milked_coffee = MilkDecorator {
        coffee: Box::new(simple_coffee),
    };
    println!("Cost of milk decorated coffee: {}", milked_coffee.cost());

    let sweet_milked_coffee = SugarDecorator {
        coffee: Box::new(milked_coffee),
    };
    println!(
        "Cost of sugar and milk decorated coffee: {}",
        sweet_milked_coffee.cost()
    );
}
