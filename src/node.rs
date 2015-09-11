use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    path: PathBuf,
}

impl Node {
    pub fn new<P: Into<PathBuf>>(path: P) -> Node {
        Node {
            path: path.into(),
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn create_child(&self, name: &str) -> io::Result<Node> {
        let path = self.path.join(name);
        let node = Node::new(path);

        try!(fs::create_dir(node.path()));

        Ok(node)
    }

    pub fn get_child(&self, name: &str) -> Option<Node> {
        let path = self.path.join(name);

        let attr = match fs::metadata(&path) {
            Ok(attr) => attr,
            Err(_) => return None,
        };
        if !attr.is_dir() {
            return None;
        }

        Some(Node::new(path))
    }

    pub fn get_or_create_child(&self, name: &str) -> Option<Node> {
        let path = self.path.join(name);

        let exists = {
            match fs::metadata(path) {
                Err(_) => false,
                _ => true,
            }
        };

        if !exists {
            self.create_child(name).ok()
        } else {
            self.get_child(name)
        }
    }

    pub fn delete(self, full: bool) -> io::Result<()> {
        if full {
            fs::remove_dir_all(self.path)
        } else {
            fs::remove_dir(self.path)
        }
    }

    pub fn children(&self) -> io::Result<Childrens> {
        let children = Childrens {
            iter: try!(fs::read_dir(&self.path)),
        };
        Ok(children)
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<Node {}>", self.path.display())
    }
}

pub struct Childrens {
    iter: fs::ReadDir,
}

impl Iterator for Childrens {
    type Item = Node;

    fn next(&mut self) -> Option<Node> {
        fn is_dir<P: AsRef<Path>>(path: P) -> bool {
            let metadata = match fs::metadata(&path) {
                Ok(metadata) => metadata,
                Err(_) => return false,
            };
            let file_type = metadata.file_type();

            if file_type.is_dir() {
                true
            } else if file_type.is_symlink() {
                let target = match fs::read_link(&path) {
                    Ok(target) => target,
                    Err(_) => return false,
                };
                is_dir(target)
            } else {
                false
            }
        }

        while let Some(entry) = self.iter.next() {
            let entry = match entry {
                Ok(entry) => entry,
                Err(_) => return None,
            };
            let path = entry.path();

            if is_dir(&path) {
                return Some(Node::new(path))
            }
        }

        None
    }
}
