use crate::periodic_table::print_element_info;

mod periodic_table;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        println!("Please provide an element name, Usage: periodictable <element name>");
        return;
    }

    let mut element_name = args
        .into_iter()
        .nth(1)
        .expect("Please provide a element name");
    let mut element_name_chars: Vec<char> =
        element_name.to_lowercase().chars().into_iter().collect();
    element_name_chars[0] = element_name_chars[0].to_uppercase().nth(0).unwrap();
    element_name = element_name_chars.into_iter().collect();
    print_element_info(&element_name);
}
