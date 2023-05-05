use std::cmp::Ordering::{Less, Equal, Greater};

use num_traits::Num;

pub trait SortType: Ord + Copy {}
pub trait NumType: SortType + Num {}

const INCS: [usize; 5] = [48, 21, 7, 3, 1];
const SWAPS: [usize; 120] = [
    1,  2,  3,  4, 5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15, 16,
    1,  3,  5,  7, 9, 11, 13, 15,  2,  4,  6,  8, 10, 12, 14, 16,
    1,  5,  9, 13, 2,  6, 10, 14,  3,  7, 11, 15,  4,  8, 12, 16,
    1,  9,  2, 10, 3, 11,  4, 12,  5, 13,  6, 14,  7, 15,  8, 16,
    6, 11,  7, 10, 4, 13, 14, 15,  8, 12,  2,  3,  5,  9,
    2,  5,  8, 14, 3,  9, 12, 15,  6,  7, 10, 11,
    3,  5, 12, 14, 4,  9,  8, 13,
    7,  9, 11, 13, 4,  6,  8, 10,
    4,  5,  6,  7, 8,  9, 10, 11, 12, 13,
    7,  8,  9, 10
];

#[inline]
fn reverse<T: SortType>(array: &mut [T], mut a: usize, mut b: usize) {
    b -= 1;
    while a < b {
        array.swap(a, b);
        a += 1;
        b -= 1;
    }
}

fn sift_down<T: SortType>(array: &mut [T], mut r: usize, d: usize, a: usize) {
    while r <= d / 2 {
        let mut l = 2 * r;

        if l < d && array[a + l - 1].cmp(&array[a + l]) == Less {
            l += 1;
        }

        if array[a + r - 1].cmp(&array[a + l - 1]) == Less {
            array.swap(a + r - 1, a + l - 1);
            r = l;
        } else {
            break;
        }
    }
}

fn max_heap_sort<T: SortType>(array: &mut [T], a: usize, b: usize) {
    let l = b - a;

    for i in (1 ..= l / 2).rev() {
        sift_down(array, i, l, a);
    }

    for i in (2 ..= l).rev() {
        array.swap(a, a + i - 1);
        sift_down(array, 1, i - 1, a);
    }
}

fn unchecked_insertion_sort<T: SortType>(array: &mut [T], a: usize, b: usize) {
    for i in a + 1 .. b {
        if array[i].cmp(&array[a]) == Less {
            array.swap(i, a);
        }

        let key = array[i].clone();
        let mut j = i - 1;
        while key.cmp(&array[j]) == Less {
            array[j + 1] = array[j];
            j -= 1;
        }
    }
}

fn shell_sort<T: SortType>(array: &mut [T], a: usize, b: usize) {
    for h in INCS {
        for i in a + h .. b {
            let key = array[i].clone();

            let mut j = i;
            while j >= a + h && array[j - h].cmp(&key) == Greater {
                array[j] = array[j - h];
                j -= h;
            }
            array[j] = key;
        }
    }
}

#[allow(unreachable_code)]
fn partition<T: SortType>(array: &mut [T], a: usize, b: usize, p: usize) -> usize {
    let mut i = a - 1;
    let mut j = b;

    loop {
        i += 1;
        while i < b && array[i].cmp(&array[p]) == Less {
            i += 1;
        }

        j -= 1;
        while j >= a && array[j].cmp(&array[p]) == Greater {
            j -= 1;
        }

        if i < j {
            array.swap(i, j);
        } else {
            return j;
        }
    }

    return a;
}

#[inline]
fn comp_swap<T: SortType>(array: &mut [T], a: usize, b: usize, g: usize, s: usize) {
    let x = s + a * g;
    let y = s + b * g;

    if array[x].cmp(&array[y]) == Greater {
        array.swap(x, y);
    }
}

fn median_of_three<T: SortType>(array: &mut [T], a: usize, mut b: usize) {
    b -= 1;
    let m = a + (b - a) / 2;

    comp_swap(array, a, m, 1, 0);
    if array[m].cmp(&array[b]) == Greater {
        array.swap(m, b);

        if array[a].cmp(&array[m]) == Greater {
            return;
        }
    }

    array.swap(a, m);
}

fn median_of_sixteen<T: SortType>(array: &mut [T], a: usize, b: usize) {
    let g = (b - 1 - a) / 16;

    for i in (0 .. SWAPS.len()).step_by(2) {
        comp_swap(array, SWAPS[i], SWAPS[i + 1], g, a);
    }

    array.swap(a, a + 8 * g);
}

