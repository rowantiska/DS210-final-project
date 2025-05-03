mod adjacency;
mod plot;
use adjacency::{read_loans, LoanGraph};
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
    fn test_build_graph_connects_shared_education() {
        let records = vec![
            LoanRecord {
                person_age: Some(30),
                person_gender: "M".into(),
                person_education: "Bachelors".into(),
                person_income: Some(50000.0),
                person_emp_exp: Some(5.0),
                person_home_ownership: "RENT".into(),
                loan_amnt: Some(2000.0),
                loan_intent: "DEBTCONSOLIDATION".into(),
                loan_int_rate: Some(13.5),
                loan_percent_income: Some(0.04),
                cb_person_cred_hist_length: Some(4),
                credit_score: Some(700),
                previous_loan_defaults_on_file: Some(0),
                loan_status: Some(0),
            },
            LoanRecord {
                person_age: Some(28),
                person_gender: "F".into(),
                person_education: "Bachelors".into(), // same education
                person_income: Some(40000.0),
                person_emp_exp: Some(3.0),
                person_home_ownership: "MORTGAGE".into(),
                loan_amnt: Some(3000.0),
                loan_intent: "EDUCATION".into(),
                loan_int_rate: Some(12.0),
                loan_percent_income: Some(0.075),
                cb_person_cred_hist_length: Some(5),
                credit_score: Some(720),
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
    fn test_degree_distribution_correctness() {
        // 3 nodes all connected to each other (triangle)
        let mut graph = LoanGraph::new();
        graph.adjacency_list.insert(0, [1, 2].iter().cloned().collect());
        graph.adjacency_list.insert(1, [0, 2].iter().cloned().collect());
        graph.adjacency_list.insert(2, [0, 1].iter().cloned().collect());

        let dist = graph.compute_degree_distribution();
        assert_eq!(dist.len(), 1); // All nodes have same degree
        assert_eq!(dist[&2], 3);   // 3 nodes with degree 2
    }
}