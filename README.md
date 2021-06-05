sortstat
========
An experiment on sort algorithms real-world complexity.  
The application runs sorting algorithms on a certain number of vectors of increasing size, tries to scale each hardcoded
complexity according to duration of sorting of the minimal vector and find which complexity happen to be close to reality.
Because real runtime of each sort algorithm differs dramatically, the initial 
vector size is selected to be sorted in about 1000 ms.

Example run:
```
% ./sortstat
Testing StdSorter...
40265320 elements: 1877 ms
48318384 elements: 2262 ms
57982064 elements: 2825 ms
69578480 elements: 3392 ms
83494176 elements: 4103 ms
100193016 elements: 4950 ms
120231624 elements: 6066 ms
StdSorter is close to O(n*log n)
Testing StdSorterUnstable...
80530640 elements: 2025 ms
96636768 elements: 2483 ms
115964128 elements: 2976 ms
139156960 elements: 3501 ms
166988352 elements: 4328 ms
200386032 elements: 5226 ms
240463248 elements: 6274 ms
StdSorterUnstable close to O(n*log n)
Testing SelectSorter...
39321 elements: 1149 ms
47185 elements: 1688 ms
56622 elements: 2485 ms
67946 elements: 3642 ms
81535 elements: 5483 ms
97842 elements: 7979 ms
117410 elements: 11662 ms
SelectSorter close to O(n^2)
Testing BubbleSorter...
78643 elements: 2294 ms
94371 elements: 3287 ms
113245 elements: 4771 ms
135894 elements: 6877 ms
163072 elements: 9761 ms
195686 elements: 14174 ms
234823 elements: 20493 ms
BubbleSorter close to O(n^2)
Testing MergeSorter...
10066330 elements: 1259 ms
12079596 elements: 1513 ms
14495516 elements: 2050 ms
17394620 elements: 2365 ms
20873544 elements: 2704 ms
25048254 elements: 3300 ms
30057906 elements: 4285 ms
MergeSorter close to O(n*log n)
Testing QuickSorter...
19660 elements: 1788 ms
23592 elements: 2499 ms
28310 elements: 1912 ms
33972 elements: 6741 ms
40766 elements: 7692 ms
48919 elements: 8318 ms
58702 elements: 16798 ms
QuickSorter close to O(n^2)
Testing RadixSorter...
5033165 elements: 1524 ms
6039798 elements: 1831 ms
7247758 elements: 2159 ms
8697310 elements: 2626 ms
10436772 elements: 3140 ms
12524127 elements: 3785 ms
15028953 elements: 4473 ms
RadixSorter close to O(kn) where k=20
```