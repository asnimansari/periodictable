use ascii_table::{Align, AsciiTable};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;

pub static PERIODIC_TABLE_CSV: &'static [u8] = include_bytes!("./periodic_table.json");

#[derive(Deserialize)]
pub struct ElementContents {
    pub atomic_number: u8,
    pub symbol: String,
    pub atomic_mass: f32,
    pub neutron_count: u8,
    pub proton_count: u8,
    pub electron_count: u8,
    pub period: u8,
    pub group: Option<u8>,
    pub phase: String,
    pub radioactive: Option<String>,
    pub natural: Option<String>,
    pub metal: Option<String>,
    pub nonmetal: Option<String>,
    pub metalloid: Option<String>,

    #[serde(rename = "type")]
    pub element_type: Option<String>,
    pub atomic_radius: Option<f32>,
    pub electronegativity: Option<f32>,
    pub first_ionization: Option<f32>,
    pub density: Option<f64>,
    pub melting_point: Option<f32>,
    pub boiling_point: Option<f32>,
    pub number_of_isotopes: Option<u8>,
    pub discoverer: Option<String>,
    pub year: Option<u32>,
    pub specific_heat: Option<f32>,
    pub number_of_shells: Option<u8>,
    pub number_of_valence: Option<u8>,
    pub electronic_configuration: Option<String>,
}

fn convert_option_to_string<T: Display>(option: Option<T>) -> String {
    match option {
        Some(value) => value.to_string(),
        None => "None".to_string(),
    }
}
impl ElementContents {
    pub fn into_printable_format(&self) -> Vec<Vec<String>> {
        let mut result: Vec<Vec<String>> = Vec::new();

        result.push(vec![
            "Atomic Number".to_string(),
            self.atomic_number.to_string(),
        ]);
        result.push(vec!["Symbol".to_string(), self.symbol.clone()]);
        result.push(vec![
            "Atomic Mass".to_string(),
            self.atomic_mass.to_string(),
        ]);
        result.push(vec![
            "Neutron Count".to_string(),
            self.neutron_count.to_string(),
        ]);
        result.push(vec![
            "Proton Count".to_string(),
            self.proton_count.to_string(),
        ]);
        result.push(vec![
            "Electron Count".to_string(),
            self.electron_count.to_string(),
        ]);
        result.push(vec!["Period".to_string(), self.period.to_string()]);
        result.push(vec![
            "Group".to_string(),
            convert_option_to_string(self.group),
        ]);
        result.push(vec!["Phase".to_string(), self.phase.clone()]);
        result.push(vec![
            "Radioactive".to_string(),
            convert_option_to_string(self.radioactive.clone()),
        ]);
        result.push(vec![
            "Natural".to_string(),
            convert_option_to_string(self.natural.clone()),
        ]);
        result.push(vec![
            "Metal".to_string(),
            convert_option_to_string(self.metal.clone()),
        ]);
        result.push(vec![
            "Nonmetal".to_string(),
            convert_option_to_string(self.nonmetal.clone()),
        ]);
        result.push(vec![
            "Metalloid".to_string(),
            convert_option_to_string(self.metalloid.clone()),
        ]);
        result.push(vec![
            "Element Type".to_string(),
            convert_option_to_string(self.element_type.clone()),
        ]);
        result.push(vec![
            "Atomic Radius".to_string(),
            convert_option_to_string(self.atomic_radius),
        ]);
        result.push(vec![
            "Electronegativity".to_string(),
            convert_option_to_string(self.electronegativity),
        ]);
        result.push(vec![
            "First Ionization".to_string(),
            convert_option_to_string(self.first_ionization),
        ]);
        result.push(vec![
            "Density".to_string(),
            convert_option_to_string(self.density),
        ]);
        result.push(vec![
            "Melting Point".to_string(),
            convert_option_to_string(self.melting_point),
        ]);

        result.push(vec![
            "Boiling Point".to_string(),
            convert_option_to_string(self.boiling_point),
        ]);
        result.push(vec![
            "Number of Isotopes".to_string(),
            convert_option_to_string(self.number_of_isotopes),
        ]);
        result.push(vec![
            "Discoverer".to_string(),
            convert_option_to_string(self.discoverer.clone()),
        ]);
        result.push(vec![
            "Year".to_string(),
            convert_option_to_string(self.year),
        ]);

        result.push(vec![
            "Specific Heat".to_string(),
            convert_option_to_string(self.specific_heat),
        ]);
        result.push(vec![
            "Number of Shells".to_string(),
            convert_option_to_string(self.number_of_shells),
        ]);

        result.push(vec![
            "Number of Valence".to_string(),
            convert_option_to_string(self.number_of_valence),
        ]);
        result.push(vec![
            "Electronic Configuration".to_string(),
            convert_option_to_string(self.electronic_configuration.clone()),
        ]);
        result
    }
}

fn load_json_to_struct() -> HashMap<String, ElementContents> {
    let raw_contents = PERIODIC_TABLE_CSV;

    let deserialized_data: HashMap<String, ElementContents> =
        serde_json::from_slice(raw_contents).expect("failed to deserialize element info file");
    return deserialized_data;
}

fn _print_element_info(element_info: &ElementContents) {
    let mut ascii_table = AsciiTable::default();
    ascii_table.set_max_width(80);

    let data: Vec<Vec<String>> = element_info.into_printable_format();
    ascii_table.print(data);
}

pub fn print_element_info(element_name: &str) {
    let all_element_info = load_json_to_struct();
    let request_element_info = all_element_info.get(element_name);
    match request_element_info {
        Some(element_info) => {
            _print_element_info(&element_info);
        }
        None => {
            println!("No element named: {} found", element_name);
        }
    }
}
