pub fn parallel_binary_search(
    arr: Vec<i32>,
    desired: i32,
    num_threads: usize,
) -> Option<usize> {
    let chunk_size = arr.len() / num_threads;
    let mut handles = vec![];

    for i in 0..num_threads {
        let arr_slice = arr.clone();
        let start = i * chunk_size;
        let end = if i == num_threads - 1 {
            arr.len() - 1
        } else {
            (i + 1) * chunk_size - 1
        };

        let handle = std::thread::spawn(move || {
            binary_search(&arr_slice, desired, start, end)
        });

        handles.push(handle);
    }

    for handle in handles {
        if let Some(index) = handle.join().unwrap() {
            return Some(index);
        }
    }

    None
}

pub fn binary_search(arr: &[i32], desired: i32, start: usize, end: usize) -> Option<usize> {
    let mut low = start;
    let mut high = end;

    while low <= high {
        let mid = (low + high) / 2;

        match arr[mid].cmp(&desired) {
            std::cmp::Ordering::Less => low = mid + 1,
            std::cmp::Ordering::Greater => high = mid - 1,
            std::cmp::Ordering::Equal => return Some(mid),
        }
    }

    None
}