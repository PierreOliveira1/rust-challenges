use std::io;

fn main() {
    let mut inputs = String::new();
    io::stdin().read_line(&mut inputs).unwrap();

    if let Some((a, b)) = inputs.split_once(" ") {
        let item: i32 = a.trim().parse().expect("Invalid input item");
        let amount: f32 = b.trim().parse().expect("Invalid input amount");

        let total: f32 = match item {
            1 => amount * 4.00,
            2 => amount * 4.50,
            3 => amount * 5.00,
            4 => amount * 2.00,
            5 => amount * 1.50,
            _ => 0.0,
        };

        println!("Total: R$ {:.2}", total);
    }
}
