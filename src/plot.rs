// module purpose: seperate the plotters functions from the degree distribution so that the output of DD can be passed into this module.
use std::collections::HashMap;
use std::error::Error;
use plotters::prelude::*;

// this function creates a plot given the distribution data from the adjacency module
// input = distribution data and an output file path for the graph png
// output = picture of a graph (png file)
// this function uses the rust package 'plotters' to create a simple dot plot of the node distribution
pub fn plot_degree_distribution(distribution: &HashMap<usize, usize>, output_path: &str) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::new(output_path, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut degrees: Vec<(usize, usize)> = distribution.iter().map(|(d, c)| (*d, *c)).collect();
    degrees.sort_by_key(|(d, _)| *d);

    let max_degree = degrees.iter().map(|(d, _)| *d).max().unwrap_or(0);
    let max_count = degrees.iter().map(|(_, c)| *c).max().unwrap_or(0);

    let mut chart = ChartBuilder::on(&root)
        .caption("Degree distribution of loan data", ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .build_cartesian_2d(0..max_degree, 0..max_count)?;

    chart.configure_mesh()
        .x_desc("Degree")
        .y_desc("# of nodes")
        .draw()?;

    let _ = chart.draw_series(
        degrees.into_iter().map(|(degree, count)| {
            Circle::new((degree, count), 4, RED.filled())
        })
    );

    root.present()?;
    Ok(())
}