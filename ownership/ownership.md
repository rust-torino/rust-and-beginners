# rust: &mut Beginners

## Ownership and borrowing

---

# Focus on chapter 4 of the Rust book

## Prerequisites

* Basic knowledge of Rust concepts: variables, loops, conditionals, pattern matching, structs and traits
* The first four chapters of the book

---

# Ownership and borrowing

# Think about clay

---

# Clay

* Alice owns some shaped clay
* She can show (borrow) her shape to many people, but they cannot re-shape it
* She can borrow the clay to John, who can reshape it
* John can show the borrowed shape to other people, but not while he is shaping the clay
* John can re-borrow the Tod, so he can change the shape
* John must return back the clay to Alice
* John could say to Tod that he cannot change the shape. Alice could have said the same.
* Alice could also give the clay to John, which will be the new owner

---

# Reason: memory safety

## Computer and memory: nothing about magic

---

# Memory safety in GC languages

* Garbage collector
* It allocates your memory, it frees your memory
* You never have access to underlying memory
* Intrinsic cost

---

# Memory safety in system languages (without OB system)

* It does not exist (unless using runtime checks, which costs)
* Major concern across all major companies
* Memory errors are easy to make, hard to find

---

# Small digression: stack (and registers) and heap

* Stack & registers: extremely volatile memory
* When you exit a function, stack is "unwound", most of registers are considered scratch
* Local stack persists in inner calls
* Stack management is handled at **compile time**
* Use the heap for other purposes (i.e.: runtime size)
* BSS memory for something in between

---

# Simple as C

## C is sold as a simple language. It is partially true, but maintaining a C project can be hard

---

# Simple as C

```c
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

char *
present(char * name) {
  static char message[16];
  snprintf(message, 16, "I am %s", name);
  return message;
}

char *
hello_world(char * message, char * name) {
  char * present_message = present(name);

  // Size of "Hello world! " = 13
  // Additional 1 is for space
  // Additional 1 is for the string terminator
  size_t const message_len = strlen(present_message) + 15 + strlen(message);

  char * out_message = malloc(message_len);
  sprintf(out_message, "Hello world! %s %s", message, present_message);
  return out_message;
}

int main(int argc, char* argv[]) {
  char * yell = "Oh yeah!";
  char * message = hello_world(yell, argv[1]);
  printf("%s\n", message);
  free(message);
}
```

---

# Not so simple

```c
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

char const *
present(char const * const name) {
  static char message[16];
  int written = snprintf(message, 16, "I am %s", name);
  return written < 0 ? NULL : message;
}

char *
hello_world(char const * const message, char const * const name) {
  char const * const present_message = present(name);

  if (present_message == NULL) {
    return NULL;
  }

  // Size of "Hello world! " = 13
  // Additional 1 is for the string terminator
  size_t message_len = strlen(present_message) + 14;

  char * out_message;
  if (message != NULL) {
    // 1 is for an additional space
    message_len += 1 + strlen(message);
  }

  out_message = malloc(sizeof(char) * message_len);
  size_t written;
  if (message == NULL) {
    written = sprintf(out_message, "Hello world! %s", present_message);
  } else {
    written = sprintf(out_message, "Hello world! %s %s", message, present_message);
  }

  if (written < 0) {
    free(out_message);
    return NULL;
  } else {
    return out_message;
  }
}

int main(int argc, char* argv[]) {
  if (argc != 2) {
    return 0;
  }

  char const * const yell = "Oh yeah!";
  char * message = hello_world(yell, argv[1]);
  if (message != NULL) {
    printf("%s\n", message);
    free(message);
  }
}
```

---

# Say hello to Rust `String`

```rust
let mut s = String::from("hello");
s.push_str(" world!");
```

---

# Strings between languages

* In C: `malloc` + `\0` terminator, `char *` on stack
* In Rust: struct with `ptr`, `len` and `capacity`. Not so different from C++ `std::string`.

---

![](string_layout.svg "Memory layout of a Rust String")

---

# Strings must be owned

* Memory *responsibility*
* Deallocation at the end of scope
* Can work with a borrowed `String`

---

# `&mut String`

```rust
fn main() {
    let mut s = String::from("hello");
    add_world(&mut s);
}

fn add_world(s: &mut String) {
    s.push_str(" world");
}
```

---

# Once you `&mut`, no more refs

```rust
let mut s = String::from("hello");

let s_mut = &mut s;
let another = &mut s; // ERROR
add_world(s);
```

