pub type Vertex = usize; 
pub type ListOfEdges = Vec<(Vertex,Vertex)>;
pub type AdjacencyLists = Vec<Vec<Vertex>>; //create all these types to make it consistent and easy to change type when needed

#[derive(Debug)] //create a struct that will be used to create the undirected graph
pub struct Graph {  
    pub n: usize,
    pub outedges: AdjacencyLists,
}

pub fn reverse_edges(list:&ListOfEdges)-> ListOfEdges { //takes directed edges and makes them undirectd
    let mut new_list = vec![]; 
    for (u,v) in list {
        new_list.push((*v,*u)); //switches the position in the tuple
    }
    new_list
}


impl Graph {
    pub fn sort_graph_lists(&mut self) { //needed to create the undirected graph
        for l in self.outedges.iter_mut() {
            l.sort();
        }
    }
    pub fn add_edge(&mut self, edges: &ListOfEdges) { //adds an edge so we can create the graph
        for (u, v) in edges {
            self.outedges[*u].push(*v); //makes a directed connection of u and v
        }
    }
    pub fn create_directed(n: usize, edges: &ListOfEdges ) -> Graph {//creates a directed that is used to make the undirected graph
        let mut g = Graph {
            n,
            outedges: vec![vec![]; n],
        };
        g.add_edge(edges);
        g.sort_graph_lists();
        g
    }
    pub fn create_undirected(n: usize, edges: &ListOfEdges ) -> Graph { //creates the undirected graph that will be used to calculate the degrees of seperation
        let mut g = Self::create_directed(n, edges); //creates a directed graph
        g.add_edge(&reverse_edges(edges)); //then simply reverses the edges to make it undirected+
        g.sort_graph_lists();
        g
    }
    pub fn conn_vec(&self) -> Vec<f64> { // I was also curious about the average connections an actor has so I create this to find out
        let mut averages = Vec::with_capacity(self.n);
        for adj_list in &self.outedges{
            let average = adj_list.len() as f64;
            averages.push(average);
        }
    averages
    }       
}
