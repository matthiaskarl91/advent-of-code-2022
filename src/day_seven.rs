use std::fs;

#[derive(Debug)]
struct Node<T>
where
    T: PartialEq,
{
    idx: usize,
    val: u32,
    name: T,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl<T> Node<T>
where
    T: PartialEq,
{
    fn new(idx: usize, name: T, val: u32) -> Self {
        Self {
            idx,
            val,
            name,
            parent: None,
            children: vec![],
        }
    }
}

#[derive(Debug, Default)]
struct Tree<T>
where
    T: PartialEq,
{
    arena: Vec<Node<T>>,
}

impl<T> Tree<T>
where
    T: PartialEq,
{
    fn node(&mut self, name: T, val: u32) -> usize {
        //first see if it exists
        for node in &self.arena {
            if node.name == name {
                return node.idx;
            }
        }
        // Otherwise, add new node
        let idx = self.arena.len();
        self.arena.push(Node::new(idx, name, val));
        idx
    }
}

pub fn execute() {
    let input = fs::read_to_string("day_seven.txt").unwrap();
    let mut lines: Vec<&str> = input.split("\n").collect();
    lines.reverse();

    let mut tree = Tree::default();
    let root = tree.node("/", 0);
    let mut current_dir: Vec<&str> = vec!["/"];
    while let Some(line) = lines.pop() {
        // println!("{:?}", line);
        if line.contains("$ cd ") {
            let mut dir_command: Vec<&str> = line.split("cd ").collect();
            if let Some(dir) = dir_command.pop() {
                // println!("{}", dir);
                if dir.contains("..") {
                    current_dir.pop();
                } else {
                    current_dir.push(dir);
                }
            };
        } else if line.contains("dir") {
            let mut dir_information: Vec<&str> = line.split("dir ").collect();
            if let Some(dir_name) = dir_information.pop() {
                let node = tree.node(dir_name, 0);
                let parent_node = tree.node(current_dir.last().unwrap(), 0);
                tree.arena[node].parent = Some(parent_node);
                tree.arena[parent_node].children.push(node);
            }
        } else if line.contains("$ ls") {
        } else {
            if let Some((size, name)) = line.split_once(" ") {
                // println!("{}: {}", name, size);
                let size_number: u32 = size.parse::<u32>().unwrap();
                let parent_node = tree.node(current_dir.last().unwrap(), 0);
                let mut name_dir = tree.arena[parent_node].name;
                let n = format!("{}{}", name_dir, name).clone();
                let node = tree.node(n.as_str(), size_number);
                tree.arena[node].parent = Some(parent_node);
                tree.arena[parent_node].children.push(node);
            };
        }
    }

    // for node in &tree.arena {
    //     if node.val == 0 {
    //         // let mut sum: u32 = get_sum_from_node(&tree, node.idx);
    //         let mut sum: u32 = node.val;
    //         let sum_children: u32 = node
    //             .children
    //             .iter()
    //             .map(|&child_idx| {
    //                 let child = &tree.arena[child_idx];
    //                 child.val
    //             })
    //             .sum();
    //         // println!("{:?}", sum + sum_children);
    //         sum += sum_children;
    //         println!("{}: {}", node.name, sum);
    //     }
    // }
    let mut result: u32 = 0;
    for node in &tree.arena {
        if node.val == 0 {
            // let mut sum: u32 = get_sum_from_node(&tree, node.idx);
            let mut sum: u32 = get_sum_from_node(&tree, node.idx);
            // let mut sum: u32 = node.val;
            // let sum_children: u32 = node
            //     .children
            //     .iter()
            //     .map(|&child_idx| {
            //         let child = &tree.arena[child_idx];
            //         child.val
            //     })
            //     .sum();
            // // println!("{:?}", sum + sum_children);
            // sum += sum_children;
            if sum < 100000 {
                result += sum;
            }
            println!("{}: {}", node.name, sum);
        }
    }
    println!("Part1: {}", result);
    // println!("{:?}", tree);
}

fn get_sum_from_node(tree: &Tree<&str>, idx: usize) -> u32 {
    let node = &tree.arena[idx];
    println!("{:?}", node);
    if node.children.len() == 0 {
        return 0;
    }
    if node.val > 0 {
        return node.val;
    }

    node.children
        .iter()
        .map(|&child_idx| get_sum_from_node(tree, child_idx))
        .sum()
}
