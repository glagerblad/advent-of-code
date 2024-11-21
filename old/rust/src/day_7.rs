use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

// USED HELP https://www.youtube.com/watch?v=ajf7DfqeIQY&ab_channel=UncleScientist

#[derive(Default, Debug)]
struct Directory {
    _name: String,
    size: RefCell<usize>,
    parent: Option<Rc<Directory>>,
    sub_dir: RefCell<BTreeMap<String, Rc<Directory>>>,
}

pub fn run_a(input: &Vec<String>) -> usize {
    let root = Rc::new(Directory::default());
    let mut current = Rc::clone(&root);
    for line in input {
        let words = line.split(" ").collect::<Vec<&str>>();
        match (words[0], words[1]) {
            ("$", "ls") => {}
            ("$", "cd") => match words[2] {
                "/" => current = Rc::clone(&root),
                ".." => current = Rc::clone(&current.parent.as_ref().unwrap()),
                name => {
                    let new_dir = current.sub_dir.borrow().get(name).unwrap().clone();
                    current = new_dir
                }
            },
            ("dir", name) => {
                current.sub_dir.borrow_mut().insert(
                    name.to_string(),
                    Rc::new(Directory {
                        _name: name.to_string(),
                        size: RefCell::new(0),
                        parent: Some(Rc::clone(&current)),
                        sub_dir: RefCell::new(BTreeMap::new()),
                    }),
                );
            }
            (size, _name) => *current.size.borrow_mut() += size.parse::<usize>().unwrap(),
        }
    }

    let mut stack = vec![Rc::clone(&root)];
    let mut sum = 0;
    while let Some(dir) = stack.pop() {
        for sub_dir in dir.sub_dir.borrow().values() {
            stack.push(Rc::clone(sub_dir))
        }
        let dir_size = get_size(0, &dir);
        if dir_size <= 100_000 {
            sum += dir_size
        }
    }
    return sum;
}

fn get_size(acc: usize, dir: &Rc<Directory>) -> usize {
    let size = *dir.size.borrow();
    return acc
        + size
        + dir
            .sub_dir
            .borrow()
            .values()
            .fold(0, |x, y| x + get_size(0, y));
}

pub fn run_b(input: &Vec<String>) -> usize {
    let root = Rc::new(Directory::default());
    let mut current = Rc::clone(&root);
    for line in input {
        let words = line.split(" ").collect::<Vec<&str>>();
        match (words[0], words[1]) {
            ("$", "ls") => {}
            ("$", "cd") => match words[2] {
                "/" => current = Rc::clone(&root),
                ".." => current = Rc::clone(&current.parent.as_ref().unwrap()),
                name => {
                    let new_dir = current.sub_dir.borrow().get(name).unwrap().clone();
                    current = new_dir
                }
            },
            ("dir", name) => {
                current.sub_dir.borrow_mut().insert(
                    name.to_string(),
                    Rc::new(Directory {
                        _name: name.to_string(),
                        size: RefCell::new(0),
                        parent: Some(Rc::clone(&current)),
                        sub_dir: RefCell::new(BTreeMap::new()),
                    }),
                );
            }
            (size, _name) => *current.size.borrow_mut() += size.parse::<usize>().unwrap(),
        }
    }

    let max_size = 70_000_000;
    let current_size = get_size(0, &root);
    let space_left = max_size - current_size;
    let space_needed = 30_000_000 - space_left;

    let mut stack = vec![Rc::clone(&root)];
    let mut small_dir_size = usize::MAX;

    while let Some(dir) = stack.pop() {
        for sub_dir in dir.sub_dir.borrow().values() {
            stack.push(Rc::clone(sub_dir))
        }
        let dir_size = get_size(0, &dir);
        if dir_size >= space_needed {
            small_dir_size = small_dir_size.min(dir_size)
        }
    }

    return small_dir_size;
}
