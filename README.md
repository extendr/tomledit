

# tomledit

<!-- badges: start -->
[![R-CMD-check](https://github.com/extendr/tomledit/actions/workflows/R-CMD-check.yaml/badge.svg)](https://github.com/extendr/tomledit/actions/workflows/R-CMD-check.yaml)
[![extendr](https://img.shields.io/badge/extendr-%5E0.7.1-276DC2)](https://extendr.github.io/extendr/extendr_api/)
<!-- badges: end -->

Create or edit TOML documents from R using `tomledit`.

`tomledit` is written in Rust using
[extendr](https://extendr.github.io/) and the
[`toml_edit`](https://docs.rs/toml_edit/) crate.

## Installation

Install the package from CRAN using

``` r
install.packages("tomledit")
```

or, install the development version using

``` r
remotes::install_github("extendr/tomledit")
```

## Usage

TOML can be created using either the `as_toml()` or `toml()` functions.

Use `as_toml()` to convert a list to TOML:

``` r
library(tomledit)

as_toml(
  list(
    person = list(age = 30L, name = "Wilma")
  )
)
```

    <Toml>
    [person]
    age = 30
    name = "Wilma"

Create TOML directly by passing key values to `toml()`:

``` r
x <- toml(person = list(age = 30L, name = "Wilma"))
x
```

    <Toml>
    [person]
    age = 30
    name = "Wilma"

Or, parse a string as TOML while preserving comments:

``` r
raw_toml <- '# Top-level table begins.
name = "Fido"
breed = "pug"

# Top-level table ends.
[owner]
name = "Regina Dogman"
member_since = 1999-08-04'

x <- parse_toml(raw_toml)
x
```

    <Toml>
    # Top-level table begins.
    name = "Fido"
    breed = "pug"

    # Top-level table ends.
    [owner]
    name = "Regina Dogman"
    member_since = 1999-08-04

Write a `Toml` object to a file using `write_toml()`.

``` r
tmp <- tempfile(fileext = ".toml")

write_toml(x, tmp)
```

Read a TOML file using `read_toml()`.

``` r
read_toml(tmp)
```

    <Toml>
    # Top-level table begins.
    name = "Fido"
    breed = "pug"

    # Top-level table ends.
    [owner]
    name = "Regina Dogman"
    member_since = 1999-08-04

Items can be inserted into a `Toml` document using `insert_items()`

``` r
y <- x |> 
  insert_items(
    date = Sys.Date(),
    date_parts = list(year = 2015L, month = "February", day = 7L)
  )

y
```

    <Toml>
    # Top-level table begins.
    name = "Fido"
    breed = "pug"
    date = 2025-03-03

    # Top-level table ends.
    [owner]
    name = "Regina Dogman"
    member_since = 1999-08-04

    [date_parts]
    year = 2015
    month = "February"
    day = 7

Or items can be removed as well using `remove_items()`

``` r
remove_items(y, c("date", "date_parts"))
```

    <Toml>
    # Top-level table begins.
    name = "Fido"
    breed = "pug"

    # Top-level table ends.
    [owner]
    name = "Regina Dogman"
    member_since = 1999-08-04

Individual items can be fetched recursively from the `Toml` document.

``` r
get_item(y, c("date_parts", "month"))
```

    [1] "February"

Or the entire `Toml` document can be converted to a list. Note, though,
that it is not always possible to perform a perfect round trip of R
objects and TOML.

``` r
from_toml(y)
```

    $name
    [1] "Fido"

    $breed
    [1] "pug"

    $owner
    $owner$name
    [1] "Regina Dogman"

    $owner$member_since
    [1] "1999-08-04"


    $date
    [1] "2025-03-03"

    $date_parts
    $date_parts$year
    [1] 2015

    $date_parts$month
    [1] "February"

    $date_parts$day
    [1] 7

## Array of Tables

By default `tomledit` converts `data.frame` objects to an [array of
tables](https://toml.io/en/v1.0.0#array-of-tables).

``` r
toml(iris = iris[1:3,])
```

    <Toml>
    [[iris]]
    "Sepal.Length" = 5.1
    "Sepal.Width" = 3.5
    "Petal.Length" = 1.4
    "Petal.Width" = 0.2
    Species = "setosa"

    [[iris]]
    "Sepal.Length" = 4.9
    "Sepal.Width" = 3.0
    "Petal.Length" = 1.4
    "Petal.Width" = 0.2
    Species = "setosa"

    [[iris]]
    "Sepal.Length" = 4.7
    "Sepal.Width" = 3.2
    "Petal.Length" = 1.3
    "Petal.Width" = 0.2
    Species = "setosa"

This is the default behavior as it is most consistent with TOML files
that are encountered in the wild. To create a single table from a
`data.frame`, set the argument `df_as_array = FALSE`.

``` r
toml(
  iris = iris[1:3,],
  df_as_array = FALSE
)
```

    <Toml>
    [iris]
    "Sepal.Length" = [5.1, 4.9, 4.7]
    "Sepal.Width" = [3.5, 3.0, 3.2]
    "Petal.Length" = [1.4, 1.4, 1.3]
    "Petal.Width" = [0.2, 0.2, 0.2]
    Species = ["setosa", "setosa", "setosa"]

## Missing Values

One reason why array of tables are recommended for `data.frame`s is
because there is no concept of a missing or null value in TOML.

Take the following example:

``` r
x <- data.frame(
  x = c(1L, NA, 2L),
  y = letters[1:3]
) 
```

Notice that when this `data.frame` is serialized to TOML the missing `x`
value is omitted:

``` r
toml(table = x)
```

    <Toml>
    [[table]]
    x = 1
    y = "a"

    [[table]]
    y = "b"

    [[table]]
    x = 2
    y = "c"

Whereas when serializing to a single table the `x` array has 2 elements
whereas the `y` element has 3 elements.

``` r
toml(table = x, df_as_array = FALSE)
```

    <Toml>
    [table]
    x = [1, 2]
    y = ["a", "b", "c"]
