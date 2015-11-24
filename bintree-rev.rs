use std::env;

struct Tree {
    left: Option<Box<Tree>>,
    right: Option<Box<Tree>>,
    item: i32,
}

impl Tree {
    pub fn new(depth: i32, i: i32) -> Tree {
        if depth <= 0 {
            Tree { item : i, left: None, right: None }
        } else {
            Tree { item : i, 
                left: Some(Box::new(Tree::new(depth - 1, 2 * i - 1))),
                right: Some(Box::new(Tree::new(depth - 1, 2 * i ))),
            }
        }
    }

    pub fn item_check(&self) -> i32 {
        self.item + 
            self.left.as_ref().map(| t | t.item_check()).unwrap_or(0) -
            self.right.as_ref().map(| t | t.item_check()).unwrap_or(0)
    }
}

const MINDEP : i32 = 4;

fn main() {
    let depth = env::args().nth(1).unwrap_or("10".to_string()).parse::<i32>().unwrap_or(10);
    println!("Running program with depth = {}", depth);
    let stretch = depth + 1;
    println!("stretch tree of depth {}\t check: {}", stretch, Tree::new(stretch, 0).item_check());
    let long_lived = Tree::new(depth, 0);
    let res = (MINDEP .. depth + 1).filter(| x | x % 2 == 0).map( | x | 
                    (1 << (depth - x + MINDEP + 1), x, (1 .. (1 << (depth - x + MINDEP)) + 1).fold(0, 
                        | xt , x1 | xt + Tree::new(x, x1).item_check() + Tree::new(x, -x1).item_check())));
    for (iters, i, check) in res { 
        println!("{}\t trees of depth {}\t check: {}", iters, i, check); 
    }

    println!("long lived tree of depth {}\t check: {}", depth, long_lived.item_check());
}
