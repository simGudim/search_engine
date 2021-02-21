

pub fn merge<T: Copy + PartialOrd>(a: &Vec<T>, mut l1: usize, r1: usize, mut l2: usize, r2: usize,) -> Vec<T> {
    let mut temp = Vec::with_capacity(a.len());
    while l1 <= r1 && l2 <= r2 {
        if a[l1] <= a[l2] {
            temp.push(a[l1]);
            l1 += 1
        } else {
            temp.push(a[l2]);
            l2 += 1;
        }
    }

    while l1 <= r1 {
        temp.push(a[l1]);
        l1 += 1;
    }

    while l2 <= r2 {
        temp.push(a[l2]);
        l2 += 1;
    }
    temp
}


pub fn mergesort<T: Copy + PartialOrd>(mut items: Vec<T>) -> Vec<T> {
    let mut size: usize = 1;
    let n: usize = items.len();
    while size < n {
        let mut i: usize = 0;

        while i < n {
            let l1: usize = i;
            let r1: usize = i + size - 1;
            let mut r2: usize= i + 2 * size - 1;
            let l2: usize= i + size;

            if l2 >= n {
                break
            }

            if r2 >= n {
                r2 = n - 1;
            }

            let temp = merge(&items, l1, r1, l2, r2);
            for j in 0..(r2-l1 +1) {
                items[i + j] = temp[j];
            }
            i = i + 2 * size;
        }
        size = 2 * size;
    }
    items
}
