/*
Copyright (c) 2021 Amari Calipso

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
#pragma once

#include <math.h>
#include <vector>

template <typename T>
struct MinMaxPair {
    T min;
    T max;
};

constexpr int incs[] = {48, 21, 7, 3, 1};
constexpr int medianOfSixteenSwaps[] = {
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
};
constexpr int incsLen = 5, medianOfSixteenSwapsLen = 120;

template <typename T>
int stdcompare(T a, T b) {
    return (a > b) - (b > a);
}

template <typename T>
struct thsSort {
    int (*compare)(T, T);

    thsSort(int (*comparefn)(T, T) = stdcompare<T>) {
        compare = comparefn;
    }
    ~thsSort() {}

    void swap(T* array, int a, int b) {
        T tmp    = array[a];
        array[a] = array[b];
        array[b] = tmp;
    }

    void reverse(T* array, int a, int b) {
        for (--b; a < b; a++, b--)
            swap(array, a, b);
    }

    void siftDown(T* array, int r, int d, int a) {
        while (r <= d / 2) {
            int l = 2 * r;

            if (l < d && compare(array[a + l - 1], array[a + l]) < 0)
                l++;

            if (compare(array[a + r - 1], array[a + l - 1]) < 0) {
                swap(array, a + r - 1, a + l - 1);
                r = l;
            } else break;
        }
    }

    void maxHeapSort(T* array, int a, int b) {
        int l = b - a;

        for (int i = l / 2; i >= 1; i--)
            siftDown(array, i, l, a);

        for (int i = b - a; i > 1; i--) {
            swap(array, a, a + i - 1);
            siftDown(array, 1, i - 1, a);
        }
    }

    void uncheckedInsertionSort(T* array, int a, int b) {
        for (int i = a + 1; i < b; i++) {
            if (compare(array[i], array[a]) < 0)
                swap(array, i, a);

            T key = array[i];

            int j = i - 1;
            for (; compare(key, array[j]) < 0; j--)
                array[j + 1] = array[j];
            array[j + 1] = key;
        }
    }

    void shellSort(T* array, int a, int b) {
        for (int h : incs) {
            for (int i = b + h; i < b; i++) {
                T v = array[i];

                int j = i;
                for (; j >= h && compare(array[j - h], v) > 0; j -= h)
                    array[j] = array[j - h];
                array[j] = v;
            }
        }
    }

    int partition(T* array, int a, int b, int p) {
        int i = a - 1,
            j = b;

        while (true) {
            for (++i; i  < b && compare(array[i], array[p]) < 0; i++);
            for (--j; j >= a && compare(array[j], array[p]) > 0; j--);

            if (i < j) swap(array, i, j);
            else       return j;
        }
    }

    void compSwap(T* array, int a, int b, int g, int s) {
        int x = s + (a * g),
            y = s + (b * g);

        if (compare(array[x], array[y]) > 0)
            swap(array, x, y);
    }

    void medianOfThree(T* array, int a, int b) {
        int m = a + (--b - a) / 2;

        compSwap(array, a, m, 1, 0);
        if (compare(array[m], array[b]) > 0) {
            swap(array, m, b);

            if (compare(array[a], array[m]) > 0) return;
        }

        swap(array, a, m);
    }

    void medianOfSixteen(T* array, int a, int b) {
        int g = (b - 1 - a) / 16;

        for (int i = 0; i < medianOfSixteenSwapsLen; i += 2)
            compSwap(array, medianOfSixteenSwaps[i], medianOfSixteenSwaps[i + 1], g, a);

        swap(array, a, a + (8 * g));
    }

    bool getSortedRuns(T* array, int a, int b) {
        bool rs = true,
              s = true;

        for (int i = a; i < b - 1; i++) {
            if (compare(array[i], array[i + 1]) > 0)
                s = false;
            else rs = false;

            if ((!rs) && (!s)) return false;
        }

        if (rs && !s) {
            reverse(array, a, b);
            return true;
        }

        return s;
    }

    void medianOfSixteenAQSort(T* array, int a, int b, int d, bool unbalanced) {
        while (b - a > 16) {
            if (getSortedRuns(array, a, b)) return;

            if (d == 0) {
                maxHeapSort(array, a, b);
                return;
            }

            int p;
            if (!unbalanced) {
                medianOfThree(array, a, b);
                p = partition(array, a, b, a);
            } else p = a;

            int l = p - a,
                r = b - (p + 1);

            if ((l == 0 || r == 0) || (l / r >= 16 || r / l >= 16) || unbalanced) {
                if (b - a > 80) {
                    swap(array, a, p);

                    if (l < r) {
                        medianOfSixteenAQSort(array, a, p, d - 1, true);
                        a = p;
                    } else {
                        medianOfSixteenAQSort(array, p + 1, b, d - 1, true);
                        b = p;
                    }
                    medianOfSixteen(array, a, b);
                    p = partition(array, a + 1, b, a);
                } else {
                    shellSort(array, a, b);
                    return;
                }
            }

            swap(array, a, p);

            medianOfSixteenAQSort(array, p, b, --d, false);
            b = p;
        }
        uncheckedInsertionSort(array, a, b);
    }

    void sort(T* array, int a, int b) {
        medianOfSixteenAQSort(array, a, b, (int)(2 * log2(b - a)), false);
    }

    void multiSwap(T* array, int a, int b, int l) {
        for (int i = 0; i < l; i++)
            swap(array, a + i, b + i);
    }

    void multiSwapBW(T* array, int a, int b, int l) {
        for (--l; l >= 0; l--)
            swap(array, a + l, b + l);
    }

    void insertTo(T* array, int from, int to) {
        T tmp = array[from];

        for (int i = from - 1; i >= to; i--)
            array[i + 1] = array[i];
        array[to] = tmp;
    }

    void insertToBW(T* array, int from, int to) {
        T tmp = array[from];

        for (int i = from; i < to; i++)
            array[i] = array[i + 1];
        array[to] = tmp;
    }

    int binarySearch(T* array, int a, int b, T val, bool l) {
        bool cmp;

        while (a < b) {
            int m = (a + b) / 2;

            if (l) cmp = compare(val, array[m]) <= 0;
            else   cmp = compare(val, array[m]) <  0;

            if (cmp) b = m;
            else     a = m + 1;
        }

        return a;
    }

    void rotate(T* array, int a, int m, int b) {
        while (b - m > 1 && m - a > 1) {
            if (b - m < m - a) {
                multiSwap(array, a, m, b - m);
                a += b - m;
            } else {
                multiSwapBW(array, a, b - (m - a), m - a);
                b -= m - a;
            }
        }

        if      (b - m == 1) insertTo(array, m, a);
        else if (m - a == 1) insertToBW(array, a, b - 1);
    }

    void insertSort(T* array, int a, int b) {
        for (int i = a + 1; i < b; i++)
            if (array[i] < array[i - 1])
                insertTo(array, i, binarySearch(array, a, i - 1, array[i], false));
    }

    void mergeUp(T* array, int a, int m, int b) {
        T* aux = new T[m - a];

        for (int i = 0; i < m - a; i++)
            aux[i] = array[i + a];

        int l = a,
            r = m;

        for (int nxt = 0; nxt < b - a; nxt++) {
            if (l >= m && r >= b) break;

            if (l < m && r >= b)
                array[a + nxt] = aux[(l++) - a];
            else if (l >= m && r < b) break;
            else if (compare(aux[l - a], array[r]) <= 0)
                 array[a + nxt] =   aux[(l++) - a];
            else array[a + nxt] = array[r++];
        }

        delete[] aux;
    }

    void mergeDown(T* array, int a, int m, int b) {
        T* aux = new T[b - m];

        for (int i = 0; i < b - m; i++)
            aux[i] = array[i + m];

        int l = m - 1,
            r = b;

        for (int nxt = b - a - 1; nxt >= 0; nxt--) {
            if (l <= a && r <= m) break;

            if (l < a && r >= a)
                array[a + nxt] = aux[(r--) - m - 1];
            else if ((l >= a && r < a) || r < m + 1) break;
            else if (compare(array[l], aux[r - m - 1]) <= 0)
                 array[a + nxt] =   aux[(r--) - m - 1];
            else array[a + nxt] = array[l--];
        }

        delete[] aux;
    }

    bool checkBounds(T* array, int a, int m, int b) {
        if (compare(array[m - 1], array[m]) <= 0)
            return true;
        else if (compare(array[a], array[b - 1]) > 0) {
            rotate(array, a, m, b);
            return true;
        }
        return false;
    }

    void mergeInPlace(T* array, int a, int m, int b) {
        if (m - a <= b - m) {
            int i = a,
                j = m, k;

            while (i < j && j < b) {
                if (compare(array[i], array[j]) > 0) {
                    k = binarySearch(array, j, b, array[i], true);
                    rotate(array, i, j, k);
                    i += k - j;
                    j = k;
                } else i++;
            }
        } else {
            int i = m - 1,
                j = b - 1, k;

            while (j > i && i >= a) {
                if (compare(array[i], array[j]) > 0) {
                    k = binarySearch(array, a, i, array[j], false);
                    rotate(array, k, i + 1, j + 1);
                    j -= (i + 1) - k;
                    i = k - 1;
                } else j--;
            }
        }
    }

    void merge(T* array, int a, int m, int b) {
        if (checkBounds(array, a, m, b)) return;

        b = binarySearch(array, m,     b, array[m - 1], true);
        a = binarySearch(array, a, m - 1, array[m],     false);

        if (b - m < m - a) {
            if (b - m <= 8)
                mergeInPlace(array, a, m, b);
            else
                mergeDown(array, a, m, b);
        } else {
            if (m - a <= 8)
                mergeInPlace(array, a, m, b);
            else
                mergeUp(array, a, m, b);
        }
    }

    bool getReversedRuns(T* array, int a, int b, int limit) {
        int i = a;
        for (; i < b - 1; i++)
            if (compare(array[i], array[i + 1]) <= 0) break;

        if (i - a > limit) reverse(array, a, i);

        return (i == b);
    }

    void stableSort(T* array, int a, int b) {
        if (getReversedRuns(array, a, b, 8)) return;
        if (b - a > 32) {
            int m = a + ((b - a) / 2);

            stableSort(array, a, m);
            stableSort(array, m, b);

            merge(array, a, m, b);
        } else insertSort(array, a, b);
    }

    MinMaxPair<T> findMinMax(T* array, int a, int b) {
        MinMaxPair<T> minMax = MinMaxPair<T>{array[a], array[a]};

        for (int i = a + 1; i < b; i++) {
            if      (compare(array[i], minMax.min) < 0)
                minMax.min = array[i];
            else if (compare(array[i], minMax.max) > 0)
                minMax.max = array[i];
        }

        return minMax;
    }

    void staticSort(T* array, int a, int b) {
        MinMaxPair<T> minMax = findMinMax(array, a, b);
        int len = b - a + 1;

        int* cnt = new int[len];
        int* pts = new int[len--];

        float mlt = len / (minMax.max - minMax.min + 1);

        for (int i = a; i < b; i++)
            cnt[(int)((float)(array[i] - minMax.min) * mlt)]++;

        pts[0] = a;
        for (int i = 1; i < len; i++)
            pts[i] = cnt[i - 1] + pts[i - 1];

        for (int v = 0; v < len; v++) {
            while (cnt[v] > 0) {
                int orig = pts[v],
                    from = orig;

                T val = array[from];
                array[from] = -1;

                do {
                    int d  = (int)((float)(val - minMax.min) * mlt),
                        to = pts[d];

                    pts[d]++;
                    cnt[d]--;

                    T tmp     = array[to];
                    array[to] = val;
                    val       = tmp;
                    from      = to;
                } while (from != orig);
            }
        }

        for (int i = 0; i < len; i++) {
            int s = i > 0 ? pts[i - 1] : a,
                e = pts[i];

            if (e - s > 1) {
                if (e - s > 16) maxHeapSort(array, s, e);
                else            uncheckedInsertionSort(array, s, e);
            }
        }

        delete[] cnt;
        delete[] pts;
    }
};
