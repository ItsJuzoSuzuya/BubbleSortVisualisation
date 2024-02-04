use std::sync::mpsc::Sender;

use rand::seq::SliceRandom;

pub fn bogo_sort(list: &mut Vec<f64>, sender: Sender<(Vec<f64>, usize, bool)>) {
    let now = std::time::SystemTime::now();
    let mut rng;
    while !is_sorted(&list) {
        rng = rand::thread_rng();
        list.shuffle(&mut rng);
        sender.send((list.to_vec(), 0, false)).unwrap();
    }
    sender.send((list.to_vec(), 0, true)).unwrap();
    println!(
        "{} {} {}",
        "Time to finish: ",
        now.elapsed().unwrap().as_millis(),
        " ms"
    );
}

fn is_sorted<T: PartialOrd>(slice: &[T]) -> bool {
    slice.windows(2).all(|window| window[0] <= window[1])
}
