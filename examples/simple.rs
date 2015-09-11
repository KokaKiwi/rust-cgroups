extern crate env_logger;
extern crate cgroups;

use std::ffi::OsStr;
use std::path::Path;
use cgroups::node::Node;

const CGROUPS_ROOT_PATH: &'static str = "/sys/fs/cgroup";

#[allow(unused)]
fn walk(parent: Node, indent: usize) {
    for _ in 0..indent {
        print!("  ");
    }
    if parent.path() == Path::new(CGROUPS_ROOT_PATH) {
        println!("{}", parent.path().display());
    } else {
        println!("/{}", parent.path().file_name().and_then(OsStr::to_str).unwrap());
    }

    for child in parent.children().unwrap() {
        walk(child, indent + 1);
    }
}

fn main() {
    env_logger::init().unwrap();

    let node = Node::new(CGROUPS_ROOT_PATH);
    let cpuset = node.get_child("cpuset").unwrap();

    let test = cpuset.get_or_create_child("test").unwrap();

    test.delete(false).unwrap();

    // walk(node, 0);
}
