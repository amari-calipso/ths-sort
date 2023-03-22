/*
Copyright (c) 2021 thatsOven

Permission is hereby granted, free of charge, to any person
obtaining a copy of this software and associated documentation
files (the "Software"), to deal in the Software without
restriction, including without limitation the rights to use,
copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the
Software is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES
OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
OTHER DEALINGS IN THE SOFTWARE.
*/

module ths_sort

import math

const (
	incs  = [48, 21, 7, 3, 1]
	swaps = [
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
	]
)

[inline]
[direct_array_access]
fn swap[T](mut array []T, a int, b int) {
	tmp     := array[a]
	array[a] = array[b]
	array[b] = tmp
}

[inline]
[direct_array_access]
fn reverse[T](mut array []T, s int, e int) {
	mut a := s
	mut b := e - 1

	for ; a < b; a++, b-- {
		swap(mut array, a, b)
	}
}

[direct_array_access]
fn sift_down[T](mut array []T, root int, d int, a int) {
	mut r := root

	for r <= d / 2 {
		mut l := 2 * r

		if l < d && array[a + l - 1] < array[a + l] {
			l++
		}

		if array[a + r - 1] < array[a + l - 1] {
			swap(mut array, a + r - 1, a + l - 1)
			r = l
		} else {
			break
		}
	}
}

[direct_array_access]
fn max_heap_sort[T](mut array []T, a int, b int) {
	l := b - a

	for i := l / 2; i >= 1; i-- {
		sift_down(mut array, i, l, a)
	}

	for i := b - a; i > 1; i-- {
		swap(mut array, a, a + i - 1)
		sift_down(mut array, 1, i - 1, a)
	}
}

fn unchecked_insert_sort[T](mut array []T, a int, b int) {
	for i in a + 1 .. b {
		if array[i] < array[a] {
			swap(mut array, i, a)
		}

		key   := array[i]
		mut j := i - 1
		for ; key < array[j]; j-- {
			array[j + 1] = array[j]
		}
		array[j + 1] = key
	}
}

[direct_array_access]
fn shell_sort[T](mut array []T, a int, b int) {
	for h in incs {
		for i in a + h .. b {
			v := array[i]

			mut j := i
			for ; j >= a + h && array[j - h] > v; j -= h {
				array[j] = array[j - h]
			}
			array[j] = v
		}
	}
}

[direct_array_access]
fn partition[T](mut array []T, a int, b int, p int) int {
	mut i := a - 1
	mut j := b

	for {
		i++
		for ; i  < b && array[i] < array[p]; i++ {}
		j--
		for ; j >= a && array[j] > array[p]; j-- {}

		if i < j {
			swap(mut array, i, j)
		} else {
			return j
		}
	}

	return a
}

[inline]
[direct_array_access]
fn comp_swap[T](mut array []T, a int, b int, g int, s int) {
	x := s + (a * g)
	y := s + (b * g)

	if array[x] > array[y] {
		swap(mut array, x, y)
	}
}

[direct_array_access]
fn median_of_three[T](mut array []T, a int, e int) {
	b := e - 1
	m := a + (b - a) / 2

	comp_swap(mut array, a, m, 1, 0)
	if array[m] > array[b] {
		swap(mut array, m, b)

		if array[a] > array[m] {
			return
		}
	}

	swap(mut array, a, m)
}

[direct_array_access]
fn median_of_sixteen[T](mut array []T, a int, b int) {
	g := (b - 1 - a) / 16

	for i := 0; i < swaps.len; i += 2 {
		comp_swap(mut array, swaps[i], swaps[i + 1], g, a)
	}

	swap(mut array, a, a + (8 * g))
}

[direct_array_access]
fn get_sorted_runs[T](mut array []T, a int, b int) bool {
	mut rs := true
	mut s  := true 

	for i in a .. b - 1 {
		if array[i] > array[i + 1] {
			s = false
		} else {
			rs = false
		}

		if (!rs) && (!s) {
			return false
		}
	}

	if rs && !s {
		reverse(mut array, a, b)
		return true
	}

	return s
}

