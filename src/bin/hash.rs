use std::{
    collections::HashMap,
    hash::{self, BuildHasher, Hasher},
};

#[allow(dead_code)]
#[inline]
fn random() -> usize {
    let state = hash::RandomState::new();
    let hasher = BuildHasher::build_hasher(&state);
    hasher.finish() as usize
}

type TreeArena = HashMap<usize, Node>;

struct Tree {
    root: usize,
    nodes: TreeArena,
}

struct Node {
    l: Option<usize>,
    r: Option<usize>,
}

impl Tree {
    pub fn create(depth: i32) -> Self {
        fn create_node(nodes: &mut TreeArena, depth: i32) -> usize {
            let node = if depth > 0 {
                Node {
                    l: Some(create_node(nodes, depth - 1)),
                    r: Some(create_node(nodes, depth - 1)),
                }
            } else {
                Node { l: None, r: None }
            };
            // let key = random();
            let key = nodes.len() + 1;
            nodes.insert(key, node);
            key
        }

        let mut nodes = TreeArena::with_capacity(1 << depth + 1);
        let root = create_node(&mut nodes, depth);

        Tree { root, nodes }
    }

    pub fn check(&self) -> i32 {
        self.check_rec(self.root)
    }

    fn check_rec(&self, node: usize) -> i32 {
        let node = self.nodes.get(&node).unwrap();

        let mut res = 1;
        if let Some(l) = node.l {
            res += self.check_rec(l);
        }
        if let Some(r) = node.r {
            res += self.check_rec(r);
        }

        res
    }
}

const MIN_DEPTH: i32 = 4;

fn main() {
    let n = std::env::args_os()
        .nth(1)
        .and_then(|s| s.into_string().ok())
        .and_then(|n| n.parse().ok())
        .unwrap_or(10);

    let max_depth = if MIN_DEPTH + 2 > n { MIN_DEPTH + 2 } else { n };
    {
        let depth = max_depth + 1;
        let tree = Tree::create(max_depth + 1);

        println!("stretch tree of depth {}\t check: {}", depth, tree.check());
    }

    let long_lived_tree = Tree::create(max_depth);

    for d in (MIN_DEPTH..max_depth + 1).step_by(2) {
        let iterations = 1 << ((max_depth - d + MIN_DEPTH) as u32);
        let mut chk = 0;
        let timer = std::time::Instant::now();
        for _i in 0..iterations {
            let a = Tree::create(d);
            chk += a.check();
        }
        println!(
            "{}\t trees of depth {}\t check: {} ({:.0?})",
            iterations,
            d,
            chk,
            timer.elapsed()
        )
    }

    println!(
        "long lived tree of depth {}\t check: {}",
        max_depth,
        long_lived_tree.check()
    );
}
