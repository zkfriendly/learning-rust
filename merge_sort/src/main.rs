mod merge_sort;
mod random_data_loader;

fn main() {
    let mut data = random_data_loader::load();

    println!("Is sorted before: {}", merge_sort::is_sorted(&data));

    data = merge_sort::sort(&mut data);

    println!("Is sorted after: {}", merge_sort::is_sorted(&data));
}