fn get_sorted_runs<T: SortType>(array: &mut [T], a: usize, b: usize) -> bool {
    let mut rs = true;
    let mut s  = true;

    for i in a .. b - 1 {
        if array[i].cmp(&array[i + 1]) == Greater {
            s = false;
        } else {
            rs = false;
        }

        if (!rs) && (!s) {
            return false;
        }
    }

    if rs && !s {
        reverse(array, a, b);
        return true;
    }

    return s;
}

fn median_of_sixteen_aqsort<T: SortType>(array: &mut [T], mut a: usize, mut b: usize, mut d: usize, unbalanced: bool) {
    while b - a > 16 {
        if get_sorted_runs(array, a, b) {
            return;
        }

        if d == 0 {
            max_heap_sort(array, a, b);
            return;
        }

        let mut p = a;
        if !unbalanced {
            median_of_three(array, a, b);
            p = partition(array, a, b, a);
        }

        let l = p - a;
        let r = b - (p + 1);

        if unbalanced || (l == 0 || r == 0) || (l / r >= 16 || r / l >= 16) {
            if b - a > 80 {
                array.swap(a, p);

                if l < r {
                    median_of_sixteen_aqsort(array, a, p, d - 1, true);
                    a = p;
                } else {
                    median_of_sixteen_aqsort(array, p + 1, b, d - 1, true);
                    b = p;
                }
            } else {
                shell_sort(array, a, b);
                return;
            }
        }

        array.swap(a, p);

        d -= 1;
        median_of_sixteen_aqsort(array, p, b, d, false);
        b = p;
    }

    unchecked_insertion_sort(array, a, b);
}

#[inline]
pub fn sort<T: SortType>(array: &mut [T], a: usize, b: usize) {
    median_of_sixteen_aqsort(array, a, b, (2.0 * ((b - a) as f32).log2()) as usize, false);
}

#[inline]
fn multi_swap<T: SortType>(array: &mut [T], a: usize, b: usize, l: usize) {
    for i in 0 .. l {
        array.swap(a + i, b + i);
    }
}

#[inline]
fn multi_swap_bw<T: SortType>(array: &mut [T], a: usize, b: usize, l: usize) {
    for i in (0 .. l).rev() {
        array.swap(a + i , b + i);
    }
}

#[inline]
fn insert_to<T: SortType>(array: &mut [T], from: usize, to: usize) {
    let tmp = array[from].clone();
    for i in (to .. from).rev() {
        array[i + 1] = array[i];
    }
    array[to] = tmp;
}

#[inline]
fn insert_to_bw<T: SortType>(array: &mut [T], from: usize, to: usize) {
    let tmp = array[from].clone();
    for i in from .. to {
        array[i] = array[i + 1];
    }
    array[to] = tmp;
}

#[inline]
fn rotate<T: SortType>(array: &mut [T], mut a: usize, m: usize, mut b: usize) {
    while b - m > 1 && m - a > 1 {
        if b - m < m - a {
            multi_swap(array, a, m, b - m);
            a += b - m;
        } else {
            multi_swap_bw(array, a, b - (m - a), m - a);
            b -= m - a;
        }
    }

    if b - m == 1 {
        insert_to(array, m, a);
    } else if m - a == 1 {
        insert_to_bw(array, a, b - 1);
    }
}

fn binsearch<T: SortType>(array: &[T], mut a: usize, mut b: usize, val: T, l: bool) -> usize {
    while a < b {
        let m = a + ((b - a) / 2);

        let cmp = if l {
            val.cmp(&array[m]).is_le()
        } else {
            val.cmp(&array[m]) == Less
        };

        if cmp {
            b = m;
        } else {
            a = m + 1;
        }
    }

    return a;
}

fn insert_sort<T: SortType>(array: &mut [T], a: usize, b: usize) {
    for i in a + 1 .. b {
        if array[i].cmp(&array[i - 1]) == Less {
            let k = binsearch(array, a, i - 1, array[i], false);
            insert_to(array, i, k);
        }
    }
}

fn merge_up<T: SortType>(array: &mut [T], a: usize, m: usize, b: usize) {
    let len = m - a;
    let mut aux: Vec<T> = Vec::with_capacity(len);

    for i in 0 .. len {
        aux[i] = array[i + a];
    }

    let mut l = a;
    let mut r = m;
    let mut buf = 0;

    while l < r && r < b {
        if aux[buf].cmp(&array[r]).is_le() {
            array[l] = aux[buf];
            buf += 1;
        } else {
            array[l] = array[r];
            r += 1;
        }
        l += 1;
    }

    while l < r {
        array[l] = aux[buf];
        l += 1;
        buf += 1;
    }
}

