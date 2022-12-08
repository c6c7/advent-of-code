use {
    std::{
        cell::RefCell,
        collections::HashMap,
        fmt,
        rc::{Rc, Weak},
    },
    tracing::{debug, info},
};

struct Directory {
    parent: Option<Weak<RefCell<Directory>>>,
    name: String,
    directory_children: HashMap<String, Rc<RefCell<Directory>>>,
    file_children: HashMap<String, File>,
}

impl fmt::Debug for Directory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parent_name = match self.parent.as_ref() {
            None => "<none>".to_string(),
            Some(p) => p.upgrade().unwrap().as_ref().borrow().name.clone(),
        };
        let directory_children = self.directory_children.keys().cloned();

        let file_children = self.file_children.keys().cloned();
        f.debug_struct("Directory")
            .field("parent", &parent_name)
            .field("name", &self.name)
            .field("directory_children", &directory_children)
            .field("file_children", &file_children)
            .finish()
    }
}

impl Directory {
    fn new(parent: Option<Rc<RefCell<Directory>>>, name: &str) -> Rc<RefCell<Directory>> {
        let child = Rc::new(RefCell::new(Directory {
            parent: parent.map(|d| Rc::downgrade(&d)),
            name: String::from(name),
            directory_children: HashMap::new(),
            file_children: HashMap::new(),
        }));
        if let Some(parent) = &child.as_ref().borrow().parent {
            parent
                .upgrade()
                .unwrap()
                .borrow_mut()
                .directory_children
                .insert(child.as_ref().borrow().name.clone(), Rc::clone(&child));
        }
        child
    }

    fn abs_path(dir: &Rc<RefCell<Directory>>) -> String {
        let dir_name = dir.as_ref().borrow().name.clone();
        match &dir.as_ref().borrow().parent {
            None => dir_name,
            Some(parent) => format!(
                "{}/{}",
                Directory::abs_path(&parent.upgrade().unwrap()),
                dir_name
            ),
        }
    }

    fn add_child_file(parent: Rc<RefCell<Self>>, mut child: File) {
        child.parent.replace(Rc::downgrade(&parent));
        parent
            .as_ref()
            .borrow_mut()
            .file_children
            .insert(child.name.clone(), child);
    }

    fn size(parent: &Rc<RefCell<Directory>>) -> usize {
        let mut total = 0;
        for (_, f) in parent.as_ref().borrow().file_children.iter() {
            total += f.size;
        }
        for (_, d) in parent.as_ref().borrow().directory_children.iter() {
            total += Directory::size(d);
        }
        total
    }

    fn part1_size_tracking(
        parent: &Rc<RefCell<Directory>>,
        dois: &mut HashMap<String, Rc<RefCell<Directory>>>,
    ) -> usize {
        let mut total = 0;
        for (_, f) in parent.as_ref().borrow().file_children.iter() {
            total += f.size;
        }
        for (_, d) in parent.as_ref().borrow().directory_children.iter() {
            total += Directory::part1_size_tracking(d, dois);
        }
        if total <= 100000 {
            dois.insert(Directory::abs_path(parent), parent.clone());
        }
        total
    }
}

#[derive(Debug, Clone)]
struct File {
    parent: Option<Weak<RefCell<Directory>>>,
    name: String,
    size: usize,
}

pub fn part1(input: String) {
    let cd_root_regex = regex::Regex::new(r"\$ cd /").unwrap();
    let cd_parent_regex = regex::Regex::new(r"\$ cd \.\.").unwrap();
    let cd_regex = regex::Regex::new(r"\$ cd (.*)").unwrap();
    let dir_regex = regex::Regex::new(r"dir (.*)").unwrap();
    let file_regex = regex::Regex::new(r"(\d+) (.*)").unwrap();
    let ls_regex = regex::Regex::new(r"\$ ls").unwrap();

    let root_directory = Directory::new(None, "");
    let mut current_directory = root_directory.clone();

    for line in input.trim().split('\n') {
        if cd_root_regex.is_match(line) {
            current_directory = root_directory.clone();
            continue;
        }
        if ls_regex.is_match(line) {
            continue;
        }
        if cd_parent_regex.is_match(line) {
            let next_directory = current_directory
                .as_ref()
                .borrow()
                .parent
                .as_ref()
                .unwrap()
                .clone();
            current_directory = next_directory.upgrade().unwrap();
            continue;
        }
        if let Some(cd_cap) = cd_regex.captures(line) {
            //debug!("cd_cap: {:?}", cd_cap);
            let next_directory = current_directory
                .as_ref()
                .borrow()
                .directory_children
                .get(cd_cap.get(1).unwrap().as_str())
                .unwrap()
                .clone();
            current_directory = next_directory;
            continue;
        }
        if let Some(dir_cap) = dir_regex.captures(line) {
            Directory::new(
                Some(current_directory.clone()),
                dir_cap.get(1).unwrap().as_str(),
            );
            continue;
        }
        if let Some(file_cap) = file_regex.captures(line) {
            Directory::add_child_file(
                current_directory.clone(),
                File {
                    parent: None,
                    name: file_cap.get(2).unwrap().as_str().to_string(),
                    size: file_cap.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                },
            );
            continue;
        }
        unreachable!()
    }

    debug!("Root directory: {:?}", root_directory.as_ref().borrow());
    debug!("Root directory size: {}", Directory::size(&root_directory));

    let mut dois = HashMap::new();
    Directory::part1_size_tracking(&root_directory, &mut dois);

    //debug!("DOIs: {:?}", dois);
    debug!("DOIs len: {}", dois.len());
    let doi_sizes = dois
        .iter()
        .map(|(abs_path, d)| format!("{}-{}", abs_path, Directory::size(d)))
        .collect::<Vec<_>>();
    debug!("DOI sizes: {:?}", doi_sizes);

    info!(
        "Part 1 Answer: {}",
        dois.iter().fold(0, |acc, (_, d)| acc + Directory::size(d))
    );
}

pub fn part2(_input: String) {}

#[cfg(test)]
mod tests {
    use {super::*, tracing_test::traced_test};

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
        Directory::add_child_file(root.clone(), f);
        assert_eq!(1, root.as_ref().borrow().file_children.len());

        debug!("root: {:?}", root.as_ref().borrow());
    }

    #[test]
    #[traced_test]
    fn simple_size() {
        let f = File {
            parent: None,
            name: "a".to_string(),
            size: 1000,
        };
        let root = Directory::new(None, "root");
        Directory::add_child_file(root.clone(), f);
        assert_eq!(1000, Directory::size(&root));
    }

    #[test]
    #[traced_test]
    fn multiple_files_size() {
        let a = File {
            parent: None,
            name: "a".to_string(),
            size: 1000,
        };
        let b = File {
            parent: None,
            name: "b".to_string(),
            size: 754,
        };
        let root = Directory::new(None, "root");
        Directory::add_child_file(root.clone(), a);
        Directory::add_child_file(root.clone(), b);
        assert_eq!(1754, Directory::size(&root));
    }

    #[test]
    #[traced_test]
    fn nested_file_fs_size() {
        let a = File {
            parent: None,
            name: "a".to_string(),
            size: 101,
        };
        let root = Directory::new(None, "root");
        let sub_root = Directory::new(Some(root.clone()), "sub_root");
        Directory::add_child_file(sub_root.clone(), a);
        assert_eq!(101, Directory::size(&root));
        assert_eq!(101, Directory::size(&sub_root));
    }
}
