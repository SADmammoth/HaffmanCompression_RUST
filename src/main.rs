mod haffman;

fn main() {
    let query = haffman::create_priority_queue("FreedommmmeinnFd");
    println!("{}", query);
    let tree = haffman::create_tree(query);
    println!("{}", tree);
}
