"""
Copyright (c) 2021-2022 thatsOven

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
"""

from math import log2

class thsSort:
    incs = (48, 21, 7, 3, 1)
    swaps = (
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
        1, 3, 5, 7, 9, 11, 13, 15, 2, 4, 6, 8, 10, 12, 14, 16,
        1, 5, 9, 13, 2, 6, 10, 14, 3, 7, 11, 15, 4, 8, 12, 16,
        1, 9, 2, 10, 3, 11, 4, 12, 5, 13, 6, 14, 7, 15, 8, 16,
        6, 11, 7, 10, 4, 13, 14, 15, 8, 12, 2, 3, 5, 9,
        2, 5, 8, 14, 3, 9, 12, 15, 6, 7, 10, 11,
        3, 5, 12, 14, 4, 9, 8, 13,
        7, 9, 11, 13, 4, 6, 8, 10,
        4, 5, 6, 7, 8, 9, 10, 11, 12, 13,
        7, 8, 9, 10
    )

    @classmethod
    def reverse(self, array, a, b):
        b -= 1

        while a < b:
            array[a], array[b] = array[b], array[a]
            a += 1
            b -= 1

    @classmethod
    def siftDown(self, array, r, d, a):
        while r <= d // 2:
            l = 2 * r

            if l < d and array[a + l - 1] < array[a + l]:
                l += 1

            if array[a + r - 1] < array[a + l - 1]:
                array[a + r - 1], array[a + l - 1] = (
                array[a + l - 1], array[a + r - 1])

                r = l
            else: break

    @classmethod
    def maxHeapSort(self, array, a, b):
        l = b - a

        for i in range(l // 2, 0, -1):
            self.siftDown(array, i, l, a)

        for i in range(b - a, 1, -1):
            array[a], array[a + i - 1] = array[a + i - 1], array[a]
            self.siftDown(array, 1, i - 1, a)

    @classmethod
    def uncheckedInsertionSort(self, array, a, b):
        for i in range(a + 1, b):
            if array[i] < array[a]:
                array[i], array[a] = array[a], array[i]

            key = array[i]

            j = i - 1
            while key < array[j]:
                array[j + 1] = array[j]
                j -= 1
            array[j + 1] = key

    @classmethod
    def shellSort(self, array, a, b):
        for k in self.incs:
            for i in range(k + a, b):
                v = array[i]
                j = i

                while j >= k + a and array[j - k] > v:
                    array[j] = array[j - k]
                    j -= k
                array[j] = v

    @classmethod
    def partition(self, array, a, b, p):
        i = a - 1
        j = b

        while True:
            i += 1
            while i < b and array[i] < array[p]: i += 1

            j -= 1
            while j >= a and array[j] > array[p]: j -= 1

            if i < j: array[i], array[j] = array[j], array[i]
            else: return j

    @classmethod
    def compSwap(self, array, a, b, g, s):
        x = s + (a * g)
        y = s + (b * g)

        if array[x] > array[y]:
            array[x], array[y] = array[y], array[x]

    @classmethod
    def medianOfThree(self, array, a, b):
        b -= 1
        m = a + (b - a) // 2

        self.compSwap(array, a, m, 1, 0)

        if array[m] > array[b]:
            array[m], array[b] = array[b], array[m]

            if array[a] > array[m]: return

        array[a], array[m] = array[m], array[a]

    @classmethod
    def medianOfSixteen(self, array, a, b):
        g = (b - 1 - a) // 16
        m = a + (8 * g)

        for i in range(0, len(self.swaps), 2):
            self.compSwap(array, self.swaps[i], self.swaps[i + 1], g, a)

        array[a], array[m] = array[m], array[a]

    @classmethod
    def getSortedRuns(self, array, a, b):
        rs = True
        s  = True

        for i in range(a, b - 1):
            if array[i] > array[i + 1]:
                s = False
            else: rs = False

            if (not rs) and (not s): return False

        if rs and not s:
            self.reverse(array, a, b)
            return True

        return s

    @classmethod
    def __sort(self, array, a, b, d, unbalanced):
        while b - a > 16:
            if self.getSortedRuns(array, a, b): return

            if d == 0:
                self.maxHeapSort(array, a, b)
                return

            if not unbalanced:
                self.medianOfThree(array, a, b)
                p = self.partition(array, a, b, a)
            else: p = a

            l = p - a
            r = b - (p + 1)

            if (l == 0 or r == 0) or (l / r >= 16 or r / l >= 16) or unbalanced:
                if b - a > 80:
                    array[a], array[p] = array[p], array[a]

                    if l < r:
                        self.__sort(array, a, p, d - 1, True)
                        a = p
                    else:
                        self.__sort(array, p + 1, b, d - 1, True)
                        b = p

                    self.medianOfSixteen(array, a, b)
                    p = self.partition(array, a + 1, b, a)
                else:
                    self.shellSort(array, a, b)
                    return

            array[a], array[p] = array[p], array[a]

            d -= 1
            self.__sort(array, p, b, d, False)
            b = p
        self.uncheckedInsertionSort(array, a, b)

    @classmethod
    def sort(self, array, a, b):
        self.__sort(array, a, b, int(2 * log2(b - a)), False)

    @classmethod
    def multiSwap(self, array, a, b, l):
        for i in range(l):
            x = a + i
            y = b + i

            array[x], array[y] = array[y], array[x]

    @classmethod
    def multiSwapBW(self, array, a, b, l):
        for i in range(l - 1, -1, -1):
            x = a + i
            y = b + i

            array[x], array[y] = array[y], array[x]

    @classmethod
    def insertTo(self, array, from_, to):
        tmp = array[from_]

        for i in range(from_ - 1, to - 1, -1):
            array[i + 1] = array[i]
        array[to] = tmp

    @classmethod
    def insertToBW(self, array, from_, to):
        tmp = array[from_]

        for i in range(from_, to):
            array[i] = array[i + 1]
        array[to] = tmp

    @classmethod
    def binarySearch(self, array, a, b, val, l):
        while a < b:
            m = (a + b) // 2

            if l:
                if val <= array[m]: b = m
                else:               a = m + 1
            else:
                if val < array[m]: b = m
                else:              a = m + 1

        return a

    @classmethod
    def rotate(self, array, a, m, b):
        while b - m > 1 and m - a > 1:
            if b - m < m - a:
                self.multiSwap(array, a, m, b - m)
                a += b - m
            else:
                self.multiSwapBW(array, a, b - (m - a), m - a)
                b -= m - a

        if   b - m == 1: self.insertTo(array, m, a)
        elif m - a == 1: self.insertToBW(array, a, b - 1)

    @classmethod
    def insertSort(self, array, a, b):
        for i in range(a + 1, b):
            if array[i] < array[i - 1]:
                self.insertTo(array, i, self.binarySearch(array, a, i - 1, array[i], False))

    @classmethod
    def mergeUp(self, array, a, m, b):
        aux = [array[i] for i in range(a, m)]

        l = a
        r = m

        for nxt in range(b - a):
            if l >= m and r >= b: break

            if l < m and r >= b:
                array[a + nxt] = aux[l - a]
                l += 1
            elif l >= m and r < b: break
            elif aux[l - a] <= array[r]:
                array[a + nxt] = aux[l - a]
                l += 1
            else:
                array[a + nxt] = array[r]
                r += 1

    @classmethod
    def mergeDown(self, array, a, m, b):
        aux = [array[i] for i in range(m, b)]

        l = m - 1
        r = b

        for nxt in range(b - a - 1, -1, -1):
            if l <= a and r <= m: break

            if l < a and r >= a:
                array[a + nxt] = aux[r - m - 1]
                r -= 1
            elif (l >= a and r < a) or r < m + 1: break
            elif array[l] <= aux[r - m - 1]:
                array[a + nxt] = aux[r - m - 1]
                r -= 1
            else:
                array[a + nxt] = array[l]
                l -= 1

    @classmethod
    def checkBounds(self, array, a, m, b):
        if array[m - 1] <= array[m]:
            return True
        elif array[a] > array[b - 1]:
            self.rotate(array, a, m, b)
            return True
        return False

    @classmethod
    def mergeInPlace(self, array, a, m, b):
        if m - a <= b - m:
            i = a
            j = m

            while i < j and j < b:
                if array[i] > array[j]:
                    k = self.binarySearch(array, j, b, array[i], True)
                    self.rotate(array, i, j, k)
                    i += k - j
                    j = k
                else: i += 1
        else:
            i = m - 1
            j = b - 1

            while j > i and i >= a:
                if array[i] > array[j]:
                    k = self.binarySearch(array, a, i, array[j], False)
                    self.rotate(array, k, i + 1, j + 1)
                    j -= i + 1 - k
                    i = k - 1
                else: j -= 1

    @classmethod
    def merge(self, array, a, m, b):
        if self.checkBounds(array, a, m, b): return

        b = self.binarySearch(array, m,     b, array[m - 1], True)
        a = self.binarySearch(array, a, m - 1, array[m],     False)

        if b - m < m - a:
            if b - m <= 8: self.mergeInPlace(array, a, m, b)
            else: self.mergeDown(array, a, m, b)
        else:
            if m - a <= 8: self.mergeInPlace(array, a, m, b)
            else: self.mergeUp(array, a, m, b)

    @classmethod
    def getReversedRuns(self, array, a, b, limit):
        i = a
        while i < b - 1:
            if array[i] <= array[i + 1]: break
            i += 1

        if i - a > limit:
            self.reverse(array, a, i)

        return i == b

    @classmethod
    def stableSort(self, array, a, b):
        if self.getReversedRuns(array, a, b, 8): return
        if b - a > 32:
            m = a + ((b - a) // 2)

            self.stableSort(array, a, m)
            self.stableSort(array, m, b)

            self.merge(array, a, m, b)
        else: self.insertSort(array, a, b)

    @classmethod
    def findMinMax(self, array, a, b):
        currMin = array[a]
        currMax = array[a]
        for i in range(a + 1, b):
            if   array[i] < currMin:
                currMin = array[i]
            elif array[i] > currMax:
                currMax = array[i]
        return currMin, currMax

    @classmethod
    def staticSort(self, array, a, b):
        min_, max_ = self.findMinMax(array, a, b)

        auxLen = b - a

        count  = [0 for _ in range(auxLen + 1)]
        offset = [0 for _ in range(auxLen + 1)]

        CONST = auxLen / (max_ - min_ + 1)

        for i in range(a, b):
            count[int((array[i] - min_) * CONST)] += 1

        offset[0] = a

        for i in range(1, auxLen):
            offset[i] = count[i - 1] + offset[i - 1]

        for v in range(auxLen):
            while count[v] > 0:
                orig  = offset[v]
                from_ = orig
                num   = array[from_]

                array[from_] = -1

                while True:
                    dig = int((num - min_) * CONST)
                    to  = offset[dig]

                    offset[dig] += 1
                    count[dig]  -= 1

                    tmp       = array[to]
                    array[to] = num
                    num       = tmp
                    from_     = to

                    if from_ == orig: break

        for i in range(auxLen):
            s = offset[i - 1] if i > 0 else a
            e = offset[i]

            if e - s <= 1:
                continue

            if e - s > 16:
                self.maxHeapSort(array, s, e)
            else:
                self.uncheckedInsertionSort(array, s, e)
