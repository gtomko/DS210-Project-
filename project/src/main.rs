mod distance;
mod graph;
mod setup;
use graph::{Graph};
use setup::{open_file};
use distance::{distance_vector, total_distance};
#[test]
fn no_connect(){
    let tuples = vec![(0,0),(1,1),(2,2)];
    let graph = Graph::create_undirected(3, &tuples);
    let dist_vector = dist_calc::distance_vector(0, &graph);
    let total_dist = dist_calc::total_distance(dist_vector, 3);
    let six_degrees_number = total_dist / 2.0;
    assert_eq!(six_degrees_number, 0.0, "Does not total 0");
}

#[test]
fn all_connect(){
    let tuples = vec![(0,1),(1,2),(2,0)];
    let graph = Graph::create_undirected(3, &tuples);
    let dist_vector = dist_calc::distance_vector(0, &graph);
    let total_dist = dist_calc::total_distance(dist_vector, 3);
    let six_degrees_number = total_dist / 2.0 ;
    assert_eq!(six_degrees_number, 1.0, "Does not total 1");
}

#[test]
fn always_pos(){
    let tuples = vec![(0,2),(1,3),(2,0),(4,5),(1,5),(2,4)];
    let graph = Graph::create_undirected(6, &tuples);
    let dist_vector = dist_calc::distance_vector(0, &graph);
    let total_dist = dist_calc::total_distance(dist_vector, 6);
    let six_degrees_number = total_dist / 5.0 ;
    assert!(six_degrees_number >= 0.0, "Less than 0")
}
#[test]
fn one_connect(){
    let tuples = vec![(0,1)];
    let graph = Graph::create_undirected(2, &tuples);
    let dist_vector = dist_calc::distance_vector(0, &graph);
    let total_dist = dist_calc::total_distance(dist_vector, 2);
    let six_degrees_number = total_dist / 1.0 ;
    println!("{}", six_degrees_number);
    assert!(six_degrees_number >= 1.0, "Less than 0") 
}

fn main() {
    let file = "netflix_titles.csv";
    let (edges, n) = open_file(file); //returns avector of the edges and the number of nodes
    let graph = Graph::create_undirected(n, &edges); 
    println!("Number of Nodes in Graph: {}", n);
    let dist_vector = distance_vector(0, &graph); //calulates the distances from zero 
    let total_dist = total_distance(dist_vector, n); //totals them
    println!("Total Degrees of Seperation: {}", total_dist);
    let six_degrees_number = total_dist / ((n-1) as f64); //get rid of the connection between itself
    println!("Average Degrees of Seperation: {}", six_degrees_number);
    let connections = (&graph).conn_vec();
    let mut total_connect = 0.0;
    for i in &connections{
        total_connect += i //totals the connections in the graph
    }
    let average_connect = total_connect/ (n  as f64); 
    println!("Average amount of connections: {:?}", average_connect);
}

