use std::collections::VecDeque;
pub fn distance_vector(start: usize, graph: &super::Graph)-> Vec<Option<u32>>{ //used to compute bfs
    let mut distance: Vec<Option<u32>> = vec![None; graph.n];
    distance[start] = Some(0); //starting distance to itslef is always zero
    let mut queue: VecDeque<usize> = VecDeque::new();
    queue.push_back(start); 
    while let Some(v) = queue.pop_front() {
        for &u in graph.outedges[v].iter() {
            if let None = distance[u]{
                distance[u] = Some(distance[v].unwrap() + 1 );
                queue.push_back(u);
            } 
        }
    }
    distance //vector of the options, Some(distances) / None  of the distances from the starting point 
}

pub fn total_distance(dist_vec: Vec<Option<u32>>, nodes: usize) -> f64{ //calculates the total distances from the distance vector
    let mut total_dist = 0.0;
    for v in 0..nodes {
        let dist = dist_vec[v].unwrap_or(0) as f64; //unwraps the values from the vecotr
        total_dist += dist
    } 
    total_dist 
}