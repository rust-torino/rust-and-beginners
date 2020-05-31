# rust: &mut Beginners

## Ownership and borrowing

---

# Focus on chapter 4 of the Rust book

## Prerequisite: the first four chapters of the book

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

## C is sold as a simple language. It is partially true, but mantaining a C project can be hard

---

# Simple as C

```c
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

char *present(char *name) {
  static char message[16];
  snprintf(message, 16, "I am %s", name);
  return message;
}

char *hello_world(char *message, char *name) {
  char *present_message = present(name);

  // Size of "Hello world! " = 13
  // Additional 1 is for space
  // Additional 1 is for the string terminator
  size_t const message_len = strlen(present_message) + 15 + strlen(message);

  char *out_message = malloc(message_len);
  sprintf(out_message, "Hello world! %s %s", message, present_message);
  return out_message;
}

int main(int argc, char *argv[]) {
  char *yell = "Oh yeah!";
  char *message = hello_world(yell, argv[1]);
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

char const *present(char const *const name) {
  static char message[16];
  int written = snprintf(message, 16, "I am %s", name);
  return written < 0 ? NULL : message;
}

char *hello_world(char const *const message, 
                  char const *const name) {
  char const *const present_message = present(name);

  if (present_message == NULL) {
    return NULL;
  }

  // Size of "Hello world! " = 13
  // Additional 1 is for the string terminator
  size_t message_len = strlen(present_message) + 14;

  char *out_message;
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

  char const *const yell = "Oh yeah!";
  char *message = hello_world(yell, argv[1]);
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
