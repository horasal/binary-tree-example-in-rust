use std::env;

enum BinaryTree {
    Child(i32, Box<BinaryTree>, Box<BinaryTree>),
    None(i32),
}

impl BinaryTree {
    pub fn new(item: i32, depth: i32) -> BinaryTree {
        if depth == 0 {
            BinaryTree::None(item)
        } else {
            BinaryTree::Child(item, 
              Box::new(BinaryTree::new(2 * item - 1, depth - 1)),
              Box::new(BinaryTree::new(2 * item, depth - 1)))
        }
    }

    pub fn item_check(&self) -> i32 {
        match self {
            &BinaryTree::Child(i, ref l, ref r) => i + l.item_check() - r.item_check(),
            &BinaryTree::None(i) => i,
        }
    }
}

const MIN_DEPTH : i32 = 4;

fn main() {
    let depth = env::args().nth(1).unwrap_or("10".to_string()).parse::<i32>().unwrap();
    println!("stretch tree of depth {}\tcheck {}", depth + 1,
             BinaryTree::new(0, depth + 1).item_check());
    let long_lived = BinaryTree::new(0, depth);
    for (d, i) in (MIN_DEPTH..depth+1).filter(| x | x % 2 == 0).map(| x | 
         (x, (1..(1<<(depth-x+MIN_DEPTH))+1).fold(0,| check, j |
      check + BinaryTree::new(j, x).item_check() + BinaryTree::new(-j, x).item_check()))) {
        println!("{}\ttrees of depth {}\tcheck {}", 1<<(depth-d+MIN_DEPTH)+1, d, i);
    }
    println!("long lived tree of depth {}\tcheck {}", depth, long_lived.item_check());
}