```
error[E0499]: cannot borrow `s` as mutable more than once at a time
  --> src/main.rs:9:19
   |
8  |     let s_mut = &mut s;
   |                 ------ first mutable borrow occurs here
9  |     let another = &mut s;
   |                   ^^^^^^ second mutable borrow occurs here
10 |     add_world(s_mut);
   |               ----- first borrow later used here
```
---

# Once you `&mut`, no more refs (again!!!)

```rust
let mut s = String::from("hello");

let s_mut = &mut s;
let another = &s; // ERROR
add_world(s);
```

```
error[E0502]: cannot borrow `s` as immutable because it is also borrowed as mutable
  --> src/main.rs:9:19
   |
8  |     let s_mut = &mut s;
   |                 ------ mutable borrow occurs here
9  |     let another = &s;
   |                   ^^ immutable borrow occurs here
10 |     add_world(s_mut);
   |               ----- mutable borrow later used here
```

---

# Stronger safety, stronger optimizations

* You cannot accidentally change things
* Compiler **knows** when things cannot change

---

# Working with pieces of a `String`

## Say hello to `&str`

* Reference to a *non-mutable* piece of a string
* `&String` -> `&str` almost "automagically"
* Sort of *fat* pointer (address + size)
* `&'static str` and BSS

---

# From C to Rust

```rust
use std::env;

fn present(name: &str) -> String {
    format!("I am {}", name)
}

fn hello_world(message: &str, name: &str) -> String {
    let present_message = present(name);
    format!("Hello world! {} {}", message, present_message)
}

fn main() {
    let name = env::args().nth(1).unwrap();
    let yell = "Oh yeah!";
    let message = hello_world(yell, &name);
    println!("{}", message);
}
```

---

# Differences

* Simpler and safer
* `present` is different, and it costs more for a reason
* No explicit allocations, even if you have 100% control of memory

---

# Ownership, borrowing and lifetimes

```rust
fn get_trimmed(s: String) -> &str {
    s.trim()
}
```

---

# Ownership, borrowing and lifetimes

```
1 | fn get_trimmer(s: String) -> &str {
  |                              ^ help: consider giving it an explicit bounded or 'static lifetime: `&'static`
```

---

# Ownership, borrowing and lifetimes

```rust
fn get_trimmed(s: String) -> &'static str {
    s.trim()
}
```

---

# Ownership, borrowing and lifetimes

```
error[E0515]: cannot return value referencing function parameter `s`
 --> src/lib.rs:2:5
  |
2 |     s.trim()
  |     -^^^^^^^
  |     |
  |     returns a value referencing data owned by the current function
  |     `s` is borrowed here
```

---

# Ownership, borrowing and lifetimes

* _Abstract_ lifetime attached to everything
* Vanishes at runtime
* With ownership and borrowing, a borrow cannot live more than the owned value
* No, you can't
* No, you are not special

---

# My heap, in a box

```rust
struct SingleLinkedList<T> {
  node: Node<T>,
}

struct Node<T> {
  data: T,
  next: Option<Box<Node<T>>>,
}
```

---

# `Box<T[]>`, `Vec<T>` & slices

```rust
let integers = Box::new([0, 1, 2, 3, 4, 5]);
assert_eq!(&integers[1..4], &[1, 2, 3]);

let mut integers = vec![0, 1, 2];
integers.push(3);
assert_eq!(&integers[1..4], &[1, 2, 3]);
```

Similar to `&str` in relation to `String`

---

# Practice together

---

# Exercise 1

Write a function that finds all the longest contiguous substrings of vowels in a set of strings. The function must take a slice of borrowed strings and must return a vector of tuples. Each tuple must contain the index of the borrowed string in the slice, the start index of a substring and the substring with all the vowels.

---

# How it works

I give you a `.rs` file with one or more tests that must pass. All files can be found [in the github repo](https://github.com/rust-torino/rust-and-beginners).

---

# Let's start

Copy-paste the test

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_all_vowels_slices() {
        /* ... */
    }
}
```

---

# Write the function signature

```rust
pub fn vowels_slices(strings: &[&str]) -> Vec<(usize, usize, &str)> {
    todo!()
}
```

This still does not work: what's the lifetime of the `&str` in the output?

---

# This is the life(time)

```rust
pub fn vowels_slices<'a>(strings: &'a [&'a str]) -> Vec<(usize, usize, &'a str)> {
    todo!()
}
```

---

# Create output, iterate over strings

