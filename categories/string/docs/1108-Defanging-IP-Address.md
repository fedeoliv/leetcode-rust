# Defanging an IP address [[1108]](https://leetcode.com/problems/defanging-an-ip-address/)

## Problem

Given a valid (IPv4) IP `address`, return a defanged version of that IP address. A defanged IP address replaces every period `"."` with `"[.]"`.

### **Example 1**

```
Input: address = "1.1.1.1"
Output: "1[.]1[.]1[.]1"
```

### **Example 2**

```
Input: address = "255.100.50.0"
Output: "255[.]100[.]50[.]0"
```

### **Constraints**

The given address is a valid IPv4 address.

## Solution

The problem essentially is replace every occurence of `"."` by `"[.]"` given an input that represents an IPv4 address in string format. This problem can easily be solved by iterating over the original string and, for every occurence of `'.'` character, append `"[.]"` on a new string. For the characters that don't match this criteria, they will simply be appended on the new string.

### Detailed algorithm

1. Create an empty output string that represents the defanged version of the IP address.
2. Iterate over the original IP address string
  * If the character is `'.'`, append `"[.]"` on the output string.
  * Otherwsie, append the character as is on the output string.
3. Return the output string.

### Complexity

| Type | Value | Notes |
|------|-------|-------|
| Time | O(N) | Iteration over a string of a size `N` |
| Space | O(N) | Creation of a character array (string) with size `N` |

The source code is available [here](../src/1108.rs).

## Considerations

* Another simple and minimalistic approach is using a regular expression. The downside is that [regex](https://docs.rs/regex/1.5.4/regex/) is an external crate - not acceptable on LeetCode.