[direct_array_access]
fn median_of_sixteen_aqsort[T](mut array []T, s int, e int, depth int, unbalanced bool) {
	mut a := s
	mut b := e
	mut d := depth
	
	for b - a > 16 {
		if get_sorted_runs(mut array, a, b) {
			return
		}

		if d == 0 {
			max_heap_sort(mut array, a, b)
			return
		}

		mut p := a
		if !unbalanced {
			median_of_three(mut array, a, b)
			p = partition(mut array, a, b, a)
		} 

		l := p - a
		r := b - (p + 1)

		if (l == 0 || r == 0) || (l / r >= 16 || r / l >= 16) || unbalanced {
			if b - a > 80 {
				swap(mut array, a, p)

				if l < r {
					median_of_sixteen_aqsort(mut array, a, p, d - 1, true)
					a = p
				} else {
					median_of_sixteen_aqsort(mut array, p + 1, b, d - 1, true)
					b = p
				}

				median_of_sixteen(mut array, a, b)
				p = partition(mut array, a + 1, b, a)
			} else {
				shell_sort(mut array, a, b)
				return
			}
		}

		swap(mut array, a, p)

		d--
		median_of_sixteen_aqsort(mut array, p, b, d, false)
		b = p
	}

	unchecked_insert_sort(mut array, a, b)
}

[inline]
pub fn sort[T](mut array []T, a int, b int) {
	median_of_sixteen_aqsort(mut array, a, b, int(2 * math.log2(b - a)), false)
}

[inline]
[direct_array_access]
fn multi_swap[T](mut array []T, a int, b int, l int) {
	for i in 0 .. l {
		swap(mut array, a + i, b + i)
	}
}

[inline]
[direct_array_access]
fn multi_swap_bw[T](mut array []T, a int, b int, l int) {
	for i := l - 1; i >= 0; i-- {
		swap(mut array, a + i, b + i)
	}
}

[inline]
[direct_array_access]
fn insert_to[T](mut array []T, from int, to int) {
	tmp := array[from]
	for i := from - 1; i >= to; i-- {
		array[i + 1] = array[i]
	}
	array[to] = tmp
}

[inline]
[direct_array_access]
fn insert_to_bw[T](mut array []T, from int, to int) {
	tmp := array[from]
	for i in from .. to {
		array[i] = array[i + 1]
	}
	array[to] = tmp
}

[inline]
[direct_array_access]
fn rotate[T](mut array []T, s int, m int, e int) {
	mut a := s
	mut b := e

	for b - m > 1 && m - a > 1 {
		if b - m < m - a {
			multi_swap(mut array, a, m, b - m)
			a += b - m
		} else {
			multi_swap_bw(mut array, a, b - (m - a), m - a)
			b -= m - a
		}
	}

	if b - m == 1 {
		insert_to(mut array, m, a)
	} else if m - a == 1 {
		insert_to_bw(mut array, a, b - 1)
	}
}

[direct_array_access]
fn binsearch[T](array []T, s int, e int, val T, l bool) int {
	mut a   := s
	mut b   := e
	mut cmp := true

	for a < b {
		m := a + ((b - a) / 2)

		if l {
			cmp = val <= array[m]
		} else {
			cmp = val < array[m]
		}

		if cmp {
			b = m
		} else {
			a = m + 1
		}
	}

	return a
}

[direct_array_access]
fn insert_sort[T](mut array []T, a int, b int) {
	for i in a + 1 .. b {
		if array[i] < array[i - 1] {
			k := binsearch(array, a, i - 1, array[i], false)
			insert_to(mut array, i, k)
		}
	}
}

[direct_array_access]
fn merge_up[T](mut array []T, a int, m int, b int) {
	mut aux := []T{len: m - a}

	for i in 0 .. m - a {
		aux[i] = array[i + a]
	}

	mut l := a
	mut r := m

	for nxt in 0 .. b - a {
		if l >= m && r >= b {
			break
		}

		if l < m && r >= b {
			array[a + nxt] = aux[l - a]
			l++
		} else if l >= m && r < b {
			break
		} else if aux[l - a] <= array[r] {
			array[a + nxt] = aux[l - a]
			l++
		} else {
			array[a + nxt] = array[r]
			r++
		}
	}
}

[direct_array_access]
fn merge_down[T](mut array []T, a int, m int, b int) {
	mut aux := []T{len: b - m}

	for i in 0 .. b - m {
		aux[i] = array[i + m]
	}

	mut l := m - 1
	mut r := b

	for nxt := b - a - 1; nxt >= 0; nxt-- {
		if l <= a && r <= m {
			break
		}

		if l < a && r >= a {
			array[a + nxt] = aux[r - m - 1]
			r--
		} else if (l >= a && r < a) || r < m + 1 {
			break
		} else if array[l] <= aux[r - m - 1] {
			array[a + nxt] = aux[r - m - 1]
			r--
		} else {
			array[a + nxt] = array[l]
			l--
		}
	}
}

