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

package THSSort;

import java.util.ArrayList;
import java.util.List;

public class THSSortLists<T extends Comparable<T>> extends THSSort<T> {
    public static <T> void swap(List<T> array, int a, int b) {
        T tmp = array.get(a);
        array.set(a, array.get(b));
        array.set(b, tmp);
    }

    public static <T> void reverse(List<T> array, int a, int b) {
        for (--b; a < b; a++, b--)
            swap(array, a, b);
    }

    private static <T extends Comparable<T>> void siftDown(List<T> array, int r, int d, int a) {
        while (r <= d / 2) {
            int l = 2 * r;

            if (l < d && array.get(a + l - 1).compareTo(array.get(a + l)) < 0) l++;

            if (array.get(a + r - 1).compareTo(array.get(a + l - 1)) < 0) {
                swap(array, a + r - 1, a + l - 1);
                r = l;
            } else break;
        }
    }

    public static <T extends Comparable<T>> void maxHeapSort(List<T> array, int a, int b) {
        int l = b - a;

        for (int i = l / 2; i > 0; i--)
            siftDown(array, i, l, a);

        for (int i = b - a; i > 1; i--) {
            swap(array, a, a + i - 1);
            siftDown(array, 1, i - 1, a);
        }
    }

    public static <T extends Comparable<T>> void uncheckedInsertionSort(List<T> array, int a, int b) {
        for (int i = a + 1; i < b; i++) {
            if (array.get(i).compareTo(array.get(a)) < 0) swap(array, i, a);

            T key = array.get(i);

            int j = i - 1;
            for (; key.compareTo(array.get(j)) < 0; j--)
                array.set(j + 1, array.get(j));
            array.set(j + 1, key);
        }
    }

    public static <T extends Comparable<T>> void shellSort(List<T> array, int a, int b) {
        for (int k : incs) {
            for (int i = k + a; i < b; i++) {
                T v = array.get(i);

                int j = i;
                for (; j >= k + a && array.get(j - k).compareTo(v) > 0; j -= k)
                    array.set(j, array.get(j - k));
                array.set(j, v);
            }
        }
    }

    public static <T extends Comparable<T>> int partition(List<T> array, int a, int b, int p) {
        int i = a - 1,
            j = b;

        while (true) {
            for (++i; i <  b && array.get(i).compareTo(array.get(p)) < 0; i++);
            for (--j; j >= a && array.get(j).compareTo(array.get(p)) > 0; j--);

            if (i < j) swap(array, i, j);
            else       return j;
        }
    }

    private static <T extends Comparable<T>> void compSwap(List<T> array, int a, int b, int g, int s) {
        int x = s + (a * g),
            y = s + (b * g);

        if (array.get(x).compareTo(array.get(y)) > 0)
            swap(array, x, y);
    }

    public static <T extends Comparable<T>> void medianOfThree(List<T> array, int a, int b) {
        int m = a + (--b - a) / 2;

        compSwap(array, a, m, 1, 0);

        if (array.get(m).compareTo(array.get(b)) > 0) {
            swap(array, m, b);

            if (array.get(a).compareTo(array.get(m)) > 0) return;
        }

        swap(array, a, m);
    }

    public static <T extends Comparable<T>> void medianOfSixteen(List<T> array, int a, int b) {
        int g = (b - 1 - a) / 16,
            m = a + (8 * g);

        for (int i = 0; i < swaps.length; i += 2)
            compSwap(array, swaps[i], swaps[i + 1], g, a);

        swap(array, a, m);
    }

    private static <T extends Comparable<T>> boolean getSortedRuns(List<T> array, int a, int b) {
        boolean rs = true,
                 s = true;

        for (int i = a; i < b - 1; i++) {
            if (array.get(i).compareTo(array.get(i + 1)) > 0) s  = false;
            else                                              rs = false;

            if ((!rs) && (!s)) return false;
        }

        if (rs && !s) {
            reverse(array, a, b);
            return true;
        }

        return s;
    }

