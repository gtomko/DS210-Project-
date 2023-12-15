extern crate csv;
use csv::{ReaderBuilder, StringRecord};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug)] //create a struct to put the casts into
struct Data {
    pub cast: Vec<String>, //will be a list of the cast put into a vector
}
type Vertex = usize; 
type ListOfEdges = Vec<(Vertex,Vertex)>;

fn create_list(entry: &Data, index: &mut HashMap<String, usize>, 
        actors: &mut usize, adjacency_list: &mut HashMap<usize, Vec<usize>>) -> ListOfEdges { //will be used when the file is opened to get a vector of a vector
    let num_cast: Vec<usize> = entry
        .cast
        .iter()
        .map(|name| {
            *index
                .entry(name.to_string())// takes the cast and converts names to a certain value
                .or_insert_with(|| {
                    *actors += 1; // increasing by one each time
                    *actors - 1
                })
        })
        .collect();    
    let mut edges = Vec::new(); // will store the edges in here 
    for &actor_id in &num_cast {
        for &other_actor_id in &num_cast {
            if actor_id != other_actor_id {
                edges.push((actor_id, other_actor_id));  // makes a list of edges between actors
                let entry = adjacency_list.entry(actor_id).or_insert(Vec::new());
                entry.push(other_actor_id); 
            }
        }
    }
    edges 
}

pub fn open_file(csv_file: &str) -> (ListOfEdges, usize) {
    let file = File::open(csv_file).expect("Couldn't open csv file");
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(BufReader::new(file));
    let mut actors = 0; // will give the number total number of actors
    let mut name_to_index: HashMap<String, usize> = HashMap::new(); //needed to run create list by storing the name and converting an index
    let mut adjacency_list: HashMap<usize, Vec<usize>> = HashMap::new(); //needed to run create list by storing the actor id and all their connections
    let mut edges = Vec::new(); 
    for result in rdr.records() {
        let record: StringRecord = result.expect("Couldn't read file");
        let entry = Data {
            cast: record[4] //only care about this line of data which holds the cast
                .trim()
                .split(',')
                .map(|s| s.to_string())
                .collect(), // opens and collects the data for the actors
        };
        let edge = create_list(&entry, &mut name_to_index, &mut actors, &mut adjacency_list);
        edges.extend(edge); //appends the edge into the vector
    }
    (edges, actors)
}


