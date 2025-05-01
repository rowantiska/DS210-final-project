use std::collections::{HashMap, HashSet};
use std::error::Error;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct LoanRecord {
    person_age: Option<u32>,
    person_gender: String,
    person_education: String,
    person_income: Option<f32>,
    person_emp_exp: Option<f32>,
    person_home_ownership: String,
    loan_amnt: Option<f32>,
    loan_intent: String,
    loan_int_rate: Option<f32>,
    loan_percent_income: Option<f32>,
    cb_person_cred_hist_length: Option<u32>,
    credit_score: Option<u32>,
    previous_loan_defaults_on_file: Option<u8>,
    loan_status: Option<u8>,
}

#[derive(Debug)]
struct LoanGraph {
    adjacency_list: HashMap<u32, HashSet<u32>>, // internal id -> neighbors
}

impl LoanGraph {
    fn new() -> Self {
        LoanGraph {
            adjacency_list: HashMap::new(),
        }
    }

    fn build_from_data(data: &[LoanRecord]) -> Self {
        let mut graph = LoanGraph::new();

        // Create edges between similar people (e.g., shared education or loan intent)
        for (i, a) in data.iter().enumerate() {
            for (j, b) in data.iter().enumerate().skip(i + 1) {
                if a.person_education == b.person_education
                    || a.loan_intent == b.loan_intent
                {
                    graph.adjacency_list
                        .entry(i as u32)
                        .or_insert_with(HashSet::new)
                        .insert(j as u32);
                    graph.adjacency_list
                        .entry(j as u32)
                        .or_insert_with(HashSet::new)
                        .insert(i as u32);
                }
            }
        }

        graph
    }

    fn compute_degree_distribution(&self) -> HashMap<usize, usize> {
        let mut distribution: HashMap<usize, usize> = HashMap::new();

        for neighbors in self.adjacency_list.values() {
            let degree = neighbors.len();
            *distribution.entry(degree).or_insert(0) += 1;
        }

        distribution
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the CSV file
    let mut rdr = csv::Reader::from_path("src/loan_data.csv")?;
    let mut records: Vec<LoanRecord> = Vec::new();

    for result in rdr.deserialize() {
        let record: LoanRecord = result?;
        records.push(record);
    }

    // Filter out rows with missing essential data
    records.retain(|r| {
        r.person_age.is_some()
            && r.person_income.is_some()
            && r.loan_amnt.is_some()
            && !r.person_education.is_empty()
            && !r.loan_intent.is_empty()
    });

    let graph = LoanGraph::build_from_data(&records);
    let distribution = graph.compute_degree_distribution();

    println!("Degree Distribution:");
    let mut degrees: Vec<_> = distribution.iter().collect();
    degrees.sort_by_key(|&(deg, _)| *deg);

    for (degree, count) in degrees {
        println!("Degree {}: {} node(s)", degree, count);
    }

    Ok(())
}