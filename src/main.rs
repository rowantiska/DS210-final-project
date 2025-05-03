mod adjacency;
mod plot;
use adjacency::{read_loans, LoanGraph, LoanRecord};
use plot::plot_degree_distribution;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let records = read_loans("src/loan_data.csv")?;
    println!("Using {} loan records", records.len()+1); // index starts at zero so add one to get real len of loan csv data

    let adjacency = LoanGraph::build_from_data(&records);
    let distribution = adjacency.compute_degree_distribution();

    println!("Degree distribution:");
    let mut degrees: Vec<_> = distribution.iter().collect();
    degrees.sort_by_key(|&(deg, _)| *deg);

    for (degree, count) in degrees {
        println!("Degree {}: {} nodes", degree, count);
    }

    plot_degree_distribution(&distribution, "degree_distribution.png")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // check to see if adjacency list is being calculated correctly (build_from_data function)
    fn test_build_graph() {
        let records = vec![
            LoanRecord {
                person_age: Some(25),
                person_gender: "M".into(),
                person_education: "Bachelors".into(),
                person_income: Some(80000.0),
                person_emp_exp: Some(5.0),
                person_home_ownership: "RENT".into(),
                loan_amnt: Some(20000.0),
                loan_intent: "DEBTCONSOLIDATION".into(),
                loan_int_rate: Some(11.5),
                loan_percent_income: Some(0.25),
                cb_person_cred_hist_length: Some(4),
                credit_score: Some(730),
                previous_loan_defaults_on_file: Some(0),
                loan_status: Some(0),
            },
            LoanRecord {
                person_age: Some(32),
                person_gender: "F".into(),
                person_education: "Bachelors".into(), // same education level (should be connected)
                person_income: Some(62000.0),
                person_emp_exp: Some(3.0),
                person_home_ownership: "RENT".into(),
                loan_amnt: Some(5200.0),
                loan_intent: "EDUCATION".into(),
                loan_int_rate: Some(10.50),
                loan_percent_income: Some(0.084),
                cb_person_cred_hist_length: Some(5),
                credit_score: Some(690),
                previous_loan_defaults_on_file: Some(0),
                loan_status: Some(1),
            },
        ];

        let graph = LoanGraph::build_from_data(&records);
        assert_eq!(graph.adjacency_list.len(), 2);
        assert_eq!(graph.adjacency_list[&0].contains(&1), true);
        assert_eq!(graph.adjacency_list[&1].contains(&0), true);
    }

    #[test]
    // check to see if degree distrubtion is calculating correctly (compute_degree_distribution function)
    fn test_degree_distribution() {
        // 3 test nodes, all should be connected
        let mut graph = LoanGraph::new();
        graph.adjacency_list.insert(0, [1, 2].iter().cloned().collect());
        graph.adjacency_list.insert(1, [0, 2].iter().cloned().collect());
        graph.adjacency_list.insert(2, [0, 1].iter().cloned().collect());

        let dist = graph.compute_degree_distribution();
        assert_eq!(dist.len(), 1); // all nodes should have the same degree of connection
        assert_eq!(dist[&2], 3);   // 3 total nodes with 2 degree
    }
}