use petgraph::{
    graph::{Graph, NodeIndex},
    visit::EdgeRef,
    Direction::{Incoming, Outgoing},
};
use std::{collections::HashMap, fmt, str::Lines};

#[derive(Clone)]
enum FS {
    Dir { name: String },
    File { name: String, size: usize },
}

impl fmt::Display for FS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FS::Dir { name } => write!(f, "Dir{{name: {}}}", name),
            FS::File { name, size } => write!(f, "File{{name: {}, size: {}}}", name, size),
        }
    }
}

impl fmt::Debug for FS {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FS::Dir { name } => write!(f, "Dir{{name: {}}}", name),
            FS::File { name, size } => write!(f, "File{{name: {}, size: {}}}", name, size),
        }
    }
}

pub fn part_one(lines: Lines) {
    let (g, root_node) = build_graph(lines);

    let dir_sizes: HashMap<String, (usize, usize)> = dir_size(&g, root_node);
    let mut sum_size = 0;
    let max_size = 100_000;
    for dir in dir_sizes.values() {
        let size = dir.0 + dir.1;
        if size <= max_size {
            sum_size += size;
        }
    }

    println!("Part One total size: {sum_size}");
}

fn build_graph(mut lines: Lines) -> (Graph<FS, ()>, NodeIndex) {
    // 0. Add the root node
    // 1. For every line...
    //  1. if the statement is `$ cd /` ignore, set current node to root node
    //  2. If the statement is `$ ls`, consume the lines, adding
    //      nodes and edges until we see a `$ cd _` command.
    //      1. If the dir is `..`, move to the incoming node of the current node
    //      2. If we see a 'dir _' and the current node doesn't have an outgoing node with that name, create a new Dir node and add a new outgoing edge between current node and new node. Add a new incoming edge between current node and new node. Set the current node to the new node and goto 1.
    //      3. If we see a file and the current node doesn't have an outoing node with that name, create a new File node and add an edge between the current node and the new node.
    let mut g: Graph<FS, ()> = Graph::new();
    let root_node = g.add_node(FS::Dir {
        name: String::from("/"),
    });

    let mut current_node = root_node;

    while let Some(line) = lines.next() {
        // println!("Processing {line}");
        let action = line.split(" ").collect::<Vec<&str>>();
        match action[..] {
            ["$", "cd", "/"] => {
                current_node = root_node;
            }
            ["$", "cd", ".."] => {
                let edges = g.edges_directed(current_node, Incoming);
                if edges.count() != 1 {
                    panic!("Something went wrong, this node has more than one parent.");
                }
                let parent = g.edges_directed(current_node, Incoming).next().unwrap();

                current_node = parent.source();
                println!(
                    "Moving to directory {}",
                    g.node_weight(current_node).unwrap()
                );
            }
            ["$", "cd", dir] => {
                let mut node_found = false;
                let mut edges = g.edges_directed(current_node, Outgoing);
                while let Some(child) = edges.next() {
                    let node = &*g.node_weight(child.target()).unwrap();
                    match node {
                        FS::Dir { name } => {
                            // This is the child of the current directory, it should be current
                            // node now
                            if name == dir {
                                current_node = child.target();
                                node_found = true;
                                break;
                            }
                        }
                        FS::File { name: _, size: _ } => {
                            continue;
                        }
                    }
                }
                if !node_found {
                    panic!("Could not find directory {dir} :(")
                }
            }
            ["$", "ls"] => {
                // Don't need to do anything, this will just work
                continue;
            }
            ["dir", dir_name] => {
                // Add this as an outgoing node to the current_node
                // I wonder if we need to account for ls'ing the current directory many
                // times...
                let new_node = g.add_node(FS::Dir {
                    name: dir_name.to_string(),
                });
                g.add_edge(current_node, new_node, ());
            }
            [size, file_name] => {
                let parsed_size = size.parse::<usize>().unwrap();
                let new_node = g.add_node(FS::File {
                    name: file_name.to_string(),
                    size: parsed_size,
                });
                g.add_edge(current_node, new_node, ());
            }
            [..] => {
                panic!("Something's messed up");
            }
        }
        // println!("Finished Processing {line}");
    }

    return (g, root_node);
}

fn dir_size(g: &Graph<FS, ()>, dir: NodeIndex) -> HashMap<String, (usize, usize)> {
    return do_dir_size(g, dir, &String::from(""));
}

fn do_dir_size(
    g: &Graph<FS, ()>,
    dir: NodeIndex,
    parent_name: &String,
) -> HashMap<String, (usize, usize)> {
    let this_name = match g.node_weight(dir).unwrap() {
        FS::Dir { name } => name,
        FS::File { name, size: _ } => panic!("File {name} should not be passed to dir_size/2"),
    };
    let mut formatted_this_name = this_name.to_owned();
    match &parent_name.to_owned()[..] {
        "" => {}
        _ => {
            formatted_this_name = format!("{}-{}", parent_name, this_name);
        }
    };
    println!("In dir {this_name}");
    let mut file_children_size = 0;
    let mut total_children_size = 0;
    let mut children: HashMap<String, (usize, usize)> = HashMap::new();
    let mut root_edges = g.edges_directed(dir, Outgoing);
    while let Some(edge) = root_edges.next() {
        let child = &*g.node_weight(edge.target()).unwrap();
        match child {
            FS::Dir { name } => {
                let sub_children = do_dir_size(&g, edge.target().to_owned(), &formatted_this_name);
                for (sub_child_file_size, _sub_child_total_size) in sub_children.values() {
                    total_children_size += sub_child_file_size;
                }
                println!("Processed children for dir {name}, adding {total_children_size}");
                children.extend(sub_children);
            }
            FS::File { name, size } => {
                println!("Adding file {name} with size {size} to dir {this_name}'s size");
                file_children_size += size;
            }
        }
    }
    println!("Dir: {this_name} File Children: {file_children_size}, Total Children: {total_children_size}");
    children.insert(
        formatted_this_name.to_owned(),
        (file_children_size, total_children_size),
    );
    return children;
}
pub fn part_two(lines: Lines) {
    let (g, root_node) = build_graph(lines);

    let dir_sizes: HashMap<String, (usize, usize)> = dir_size(&g, root_node);
    let total_size = 70_000_000;
    let min_free = 30_000_000;
    let root_size = match dir_sizes.get("/") {
        Some(val) => val.0 + val.1,
        None => panic!("/ should exists..."),
    };

    let total_free = total_size - root_size;
    let min_to_free = min_free - total_free;

    let mut actual_to_free = std::usize::MAX;
    let mut dir_name: &String = &String::new();

    for (name, dir) in dir_sizes.iter() {
        let size = dir.0 + dir.1;
        if size > min_to_free && size <= actual_to_free {
            actual_to_free = size;
            dir_name = name;
        }
    }

    println!("Part Two: Delete {dir_name} of size {actual_to_free}");
}
