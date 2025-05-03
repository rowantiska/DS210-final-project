// module purpose: seperate the degree distrubion functions from the plotters functions -> cleaner orgization (pub/private does not matter for my project so everything is public)
use std::collections::{HashMap, HashSet};
use std::error::Error;
use csv::{Reader, StringRecord};

// struct to store each loan record
#[derive(Debug)]
pub struct LoanRecord {
    pub person_age: Option<u32>,
    pub person_gender: String,
    pub person_education: String,
    pub person_income: Option<f32>,
    pub person_emp_exp: Option<f32>,
    pub person_home_ownership: String,
    pub loan_amnt: Option<f32>,
    pub loan_intent: String,
    pub loan_int_rate: Option<f32>,
    pub loan_percent_income: Option<f32>,
    pub cb_person_cred_hist_length: Option<u32>,
    pub credit_score: Option<u32>,
    pub previous_loan_defaults_on_file: Option<u8>,
    pub loan_status: Option<u8>,
}

// create struct to hold the adj list (hashmap)
#[derive(Debug)]
pub struct LoanGraph {
    pub adjacency_list: HashMap<u32, HashSet<u32>>,
}

impl LoanGraph {
    // create new adj hashmap instance, returns the hashmap
    pub fn new() -> Self {
        LoanGraph {
            adjacency_list: HashMap::new(),
        }
    }
    // creates edges from the loan dataset, 
    // input =  adj list from the LoanGraph struct, 
    // output = a new LoanGraph that has the edges of the data
    // build_from_data uses a double for loop with .iter() and .enumerate() to create a node and then an edge to that node, 
    // and it does this for every peice of data. Then inserts the data into the LoanGraph datatype
    pub fn build_from_data(data: &[LoanRecord]) -> Self {
        let mut graph = LoanGraph::new();

        for (i, a) in data.iter().enumerate() {
            for (j, b) in data.iter().enumerate().skip(i + 1) {
                //if person (A&B) has same education or person (A&B) same loan intent -> add to adj list
                if a.person_education == b.person_education || a.loan_intent == b.loan_intent {
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
        return graph;
    }

    // calculates the degree distribution from the LoanGraph graph
    // input = self (associated with LoanGraph impl)
    // output = distribution hashmap (returns)
    // the function finds the degree distribution by looping through all the values in  the adjacency_list and counting how
    // many neighbors the entry or each item has
    pub fn compute_degree_distribution(&self) -> HashMap<usize, usize> {
        let mut distribution = HashMap::new();

        for neighbors in self.adjacency_list.values() {
            let degree = neighbors.len();
            *distribution.entry(degree).or_insert(0) += 1;
        }

        return distribution;
    }
}

// this function just reads the csv data file and creates a vec from it
// input = file path to CSV data
// output = data vector
// the function matches the LoanRecord struct from parse_record function and if it matches the same data length/entries,
// then it pushes to the data vector
pub fn read_loans(path: &str) -> Result<Vec<LoanRecord>, Box<dyn Error>> {
    let mut rdr = Reader::from_path(path)?;
    let mut data = Vec::new();

    for (i, result) in rdr.records().enumerate() {
        let record = result?;
        match parse_record(&record) {
            Some(loan) => data.push(loan),
            None => {
                eprintln!("Skipping malformed row at line {}", i + 2);
            }
        }
    }
    Ok(data)
}

// parses each peice of data from the csv and gives the correct data type
// input =  data from csv file (record)
// output = Some(LoanRecord) if match is successful, None if fail
// parse_record is used within the read_loans function to match each row to make sure each peice of data is present,
// if the data does not match the LoanRecord struct format, then it is ignored; it's kinda like a kind of data cleaning
fn parse_record(record: &StringRecord) -> Option<LoanRecord> {
    Some(LoanRecord {
        person_age: record.get(0)?.parse().ok(),
        person_gender: record.get(1)?.to_string(),
        person_education: record.get(2)?.to_string(),
        person_income: record.get(3)?.parse().ok(),
        person_emp_exp: record.get(4)?.parse().ok(),
        person_home_ownership: record.get(5)?.to_string(),
        loan_amnt: record.get(6)?.parse().ok(),
        loan_intent: record.get(7)?.to_string(),
        loan_int_rate: record.get(8)?.parse().ok(),
        loan_percent_income: record.get(9)?.parse().ok(),
        cb_person_cred_hist_length: record.get(10)?.parse().ok(),
        credit_score: record.get(11)?.parse().ok(),
        previous_loan_defaults_on_file: record.get(12)?.parse().ok(),
        loan_status: record.get(13)?.parse().ok(),
    })
}