fn merge_down<T: SortType>(array: &mut [T], a: usize, m: usize, b: usize) {
    let len = b - m;
    let mut aux: Vec<T> = Vec::with_capacity(len);

    for i in 0 .. len {
        aux[i] = array[i + m];
    }

    let mut buf = len - 1;
    let mut l = m - 1;
    let mut r = b - 1;

    while r > l && l >= a {
        if aux[buf].cmp(&array[l]).is_ge() {
            array[r] = aux[buf];
            buf -= 1;
        } else {
            array[r] = array[l];
            l -= 1;
        }
        r -= 1;
    }

    while r > l {
        array[r] = aux[buf];
        r -= 1;
        buf -= 1;
    }
}

#[inline]
fn check_bounds<T: SortType>(array: &mut [T], a: usize, m: usize, b: usize) -> bool {
    if array[m - 1].cmp(&array[m]).is_le() {
        return true;
    } else if array[a].cmp(&array[b - 1]) == Greater {
        rotate(array, a, m, b);
        return true;
    }

    return false;
}

fn merge_inplace<T: SortType>(array: &mut [T], a: usize, m: usize, b: usize) {
    if m - a <= b - m {
        let mut i = a;
        let mut j = m;

        while i < j && j < b {
            if array[i].cmp(&array[j]) == Greater {
                let k = binsearch(array, j, b, array[i], true);
                rotate(array, i, j, k);
                i += k - j;
                j = k;
            } else {
                i += 1;
            }
        }
    } else {
        let mut i = m - 1;
        let mut j = b - 1;

        while j > i && i >= a {
            if array[i].cmp(&array[j]) == Greater {
                let k = binsearch(array, a, i, array[j], false);
                rotate(array, k, i + 1, j + 1);
                j -= i + 1 - k;
                i = k - 1;
            } else {
                j -= 1;
            }
        }
    }
}

fn merge<T: SortType>(array: &mut [T], mut a: usize, m: usize, mut b: usize) {
    if check_bounds(array, a, m, b) {
        return;
    }

    b = binsearch(array, m, b, array[m - 1], true);
    a = binsearch(array, a, m - 1, array[m], false);

    if b - m < m - a {
        if b - m <= 16 {
            merge_inplace(array, a, m, b);
        } else {
            merge_down(array, a, m, b);
        }
    } else {
        if m - a <= 16 {
            merge_inplace(array, a, m, b);
        } else {
            merge_up(array, a, m, b);
        }
    }
}

#[inline]
fn get_reversed_runs<T: SortType>(array: &mut [T], a: usize, b: usize, limit: usize) -> bool{
    let mut i = a;
    while i < b - 1 {
        if array[i].cmp(&array[i + 1]).is_le() {
            break;
        }
    }

    if i - a > limit {
        reverse(array, a, i);
    }

    return i == b;
}

pub fn stable_sort<T: SortType>(array: &mut [T], a: usize, b: usize) {
    if get_reversed_runs(array, a, b, 8) {
        return
    }

    if b - a > 32 {
        let m = a + ((b - a) / 2);

        stable_sort(array, a, m);
        stable_sort(array, m, b);

        merge(array, a, m, b);
    } else {
        insert_sort(array, a, b);
    }
}



#[inline]
fn find_min_max<T: NumType>(array: &[T], a: usize, b: usize) -> (T, T) {
    let mut min = array[a].clone();
    let mut max = array[a].clone();

    for i in a + 1 .. b {
        if array[i] < min {
            min = array[i];
        } else if array[i] > max {
            max = array[i];
        }
    }

    return (min, max);
}

pub fn static_sort<T: NumType, GetFn>(array: &mut [T], a: usize, b: usize, get: GetFn) where GetFn: Fn(T) -> f32 {
    let (min, max) = find_min_max(array, a, b);

    let mut len = b - a + 1;
    let mut cnt: Vec<usize> = vec![0; len];
    let mut pts: Vec<usize> = vec![0; len];
    len -= 1;

    let mlt = len as f32 / (get(max) - get(min) + 1.0);

    for i in a .. b {
        cnt[((get(array[i]) - get(min)) * mlt) as usize] += 1;
    }

    pts[0] = a;
    for i in 1 .. len {
        pts[i] = cnt[i - 1] + pts[i - 1];
    }

    for i in 0 .. len {
        while cnt[i] > 0 {
            let orig     = pts[i];
            let mut from = orig;
            let mut val      = array[from];

            loop {
                let d = ((get(val) - get(min)) * mlt) as usize;
                let to = pts[d];

                pts[d] += 1;
                cnt[d] -= 1;

                let tmp = array[to];
                array[to] = val;
                val       = tmp;
                from      = to;

                if from == orig {
                    break;
                }
            }
        }
    }

    let mut s = a;
    for i in 0 .. len {
        let e = pts[i];

        if e - s <= 1 {
            s = pts[i];
            continue;
        }

        if e - s > 16 {
            max_heap_sort(array, s, e);
        } else {
            unchecked_insertion_sort(array, s, e);
        }

        s = pts[i];
    }
}