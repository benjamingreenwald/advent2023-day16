use std::fs;
use std::collections::{HashMap, HashSet, VecDeque};
use petgraph::graph::{DiGraph,NodeIndex};
use petgraph::visit::Dfs;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum DirectionOfMotion {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

fn next_nodes(grid: &Vec<Vec<char>>, node: &(usize, usize,  DirectionOfMotion)) -> Vec<(usize, usize, DirectionOfMotion)> {
    let current_node = grid[node.1][node.0];
    let x = node.0 as i32;
    let y = node.1 as i32;
    let dir = &node.2;
    let ixlen = grid[0].len() as i32;
    let iylen = grid.len() as i32;

    let result = match dir {
        DirectionOfMotion::LEFT => {
            match current_node {
                '/' => {
                    vec![(x, y + 1, DirectionOfMotion::DOWN)]
                },
                '\\' => {
                    if y > 0 {
                        vec![(x, y - 1, DirectionOfMotion::UP)]
                    } else {
                        vec![]
                    }
                },
                '-' => {
                    vec![(x-1, y, DirectionOfMotion::LEFT)]
                },
                '|' => {
                    vec![(x, y-1, DirectionOfMotion::UP), (x, y+1, DirectionOfMotion::DOWN)]
                },
                '.' => {
                    vec![(x-1, y, DirectionOfMotion::LEFT)]
                },
                _ => unreachable!()
            }
        },
        DirectionOfMotion::RIGHT => {
            match current_node {
                '/' => {
                    vec![(x, y-1, DirectionOfMotion::UP)]
                },
                '\\' => {
                    vec![(x, y+1, DirectionOfMotion::DOWN)]
                },
                '-' => {
                    vec![(x+1, y, DirectionOfMotion::RIGHT)]
                },
                '|' => {
                    vec![(x, y-1, DirectionOfMotion::UP), (x, y+1, DirectionOfMotion::DOWN)]
                },
                '.' => {
                    vec![(x+1, y, DirectionOfMotion::RIGHT)]
                },
                _ => unreachable!()
            }
        },
        DirectionOfMotion::UP => {
            match current_node {
                '/' => {
                    vec![(x+1, y, DirectionOfMotion::RIGHT)]
                },
                '\\' => {
                    vec![(x-1, y, DirectionOfMotion::LEFT)]
                },
                '-' => {
                    vec![(x-1, y, DirectionOfMotion::LEFT), (x+1, y, DirectionOfMotion::RIGHT)]
                },
                '|' => {
                    vec![(x, y-1, DirectionOfMotion::UP)]
                },
                '.' => {
                    vec![(x, y-1, DirectionOfMotion::UP)]
                },
                _ => unreachable!()
            }
        },
        DirectionOfMotion::DOWN => {
            match current_node {
                '/' => {
                    vec![(x-1, y, DirectionOfMotion::LEFT)]
                },
                '\\' => {
                    vec![(x+1, y, DirectionOfMotion::RIGHT)]
                },
                '-' => {
                    vec![(x-1, y, DirectionOfMotion::LEFT), (x+1, y, DirectionOfMotion::RIGHT)]
                },
                '|' => {
                    vec![(x, y+1, DirectionOfMotion::DOWN)]
                },
                '.' => {
                    vec![(x, y+1, DirectionOfMotion::DOWN)]
                },
                _ => unreachable!()
            }
        },
    };
    result.into_iter().filter(|(ix,iy,_)| {
        (*iy >= 0) && (*iy < iylen) && (*ix >= 0) && (*ix < ixlen)
    }).map(|(ix, iy, dir)| {
        (ix as usize, iy as usize, dir)
    }).collect()
}

fn get_node_index(node: &(usize,usize,DirectionOfMotion), graph: &mut DiGraph<(usize,usize,DirectionOfMotion),()>, node_mapping: &mut HashMap<(usize,usize,DirectionOfMotion), NodeIndex>) -> NodeIndex {
    if let Some(ndx) = node_mapping.get(&node) {
        *ndx
    } else {
        let ndx = graph.add_node(node.clone());
        node_mapping.insert((*node).clone(), ndx);
        ndx
    }
}

pub fn day16() {
    let unparsed_file = fs::read_to_string("input-16.txt").expect("can't read input");
    let grid: Vec <Vec<char> > = unparsed_file.lines().map( | line | {
        line.chars().collect()
    }).collect();

    let entry_nodes: Vec<(usize,usize,DirectionOfMotion)> = (0..grid[0].len()).into_iter().map(|x| {
        (x,0,DirectionOfMotion::DOWN)
    }).chain((0..grid[0].len()).into_iter().map(|x| {
        (x,grid.len()-1, DirectionOfMotion::UP)
    })).chain((0..grid.len()).into_iter().map(|y| {
        (0, y, DirectionOfMotion::RIGHT)
    })).chain((0..grid.len()).into_iter().map(|y|{
        (grid[0].len()-1, y, DirectionOfMotion::LEFT)
    })).collect();

    let mut graph: DiGraph<(usize,usize,DirectionOfMotion),()> = DiGraph::default();
    let mut visited = HashSet::new();
    let mut node_mapping = HashMap::new();
    let mut worklist: VecDeque<(usize,usize,DirectionOfMotion)> = VecDeque::new();
    for n in &entry_nodes {
        worklist.push_back(n.clone());
    }

    while let Some(current) = worklist.pop_front() {
        if visited.contains(&current) {
            continue;
        }
        visited.insert(current.clone());

        let node_ndx = get_node_index(&current, &mut graph, &mut node_mapping);
        for next_node in next_nodes(&grid, &current) {
            let next_node_ndx = get_node_index(&next_node, &mut graph, &mut node_mapping);
            graph.update_edge(node_ndx, next_node_ndx, ());
            worklist.push_back(next_node);
        }
    }

    let mut scc_graph: DiGraph<usize, ()> = DiGraph::default();
    let mut node_to_scc_mapping = HashMap::new();
    let sccs = petgraph::algo::kosaraju_scc(&graph);
    for (i, sccelem) in sccs.iter().enumerate() {
        let scc_node_ndx = scc_graph.add_node(i);
        for ndx in sccelem {
            node_to_scc_mapping.insert(ndx, scc_node_ndx);
        }
    }

    for e in graph.raw_edges() {
        let scc_source = node_to_scc_mapping.get(&e.source()).unwrap();
        let scc_dest = node_to_scc_mapping.get(&e.target()).unwrap();
        scc_graph.update_edge(*scc_source, *scc_dest, ());
    }

    let result: usize = entry_nodes.iter().map(|n| {
        let entry_node_ndx = get_node_index(n, &mut graph, &mut node_mapping);
        let entry_scc_ndx = node_to_scc_mapping.get(&entry_node_ndx).unwrap();
        let mut dfs = Dfs::new(&scc_graph, *entry_scc_ndx);
        let mut found_locations = HashSet::new();
        while let Some(scc_ndx) = dfs.next(&scc_graph) {
            let scc_index = scc_graph.node_weight(scc_ndx).unwrap();
            for node in &sccs[*scc_index] {
                let (x, y, _) = graph.node_weight(*node).unwrap();
                found_locations.insert((*x, *y));
            }
        }
        found_locations.len()
    }).max().unwrap();

    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        day16();
    }
}
