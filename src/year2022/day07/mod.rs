use std::{
    cell::RefCell,
    collections::HashSet,
    fmt,
    hash::{Hash, Hasher},
    rc::Rc,
};

struct Directory {
    parent: Option<Rc<RefCell<Directory>>>,
    name: String,
    directory_children: HashSet<Rc<RefCell<Directory>>>,
    file_children: HashSet<File>,
}

impl fmt::Debug for Directory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parent_name = match self.parent.as_ref() {
            None => "<none>".to_string(),
            Some(p) => p.as_ref().borrow().name.clone(),
        };
        f.debug_struct("Directory")
            .field("parent", &parent_name)
            .field("name", &self.name)
            .field(
                "directory_children",
                &self
                    .directory_children
                    .iter()
                    .map(|d| d.as_ref().borrow().name.clone())
                    .collect::<HashSet<_>>(),
            )
            .field("file_children", &self.file_children)
            .finish()
    }
}

enum FsObject {
    Directory(Directory),
    File(File),
}

impl Directory {
    fn new_root() -> Self {
        Directory {
            parent: None,
            name: "".to_string(),
            directory_children: HashSet::new(),
            file_children: HashSet::new(),
        }
    }
}

impl Hash for Directory {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

#[derive(Debug)]
struct File {
    name: String,
    size: usize,
}

impl Hash for File {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

pub fn part1(input: String) {}

pub fn part2(input: String) {}

#[cfg(test)]
mod tests {
    use {super::*, tracing::debug, tracing_test::traced_test};

    #[test]
    #[traced_test]
    fn simple_directory_construction() {
        let root = Directory::new_root();
        debug!("{root:?}");
    }

    #[test]
    #[traced_test]
    fn hand_constructed_fs() {}
}
