pub fn bubble_sort(list: &mut Vec<f64>) {
    let mut sorted = false;

    while !sorted {
        let mut swapped = false;
        for i in 1..list.len() {
            if list[i - 1] > list[i] {
                list.swap(i - 1, i);
                swapped = true;
            }
        }
        if !swapped {
            sorted = true;
        }
    }
}
