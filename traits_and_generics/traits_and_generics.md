# rust: &mut Beginners

## Traits and Generics

---

# Focus on chapter 10 of the Rust book

## Prerequisites

* Basic knowledge of the major features of the language: types and functions in particular.
* The first ten chapters of the book (extremely related to the previous point)

---

# Traits

---

# Traits as _expected behavior_

* I want to say that my type *behaves* in a certain way
* I want a parameter that *behaves* in a certain way
* I don't care what something **is**, I care about what it **does**

---

# Trait vs inheritance

* With inheritance, I say that `Dog` **is** an `Animal` (because of subtyping)
* With inheritance, I say that my function accepts something that **is** an `Animal`

* With traits, I say that `Dog` **behaves** like an `Animal`
* With traits, I say that my function accepts something that **behaves** like an `Animal`

---

# Traits seen as _abstract class_

* In Java, for instance, abstract classes can express a similar intent
* Still based on _Object oriented programming_, which is a different paradigm
* Still need to use mixins if you are interested in obtaining similar behavior
* Traits available from Java 8, Java 10 has `var` to _combine traits_ (but I don't know Java, sorry)

---

# Traits as traits

* Stop using them _by comparison_ -- traits are traits, nothing else
* Traits are everywhere
* Traits are extremely powerful for express intents in a fine-grained way
* Traits full powerfulness must be still unlocked (see later)

---

# A brief overview

```rust
trait Animal {
  fn walk(&self);
  fn run(&self);
  fn eat(&mut self);
}

struct Dog {
  age: u8,
  vitality: u8,
}

impl Animal for Dog {
  fn walk(&self) {
    todo!()
  }
  
  fn run(&self) {
    todo!()
  }
  
  fn eat(&mut self) {
    todo!()
  }
}
```

---

# Animals can be eaten too

## Associated types

```rust
trait Animal {
  /* ... */  
  type MaterialWhenEaten;
  fn being_eaten(self) -> Self::MaterialWhenEaten;
}

impl Animal for Dog {
  /* ... */
  type MaterialWhenEaten = Flesh;

  fn being_eaten(self) -> Self::MaterialWhenEaten {
    Flesh
  }
}

struct Flesh;
```

---

# Associated consts

```rust
trait Num {
    const ZERO: Self;
}

impl Num for i32 {
    const ZERO: Self = 0;
}

impl Num for f32 {
    const ZERO: Self = 0.;
}

impl Num for f64 {
    const ZERO: Self = 0.;
}
```

---

# Using traits: toward generics

In Rust, we can explicitly express if we want to have a type at compile-time or at run-time

---

# Like in OOP

```rust
fn animal_eats(animal: &mut dyn Animal) {
  animal.eat();
}
```

---

# Runtime polymorphism

* You can call the function with a mutable reference of _something_ that implements `Animal`
* Only one compiled version in the binary (no monomorphization involved)
* The _real_ type accessed through a virtual table and pointer dereferences behind the scenes
* This only work with the initial example, without the associated type

---

# With the associated type

```rust
fn animal_eats(animal: &mut dyn Animal<MaterialWhenEaten=Flesh>) {
  animal.eat();
}
```

You need to express the type!

---

# More levels of runtime polymorphism

```rust
trait Eatable {}

fn animal_eats(animal: &mut dyn Animal<MaterialWhenEaten=Box<dyn Eatable>>) {
  animal.eat();
}
```

Yuch!

---

# `dyn T` is `!Sized`

## A _container_ is needed for everything `!Sized`

---

# Expressing `Sized` trait bounds

* `trait A: ?Sized {}`
* `trait A: Sized {}`

---

# About sizeness

* When you write a trait, it is automatically `?Sized`. If you need it to imply `Sized`, you need to specify it
* When you write a trait bound, it is automatically `Sized`. If you allow a `?Sized` trait, you need to specify it

---

# Expressing trait bounds

```rust
trait Walk {
  fn walk(&self);
}

trait Run: Walk {
  fn run(&self);
}
```

---

# I won't pay!

## From run-time to compile time with generics

---

# Accepting generics arguments

```rust
fn do_walk<W: Walk>(walkable: W) {
  walkable.walk();
}
```

---

# Using `where` clause

```rust
fn do_walk<W>(walkable: W)
where
  W: Walk,
{
  walkable.walk();
}
```

---

# Specify associated type trait bound

```rust
trait Eatable {
  type MaterialWhenEaten;

  fn being_eaten(self) -> Self::MaterialWhenEaten;
}

fn do_eat<E>(other: E)
where
  E: Eatable<MaterialWhenEaten = Flesh>,
{
  other.being_eaten();
}
```

---

# Being generic over associated type

```rust
fn do_eat<E, P>(other: E)
where
  E: Eatable<MaterialWhenEaten = P>,
  P: ToProteins,
{
  other.being_eaten();
}
```

---

# Bind only your traits!

* Use generics bounds on your traits
* **Don't use** bounds on your structs -- leave generics unbound if you can

---

# Using `impl Trait`

```rust
fn do_eat(other: impl Eatable<MaterialWhenEaten = impl ToProteins>)
{
  other.being_eaten();
}
```

---

# Explicit generics vs `impl Trait`

* With an explicit generic, the caller _decides_ the type
* With an `impl Trait`, the function _decides_ the type

---

# Good usage of `impl Trait`

## Returning an `impl Iterator`

```rust
trait Iterator {
  type Item;

  fn next(&mut self) -> Option<Self::Item>;
  /* + a lot of default implemented methods */
}
```

---

# Good usage of `impl Trait` with iterators

```rust
fn do_iter(&self) -> impl Iterator<Item = &A> {
  /* ... */
}
```

* The caller **must not** be able to choose what the function returns
* An _opaque_ type is returned (you can change the type without having an API break)
* Iterator types can be very complex, this return type is trivial

---

# Higher-Rank Trait Bounds (HRTBs)

```rust
fn call_on_ref_zero<F>(f: F)
where
  //     This lifetime is sort of unbound
  //     |
  //     v
  F: for<'a> Fn(&'a i32)
{
    let zero = 0;
    f(&zero);
}
```

---

# Understanding the limits of traits

---

# Orphan rules

* You can impl **your** trait for **your** struct
* You can impl **your** trait for foreign structs
* You can impl foreign traits for **your** struct
* You **cannot** impl foreign traits for foreign structs

---

# Object safety for `dyn Trait`

A trait is object safe if all the methods follow the following rules:

* The return type isn’t Self.
* There are no generic type parameters.

---

# We don't have Generic Associated Types (GATs)

You cannot write this, for now:

```rust
trait StreamingIterator {
    type Item<'a>;
    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>;
}
```
