use defaultdict::DefaultHashMap;
use petgraph::visit::IntoNeighbors;

use crate::graph_p::Edge;
use crate::graph_p::Graph;
use crate::graph_p::Node;
use core::cmp::min;

pub fn articulation_point_helper(
    graph: &Graph,
    node_idx: String,
    visited: &mut DefaultHashMap<String, bool>,
    disc: &mut DefaultHashMap<String, i32>,
    low: &mut DefaultHashMap<String, i32>,
    time: &mut usize,
    parent: String,
    ap: &mut DefaultHashMap<String, bool>,
) {
    let mut children = 0;
    visited.insert(node_idx.clone(), true);

    *time += 1;
    disc.insert(node_idx.clone(), *time as i32);
    low.insert(node_idx.clone(), *time as i32);

    let neighbors = graph.get_neighbors(node_idx.clone());
    for neighbor in neighbors {
        if !visited.get(&neighbor.name) {
            children += 1;
            articulation_point_helper(
                &graph,
                neighbor.name.clone(),
                visited,
                disc,
                low,
                time,
                node_idx.clone(),
                ap,
            );
            low.insert(
                node_idx.clone(),
                min(*low.get(&node_idx), *low.get(&neighbor.name)),
            );

            if !parent.eq("-1") && low.get(&neighbor.name) >= disc.get(&node_idx) {
                ap.insert(node_idx.clone(), true);
            }
        } else if !neighbor.name.eq(&parent) {
            low.insert(
                node_idx.clone(),
                min(*low.get(&node_idx), *disc.get(&neighbor.name)),
            );
        }
    }

    if parent.eq("-1") && children > 1 {
        ap.insert(node_idx.clone(), true);
    }
}

pub fn articulation_point(graph: Graph) -> Vec<petgraph::graph::NodeIndex> {
    // hashmap
    let mut disc = DefaultHashMap::<String, i32>::new();
    let mut low = DefaultHashMap::<String, i32>::new();
    let mut visited = DefaultHashMap::<String, bool>::new();
    let mut ap = DefaultHashMap::<String, bool>::new();
    let mut time = 0;
    let parent = "-1".to_string();

    for node in graph.get_nodes() {
        if !visited.get(&node.name) {
            articulation_point_helper(
                &graph,
                node.name,
                &mut visited,
                &mut disc,
                &mut low,
                &mut time,
                parent.clone(),
                &mut ap,
            );
        }
    }

    let mut articulation_points = Vec::new();
    for key in ap.keys() {
        if *ap.get(key) {
            articulation_points.push(graph.get_node_idx(key.to_string()));
        }
    }
    return articulation_points;
}

// Create a unit test for the Graph struct
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialize_graph() {
        // Create a graph
        let mut G = Graph::new();

        // Add nodes
        let u: String = "u".to_string();
        let v: String = "v".to_string();
        let w: String = "w".to_string();
        let x: String = "x".to_string();
        let y: String = "y".to_string();

        G.add_node(u.clone());
        G.add_node(v.clone());
        G.add_node(w.clone());
        G.add_node(x.clone());
        G.add_node(y.clone());

        // Add edges
        G.add_edge(u.clone(), v.clone(), 1.0);
        G.add_edge(u.clone(), w.clone(), 1.0);
        G.add_edge(u.clone(), x.clone(), 1.0);
        G.add_edge(w.clone(), x.clone(), 1.0);
        G.add_edge(v.clone(), y.clone(), 1.0);
        G.add_edge(x.clone(), y.clone(), 1.0);

        println!("\n");

        // Test the articulation point function
        let articulation_points = articulation_point(G.clone());
        let len_articulation_points = articulation_points.len();
        for node in articulation_points {
            let node = G.g.node_weight(node).unwrap();
            println!("Articulation Point: {}", node.name);
        }
        assert_eq!(len_articulation_points, 0);

        let mut G_2 = Graph::new();
        let v1: String = "1".to_string();
        let v2: String = "2".to_string();
        let v3: String = "3".to_string();
        let v4: String = "4".to_string();
        let v5: String = "5".to_string();

        G_2.add_node(v1.clone());
        G_2.add_node(v2.clone());
        G_2.add_node(v3.clone());
        G_2.add_node(v4.clone());
        G_2.add_node(v5.clone());

        G_2.add_edge(v2.clone(), v1.clone(), 1.0);
        G_2.add_edge(v1.clone(), v3.clone(), 1.0);
        G_2.add_edge(v3.clone(), v2.clone(), 1.0);
        G_2.add_edge(v1.clone(), v4.clone(), 1.0);
        G_2.add_edge(v4.clone(), v5.clone(), 1.0);

        let articulation_points = articulation_point(G_2.clone());
        let len_articulation_points = articulation_points.len();
        for node in articulation_points {
            let node = G_2.g.node_weight(node).unwrap();
            println!("Articulation Point: {}", node.name);
        }

        assert_eq!(len_articulation_points, 2);
    }
}
