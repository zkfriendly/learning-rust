mod random_data_loader;

fn main() {
    let data = random_data_loader::load();

    println!("{:?}", data);
}