    private static <T extends Comparable<T>> void sort(List<T> array, int a, int b, int d, boolean unbalanced) {
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
                        sort(array, a, p, d - 1, true);
                        a = p;
                    } else {
                        sort(array, p + 1, b, d - 1, true);
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

            sort(array, p, b, --d, false);
            b = p;
        }
        uncheckedInsertionSort(array, a, b);
    }

    public static <T extends Comparable<T>> void sort(List<T> array, int a, int b) {
        sort(array, a, b, log2(b - a), false);
    }

    public static <T> void multiSwap(List<T> array, int a, int b, int l) {
        for (int i = 0; i < l; i++)
            swap(array, a + i, b + i);
    }

    public static <T> void multiSwapBW(List<T> array, int a, int b, int l) {
        for (--l; l >= 0; l--)
            swap(array, a + l, b + l);
    }

    public static <T> void insertTo(List<T> array, int from, int to) {
        T tmp = array.get(from);

        for (int i = from - 1; i >= to; i--)
            array.set(i + 1, array.get(i));
        array.set(to, tmp);
    }

    public static <T> void insertToBW(List<T> array, int from, int to) {
        T tmp = array.get(from);

        for (int i = from; i < to; i++)
            array.set(i, array.get(i + 1));
        array.set(to, tmp);
    }

    public static <T extends Comparable<T>> int binarySearch(List<T> array, int a, int b, T v, boolean l) {
        boolean cmp;

        while (a < b) {
            int m = (a + b) / 2;

            if (l) cmp = v.compareTo(array.get(m)) <= 0;
            else   cmp = v.compareTo(array.get(m)) <  0;

            if (cmp) b = m;
            else     a = m + 1;
        }

        return a;
    }

    public static <T> void rotate(List<T> array, int a, int m, int b) {
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

    public static <T extends Comparable<T>> void insertSort(List<T> array, int a, int b) {
        for (int i = a + 1; i < b; i++)
            if (array.get(i).compareTo(array.get(i - 1)) < 0)
                insertTo(array, i, binarySearch(array, a, i - 1, array.get(i), false));
    }

    private static <T extends Comparable<T>> void mergeUp(List<T> array, int a, int m, int b) {
        ArrayList<T> aux = new ArrayList<T>();

        for (int i = 0; i < m - a; i++)
            aux.add(array.get(i + a));

        int l = a,
            r = m;

        for (int nxt = 0; nxt < b - a; nxt++) {
            if (l >= m && r >= b) break;

            if (l < m && r >= b)
                array.set(a + nxt, aux.get((l++) - a));
            else if (l >= m && r < b) break;
            else if (aux.get(l - a).compareTo(array.get(r)) <= 0)
                array.set(a + nxt, aux.get((l++) - a));
            else array.set(a + nxt, array.get(r++));
        }
    }

    private static <T extends Comparable<T>> void mergeDown(List<T> array, int a, int m, int b) {
        ArrayList<T> aux = new ArrayList<T>();

        for (int i = 0; i < b - m; i++)
            aux.add(array.get(i + m));

        int l = m - 1,
            r = b;

        for (int nxt = b - a - 1; nxt >= 0; nxt--) {
            if (l <= a && r <= m) break;

            if (l < a && r >= a)
                array.set(a + nxt, aux.get((r--) - m - 1));
            else if ((l >= a && r < a) || r < m + 1) break;
            else if (array.get(l).compareTo(aux.get(r - m - 1)) <= 0)
                array.set(a + nxt, aux.get((r--) - m - 1));
            else array.set(a + nxt, array.get(l--));
        }
    }

    private static <T extends Comparable<T>> boolean checkBounds(List<T> array, int a, int m, int b) {
        if (array.get(m - 1).compareTo(array.get(m)) <= 0) return true;
        else if (array.get(a).compareTo(array.get(b - 1)) > 0) {
            rotate(array, a, m, b);
            return true;
        }
        return false;
    }

    public static <T extends Comparable<T>> void mergeInPlace(List<T> array, int a, int m, int b) {
        if (m - a <= b - m) {
            int i = a,
                j = m, k;

            while (i < j && j < b) {
                if (array.get(i).compareTo(array.get(j)) > 0) {
                    k = binarySearch(array, j, b, array.get(i), true);
                    rotate(array, i, j, k);
                    i += k - j;
                    j = k;
                } else i++;
            }
        } else {
            int i = m - 1,
                j = b - 1, k;

            while (j > i && i >= a) {
                if (array.get(i).compareTo(array.get(j)) > 0) {
                    k = binarySearch(array, a, i, array.get(j), false);
                    rotate(array, k, i + 1, j + 1);
                    j -= i + 1 - k;
                    i = k - 1;
                } else j--;
            }
        }
    }

    public static <T extends Comparable<T>> void merge(List<T> array, int a, int m, int b) {
        if (checkBounds(array, a, m, b)) return;

        b = binarySearch(array, m,     b, array.get(m - 1), true);
        a = binarySearch(array, a, m - 1, array.get(m),     false);

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

    private static <T extends Comparable<T>> boolean getReversedRuns(List<T> array, int a, int b, int l) {
        int i = a;
        for (; i < b - 1; i++)
            if (array.get(i).compareTo(array.get(i + 1)) <= 0) break;

        if (i - a > l) reverse(array, a, i);

        return i == b;
    }

    public static <T extends Comparable<T>> void stableSort(List<T> array, int a, int b) {
        if (getReversedRuns(array, a, b, 8)) return;
        if (b - a > 32) {
            int m = a + ((b - a) / 2);

            stableSort(array, a, m);
            stableSort(array, m, b);

            merge(array, a, m, b);
        } else insertSort(array, a, b);
    }

    public static <T extends Comparable<T>> MinMaxPair<T> findMinMax(List<T> array, int a, int b) {
        MinMaxPair<T> minMax = new MinMaxPair<T>(array.get(a), array.get(a));

        for (int i = a + 1; i < b; i++) {
            if      (array.get(i).compareTo(minMax.min) < 0)
                minMax.min = array.get(i);
            else if (array.get(i).compareTo(minMax.max) > 0)
                minMax.max = array.get(i);
        }

        return minMax;
    }

    public static <T extends Number & Comparable<T>> void staticSort(List<T> array, int a, int b) {
        MinMaxPair<T> minMax = findMinMax(array, a, b);
        int len = b - a + 1;

        int[] cnt = new int[len];
        int[] pts = new int[len--];

        float min = minMax.min.floatValue();

        float mlt = len / (minMax.max.floatValue() - min + 1);

        for (int i = a; i < b; i++)
            cnt[(int)((float)(array.get(i).floatValue() - min) * mlt)]++;

        pts[0] = a;
        for (int i = 1; i < len; i++)
            pts[i] = cnt[i - 1] + pts[i - 1];

        for (int i = 0; i < len; i++) {
            while (cnt[i] > 0) {
                int orig = pts[i],
                    from = orig;

                T val = array.get(from);
                array.set(from, minMax.min);

                do {
                    int d  = (int)((float)(val.floatValue() - min) * mlt),
                        to = pts[d];

                    pts[d]++;
                    cnt[d]--;

                    T tmp     = array.get(to);
                    array.set(to, val);
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
    }
}
