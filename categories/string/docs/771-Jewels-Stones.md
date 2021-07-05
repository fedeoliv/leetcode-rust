# Jewels and Stones [[771]](https://leetcode.com/problems/jewels-and-stones/)

## Problem

You're given strings `jewels` representing the types of stones that are jewels, and `stones` representing the stones you have. Each character in `stones` is a type of stone you have. You want to know how many of the stones you have are also jewels.

Letters are case sensitive, so `"a"` is considered a different type of stone from `"A"`.
 
### **Example 1**

```
Input: jewels = "aA", stones = "aAAbbbb"
Output: 3
```

### **Example 2**

```
Input: jewels = "z", stones = "ZZ"
Output: 0
```

### **Constraints**

* 1 <= jewels.length, stones.length <= 50
* jewels and stones consist of only English letters.
* All the characters of jewels are **unique**.

## Solution

### **Naive approach**

The naive approach to solve is traverse the string `stones`, and for each character `c` that represents the stone type, check whether it exists on the string `jewels` or not. If it exists, increments a counter. 

#### Detailed algorithm

1. Initialize a jewels counter variable as 0. 
2. For each character `c` in the `stones` string:
  * For each character `j` in the `jewels` string:
    * 2.1.1. If `c` = `j`, then increment the jewels counter and break the inner loop.
    * 2.1.2.Otherwise, continue the inner loop.
3. Return the jewels counter

#### Complexity

| Type | Value | Notes |
|------|-------|-------|
| Time | O(M * N) | Assuming `stones` has size `M` and `jewels` has size `N` |
| Space | O(1) | No additional resources added on memory. |

### **Second Approach**

In the previous approach we traversed the `jewels` string to check whether the stone type exists or not, which takes O(N) in the worst scenario. Another approach is to store each character of the `jewels` string in a HashSet, so we can efficiently access data in O(1).

#### Detailed algorithm

1. Initialize a jewels counter variable as 0.
2. Create a new jewels HashSet instance.
3. For each character `j` in the `jewels` string:
  * Insert `j` in the jewels HashSet
4. For each character `c` in the `stones` string:
  * Check if `c` exists in the jewels HashSet
    * If yes, increment the jewels counter
5. Return tye jewels counter.

#### Complexity

| Type | Value | Notes |
|------|-------|-------|
| Time | O(M + N) | Assuming `stones` has size `M` and `jewels` has size `N` |
| Space | O(N) | Considering the jewels HashSet with size `N` |

The source code is available [here](../src/771-jewels-stones.rs).
