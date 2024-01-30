use std::sync::mpsc::Sender;

pub fn bubble_sort(list: &mut Vec<f64>, sender: Sender<(Vec<f64>, usize, bool)>) {
    let mut sorted = false;
    let mut list_len = list.len();
    let now = std::time::SystemTime::now();

    while !sorted {
        let mut swapped = false;
        for i in 1..list_len {
            if list[i - 1] > list[i] {
                list.swap(i - 1, i);
                swapped = true;
                sender.send((list.clone(), i - 1, false)).unwrap();
            }
        }
        if !swapped {
            sorted = true;
            sender.send((list.to_vec(), 0, true)).unwrap();
        }
        list_len -= 1;
    }
    println!("{}", now.elapsed().unwrap().as_millis());
}
