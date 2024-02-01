use std::{fmt::Debug, sync::mpsc::Sender};

pub fn merge_sort<T: PartialOrd + Clone + Debug>(
    root: &mut Vec<T>,
    list: &mut Vec<T>,
    start_index: usize,
    sender: Sender<(Vec<T>, usize, bool)>,
) {
    let now = std::time::SystemTime::now();
    if list.len() <= 1 {
        return;
    }

    let middle = list.len() / 2;
    let mut left = list[0..middle].to_vec();
    let mut right = list[middle..].to_vec();

    merge_sort(root, &mut left, start_index, sender.clone());
    merge_sort(root, &mut right, start_index + middle, sender.clone());

    merge(root, list, &left, &right, start_index, sender.clone());
    if list.len() == root.len() {
        println!(
            "{} {}",
            "Time to finish: ",
            now.elapsed().unwrap().as_millis()
        );
        sender.send((root.clone(), 0, true)).unwrap();
    }
}

pub fn merge<T: PartialOrd + Clone + Debug>(
    root: &mut Vec<T>,
    list: &mut Vec<T>,
    left: &Vec<T>,
    right: &Vec<T>,
    index: usize,
    sender: Sender<(Vec<T>, usize, bool)>,
) {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    let mut root_index = index;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            root[root_index] = left[i].clone();
            list[k] = left[i].clone();
            i += 1;
        } else {
            root[root_index] = right[j].clone();
            list[k] = right[j].clone();
            j += 1;
        }

        sender.send((root.clone(), root_index, false)).unwrap();
        root_index += 1;
        k += 1;
    }

    while i < left.len() {
        root[root_index] = left[i].clone();
        list[k] = left[i].clone();
        sender.send((root.clone(), root_index, false)).unwrap();
        i += 1;
        k += 1;
        root_index += 1;
    }

    while j < right.len() {
        root[root_index] = right[j].clone();
        list[k] = right[j].clone();
        sender.send((root.clone(), root_index, false)).unwrap();
        j += 1;
        k += 1;
        root_index += 1;
    }
}