```rust
let mut out = Vec::new();
for (string_index, string) in strings.iter().enumerate() {
    todo!()
}

out
```

---

# Iterate over chars, check vowels

```rust
let mut out = Vec::new();
for (string_index, string) in strings.iter().enumerate() {
    for (char_index, c) in string.char_indices() {
        match c.to_ascii_lowercase() {
            'a' | 'e' | 'i' | 'o' | 'u' => todo!(),
            _ => {}
        }
    }
}

out
```

---

# Store the start of vowels slice

```rust
let mut out = Vec::new();
for (string_index, string) in strings.iter().enumerate() {
    let mut start = None;
    for (char_index, c) in string.char_indices() {
        match c.to_ascii_lowercase() {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                if start.is_none() {
                    start = Some(char_index)
                }
            }
            _ => todo!(),
        }
    }
}

out
```

---

# Push slices of vowels

```rust
match c.to_ascii_lowercase() {
    /* ... */
    _ => {
        if let Some(slice_start) = start {
            let slice = &string[slice_start..char_index];
            out.push((string_index, slice_start, slice));
            start = None;
        }
    }
}
```

---

# Save last slice when string ends with vowel

```rust
let mut out = Vec::new();
for (string_index, string) in strings.iter().enumerate() {
    let mut start = None;
    for (char_index, c) in string.char_indices() {
        /* ... */
    }

    if let Some(slice_start) = start {
        let slice = &string[slice_start..];
        out.push((string_index, slice_start, slice));
    }
}

out
```

---

# Whole code

```rust
pub fn vowels_slices<'a>(strings: &'a [&'a str]) -> Vec<(usize, usize, &'a str)> {
    let mut out = Vec::new();
    for (string_index, string) in strings.iter().enumerate() {
        let mut start = None;
        for (char_index, c) in string.char_indices() {
            match c.to_ascii_lowercase() {
                'a' | 'e' | 'i' | 'o' | 'u' => {
                    if start.is_none() {
                        start = Some(char_index)
                    }
                }
                _ => {
                    if let Some(slice_start) = start {
                        let slice = &string[slice_start..char_index];
                        out.push((string_index, slice_start, slice));
                        start = None;
                    }
                }
            }
        }

        if let Some(slice_start) = start {
            let slice = &string[slice_start..];
            out.push((string_index, slice_start, slice));
        }
    }

    out
}
```

---

# Different approach, with iterators + one downside

```rust
use std::{convert::identity, iter};

pub fn vowels_slices<'a>(strings: &'a [&'a str]) -> Vec<(usize, usize, &'a str)> {
    strings
        .iter()
        .enumerate()
        .flat_map(|(string_index, s)| {
            s.char_indices()
                .chain(iter::once((s.len(), '!')))
                .scan(None, move |start, (char_index, c)| {
                    match c.to_ascii_lowercase() {
                        'a' | 'e' | 'i' | 'o' | 'u' => {
                            if start.is_none() {
                                *start = Some(char_index);
                            }
                            Some(None)
                        }
                        _ => match *start {
                            Some(start_index) => {
                                *start = None;
                                Some(Some((
                                    string_index,
                                    start_index,
                                    &s[start_index..char_index],
                                )))
                            }
                            None => Some(None),
                        },
                    }
                })
                .filter_map(identity)
        })
        .collect()
}
```

---

# Practice by yourself

Pair programming is appreciated and suggested

---

# Exercise 2

Count the occurrences of longest slices of vowels. Must be returned something that can be transformed into an iterator of tuples, where the first element is a (direct or indirect) reference of a substring in the given data, the second the number of occurrences of the slice. The comparison must be case insensitive (only for ASCII vowels).

---

# Exercise 3

A slice of numbers is given. We define a slice of numbers A with the length of 5, and we defined the operation `sum` that sums together all the values of the slice. For each A, it could exist a slice B that does not overlap with A so that `sum(A) == sum(B)`. If for the slice A exist many slices B, the longest slice must be taken, and between slices with the same length, the slice nearest the end of the string must be taken.
Given a slice of numbers, write a function that returns all pairs of `(A, B)` as described above.

---

# Thanks!

---

---

# Stop! Solutions ahead

---

# Hey, you've been warned!

---

# Exercise 2

---