[inline]
[direct_array_access]
fn check_bounds[T](mut array []T, a int, m int, b int) bool {
	if array[m - 1] <= array[m] {
		return true
	} else if array[a] > array[b - 1] {
		rotate(mut array, a, m, b)
		return true
	}

	return false
}

[direct_array_access]
fn merge_inplace[T](mut array []T, a int, m int, b int) {
	if m - a <= b - m {
		mut i := a
		mut j := m

		for i < j && j < b {
			if array[i] > array[j] {
				k := binsearch(array, j, b, array[i], true)
				rotate(mut array, i, j, k)
				i += k - j
				j = k
			} else {
				i++
			}
		}
	} else {
		mut i := m - 1
		mut j := b - 1

		for j > i && i >= a {
			if array[i] > array[j] {
				k := binsearch(array, a, i, array[j], false)
				rotate(mut array, k, i + 1, j + 1)
				j -= (i + 1) - k
				i = k - 1
			} else {
				j--
			}
		}
	}
}

[direct_array_access]
fn merge[T](mut array []T, s int, m int, e int) {
	if check_bounds(mut array, s, m, e) {
		return
	}

	b := binsearch(array, m,     e, array[m - 1], true)
	a := binsearch(array, s, m - 1, array[m],     false)

	if b - m < m - a {
		if b - m <= 8 {
			merge_inplace(mut array, a, m, b)
		} else {
			merge_down(mut array, a, m, b)
		}
	} else {
		if m - a <= 8 {
			merge_inplace(mut array, a, m, b)
		} else {
			merge_up(mut array, a, m, b)
		}
	}
}

[inline]
fn get_reversed_runs[T](mut array []T, a int, b int, limit int) bool {
	mut i := a
	for ; i < b - 1; i++ {
		if array[i] <= array[i + 1] {
			break
		}
	}

	if i - a > limit {
		reverse(mut array, a, i)
	}

	return i == b
}

[direct_array_access]
pub fn stable_sort[T](mut array []T, a int, b int) {
	if get_reversed_runs(mut array, a, b, 8) {
		return
	}

	if b - a > 32 {
		m := a + ((b - a) / 2)

		stable_sort(mut array, a, m)
		stable_sort(mut array, m, b)

		merge(mut array, a, m, b)
	} else {
		insert_sort(mut array, a, b)
	}
}

[inline]
fn find_min_max[T](mut array []T, a int, b int) (T, T) {
	mut min := array[a]
	mut max := array[a]

	for i := a + 1; i < b; i++ {
		if array[i] < min {
			min = array[i]
		} else if array[i] > max {
			max = array[i]
		}
	}

	return min, max
}

[direct_array_access]
pub fn static_sort[T](mut array []T, a int, b int, get fn (T) f32) {
	min, max := find_min_max(mut array, a, b)

	mut len := b - a + 1
	mut cnt := []int{len: len}
	mut pts := []int{len: len}
	len--

	mlt := len / (get(max) - get(min) + 1)

	for i in a .. b {
		cnt[int(f32(get(array[i]) - get(min)) * mlt)]++
	}

	pts[0] = a
	for i in 1 .. len {
		pts[i] = cnt[i - 1] + pts[i - 1]
	}

	for i in 0 .. len {
		for cnt[i] > 0 {
			orig     := pts[i]
			mut from := orig
			mut val  := array[from]

			array[from] = -1

			for {
				d  := int(f32(get(val) - get(min)) * mlt)
				to := pts[d]

				pts[d]++
				cnt[d]--

				tmp      := array[to]
				array[to] = val
				val       = tmp
				from      = to

				if from == orig {
					break
				}
			}
		}
	}

	for i in 0 .. len {
		mut s := 0
		e     := pts[i]

		if i > 0 {
			s = pts[i - 1]
		} else {
			s = a
		}

		if e - s > 1 {
			if e - s > 16 {
				max_heap_sort(mut array, s, e)
			} else {
				unchecked_insert_sort(mut array, s, e)
			}
		}
	}
}