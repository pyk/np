# Data Type?
Do we need introduce new data type for multidimensional array?
I think we don't need that.

Let's just use:

    Vec<Vec<T>> as 2D Array
    Vec<Vec<Vec<T>>> as 3D Array

Our focus will be on 2D array and 3D array creation like:

    zeros()

that can return 2D array filled with zeros. Is it possible?
we will se.

If we only use `Vec<Vec<T>>` or `Vec<Vec<Vec<T>>>`, we will
not have an information about the array it self. Like, it is
two dimensional or tree dimensional?

Yeah, but we don't need to keep that information right?
let's see.

btw, we can't use one function `zeros()` to build zero-valued
two dimensional and three dimensional array because rust
is [doesn't support function overloading](https://blog.rust-lang.org/2015/05/11/traits.html).

> *Overloading*. Rust does not support traditional overloading
> where the same method is defined with multiple signatures.
> But traits provide much of the benefit of overloading: if
> a method is defined generically over a trait, it can be
> called with any type implementing that trait. Compared to
> traditional overloading, this has two advantages. First,
> it means the overloading is less ad hoc: once you understand
> a trait, you immediately understand the overloading pattern
> of any APIs using it. Second, it is extensible: you can
> effectively provide new overloads downstream from a method
> by providing new trait implementations.

So, we can use trait to solve this right? but how?

# with_size, with_shape, one_dim, two_dim ...

We will replace `Vec::with_size(x)` and `Vec::with_shape(&[x])`
with `Vec::one_dim(x)`, `Vec::two_dim(a,b)`, `Vec::three_dim(a,b,c)`
and `Vec::four_dim(a,b,c,d)`. I think it's more intuitive.

For example:

```
let a: Vec<Vec<i64>> = Vec::two_dim(x, y)
```

It clearly says, initialize 2D vector. Oke, Nice!

# Developer Experience first
I think we already got the basics, we need to follow numpy docs
first before continue adding new feature.

Read the Numpy docs -> Modify the docs for `np` -> Write
and example.