```rust
pub fn vowels_slices_occurrences<'a>(strings: &'a [&'a str]) -> BTreeMap<StrWrap<'a>, u32> {
    let mut occurrences = BTreeMap::new();

    let mut inc_slice = |slice| {
        occurrences
            .entry(StrWrap(slice))
            .and_modify(|count| *count += 1)
            .or_insert(1);
    };

    for string in strings {
        let mut start = None;
        string
            .char_indices()
            .map(|(index, c)| (index, c.to_ascii_lowercase()))
            .for_each(|(index, c)| match c {
                'a' | 'e' | 'i' | 'o' | 'u' => {
                    if start.is_none() {
                        start = Some(index);
                    }
                }
                _ => {
                    if let Some(start_index) = start {
                        inc_slice(&string[start_index..index]);
                        start = None;
                    }
                }
            });

        if let Some(start) = start {
            inc_slice(&string[start..]);
        }
    }
    occurrences
}
```

---

```rust
#[derive(Copy, Clone)]
pub struct StrWrap<'a>(&'a str);

impl AsRef<str> for StrWrap<'_> {
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl Deref for StrWrap<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl PartialOrd for StrWrap<'_> {
    fn partial_cmp(&self, other: &StrWrap<'_>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for StrWrap<'_> {
    fn cmp(&self, other: &StrWrap<'_>) -> Ordering {
        self.0
            .chars()
            .zip(other.0.chars())
            .map(|(a, b)| a.to_ascii_lowercase().cmp(&b.to_ascii_lowercase()))
            .find(|order| !matches!(order, Ordering::Equal))
            .unwrap_or_else(|| self.0.len().cmp(&other.0.len()))
    }
}

impl PartialEq for StrWrap<'_> {
    fn eq(&self, other: &StrWrap<'_>) -> bool {
        self.0.eq_ignore_ascii_case(other.0)
    }
}

impl Eq for StrWrap<'_> {}
```

---

```rust
impl Ord for StrWrap<'_> {
    fn cmp(&self, other: &StrWrap<'_>) -> Ordering {
        self.0
            .chars()
            .zip(other.0.chars())
            .map(|(a, b)| a.to_ascii_lowercase().cmp(&b.to_ascii_lowercase()))
            .find(|order| !matches!(order, Ordering::Equal))
            .unwrap_or_else(|| self.0.len().cmp(&other.0.len()))
    }
}

impl PartialEq for StrWrap<'_> {
    fn eq(&self, other: &StrWrap<'_>) -> bool {
        self.0.eq_ignore_ascii_case(other.0)
    }
}
```

---

# Exercise 3

---

```rust
pub fn all_same_sum_tuples(data: &[u32]) -> Vec<(&[u32; WIN_SIZE], &[u32])> {
    data.windows(WIN_SIZE)
        .enumerate()
        .map(|(start_index, window)| {
            (
                <&[u32; WIN_SIZE]>::try_from(window).unwrap(),
                data.iter().skip(WIN_SIZE + start_index),
                start_index + WIN_SIZE,
            )
        })
        .filter_map(|(window, rest_iter, rest_start_index)| {
            let window_sum: u32 = window.iter().copied().sum();
            rest_iter
                .enumerate()
                .filter(|&(_, x)| x <= &window_sum)
                .map(|(rest_offset, &x)| (rest_offset + rest_start_index, x))
                .with_iter()
                .filter_map(|((first_index, x), rest)| {
                    rest.scan(x, |acc, (index, cur)| {
                        *acc += cur;
                        Some((index, *acc))
                    })
                    .take_while(|&(_, x)| x <= window_sum)
                    .last()
                    .filter(|&(_, sum)| sum == window_sum)
                    .map(|(last_index, _)| &data[first_index..=last_index])
                })
                .enumerate()
                .max_by(|(index_a, slice_a), (index_b, slice_b)| {
                    slice_a
                        .len()
                        .cmp(&slice_b.len())
                        .then_with(|| index_a.cmp(index_b))
                })
                .map(|(_, slice)| (window, slice))
        })
        .collect()
}
```

---

```rust
const WIN_SIZE: usize = 5;

trait WithIterExt: Sized {
    fn with_iter(self) -> WithIter<Self>;
}

impl<I> WithIterExt for I
where
    I: Iterator,
{
    fn with_iter(self) -> WithIter<Self> {
        WithIter(self)
    }
}

struct WithIter<I>(I);

impl<I> Iterator for WithIter<I>
where
    I: Iterator + Clone,
{
    type Item = (<I as Iterator>::Item, I);
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|value| (value, self.0.clone()))
    }
}
```

---

# Thanks again!

---
