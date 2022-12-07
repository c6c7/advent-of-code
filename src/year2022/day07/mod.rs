use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    fmt,
    hash::{Hash, Hasher},
    rc::Rc,
};

struct Directory {
    parent: Option<Rc<RefCell<Directory>>>,
    name: String,
    directory_children: HashMap<String, Rc<RefCell<Directory>>>,
    file_children: HashMap<String, File>,
}

impl fmt::Debug for Directory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parent_name = match self.parent.as_ref() {
            None => "<none>".to_string(),
            Some(p) => p.as_ref().borrow().name.clone(),
        };
        let directory_children = self
            .directory_children
            .iter()
            .map(|(name, _)| name.clone())
            .collect::<HashSet<_>>();

        let file_children = self
            .file_children
            .iter()
            .map(|(name, _)| name.clone())
            .collect::<HashSet<_>>();
        f.debug_struct("Directory")
            .field("parent", &parent_name)
            .field("name", &self.name)
            .field("directory_children", &directory_children)
            .field("file_children", &file_children)
            .finish()
    }
}

enum FsObject {
    Directory(Directory),
    File(File),
}

impl Directory {
    fn new(parent: Option<Rc<RefCell<Directory>>>, name: &str) -> Rc<RefCell<Directory>> {
        let child = Rc::new(RefCell::new(Directory {
            parent,
            name: String::from(name),
            directory_children: HashMap::new(),
            file_children: HashMap::new(),
        }));
        if let Some(parent) = &child.as_ref().borrow().parent {
            parent
                .as_ref()
                .borrow_mut()
                .directory_children
                .insert(child.as_ref().borrow().name.clone(), Rc::clone(&child));
        }
        child
    }

    fn add_child_file(parent: Rc<RefCell<Self>>, mut child: File) {
        child.parent.replace(Rc::clone(&parent));
        parent
            .as_ref()
            .borrow_mut()
            .file_children
            .insert(child.name.clone(), child);
    }
}

#[derive(Debug, Clone)]
struct File {
    parent: Option<Rc<RefCell<Directory>>>,
    name: String,
    size: usize,
}

pub fn part1(input: String) {}

pub fn part2(input: String) {}

#[cfg(test)]
mod tests {
    use {super::*, tracing::debug, tracing_test::traced_test};

    #[test]
    #[traced_test]
    fn simple_directory_construction() {
        let root = Directory::new(None, "root");
        debug!("{:?}", root.as_ref().borrow());
    }

    #[test]
    #[traced_test]
    fn single_file_fs() {
        let f = File {
            parent: None,
            name: "a".to_string(),
            size: 0,
        };
        let root = Directory::new(None, "root");
        Directory::add_child_file(root.clone(), f);
        assert_eq!(1, root.as_ref().borrow().file_children.len());

        debug!("{:?}", root.as_ref().borrow());
    }

    #[test]
    #[traced_test]
    fn nested_file_fs() {
        let f = File {
            parent: None,
            name: "a".to_string(),
            size: 0,
        };
        let root = Directory::new(None, "root");
        let sub_root = Directory::new(Some(root.clone()), "sub_root");
        assert_eq!(0, root.as_ref().borrow().file_children.len());
        assert_eq!(1, root.as_ref().borrow().directory_children.len());

        Directory::add_child_file(sub_root.clone(), f);
        assert_eq!(1, sub_root.as_ref().borrow().file_children.len());
        assert_eq!(0, sub_root.as_ref().borrow().directory_children.len());

        debug!("root: {:?}", root.as_ref().borrow());
        debug!("sub_root: {:?}", sub_root.as_ref().borrow());
    }

    #[test]
    #[traced_test]
    fn duplicate_named_file_dropped() {
        let f = File {
            parent: None,
            name: "a".to_string(),
            size: 0,
        };
        let root = Directory::new(None, "root");
        Directory::add_child_file(root.clone(), f.clone());
        assert_eq!(1, root.as_ref().borrow().file_children.len());
        Directory::add_child_file(root.clone(), f.clone());
        assert_eq!(1, root.as_ref().borrow().file_children.len());

        debug!("root: {:?}", root.as_ref().borrow());
    }
}
