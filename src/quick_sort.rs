use std::{sync::mpsc::Sender, time::SystemTime};

pub fn quick_sort<T: Clone + PartialOrd>(list: &mut Vec<T>, sender: Sender<(Vec<T>, usize, bool)>) {
    let now = SystemTime::now();
    _quick_sort(list, 0, list.len() - 1, sender.clone());
    println!(
        "{} {} {}",
        "Time to finish: ",
        now.elapsed().unwrap().as_millis(),
        " ms"
    );
    sender.send((list.clone(), 0, true)).unwrap();
}

fn _quick_sort<T: Clone + PartialOrd>(
    list: &mut Vec<T>,
    left: usize,
    right: usize,
    sender: Sender<(Vec<T>, usize, bool)>,
) {
    if left < right {
        let partition_index = partition(list, left, right, sender.clone());

        _quick_sort(
            list,
            left,
            if partition_index > 0 {
                partition_index - 1
            } else {
                0
            },
            sender.clone(),
        );
        _quick_sort(list, partition_index + 1, right, sender.clone());
    }
}

fn partition<T: Clone + PartialOrd>(
    list: &mut Vec<T>,
    left: usize,
    right: usize,
    sender: Sender<(Vec<T>, usize, bool)>,
) -> usize {
    let mut left_pointer = left;
    let mut right_pointer = right - 1;
    let pivot = right;

    loop {
        while list[left_pointer] < list[pivot] {
            left_pointer += 1;
        }
        while right_pointer > 0 && list[right_pointer] > list[pivot] {
            right_pointer -= 1;
        }
        if left_pointer >= right_pointer {
            break;
        } else {
            list.swap(left_pointer, right_pointer);
            sender.send((list.clone(), right_pointer, false)).unwrap();
            left_pointer += 1;
            right_pointer -= 1;
        }
    }

    list.swap(left_pointer, pivot);
    left_pointer
}
