use crate::periodic_table::print_element_info;

mod periodic_table;

fn main() {
    let mut element_name = std::env::args()
        .nth(1)
        .expect("Please provide a element name");
    let mut element_name_chars: Vec<char> =
        element_name.to_lowercase().chars().into_iter().collect();
    element_name_chars[0] = element_name_chars[0].to_uppercase().nth(0).unwrap();
    element_name = element_name_chars.into_iter().collect();
    print_element_info(&element_name);
}
