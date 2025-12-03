struct Laptop {
    price: u32,
}

impl Laptop {
    fn cost(&self, qty: u32) -> u32 {
        self.price * qty
    }
}

fn main() {
    let laptops = [
        Laptop { price: 650_000 }, // HP
        Laptop { price: 755_000 }, // IBM
        Laptop { price: 550_000 }, // Toshiba
        Laptop { price: 850_000 }, // Dell
    ];

    


let total: u32 = laptops.iter().map(|l| l.cost(3)).sum();

       println!("Total cost: ₦{}", total);
}

