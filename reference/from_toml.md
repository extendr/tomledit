# Convert `Toml` to an R object

Use `from_toml()` to convert a `Toml` document to an R object. Note that
that due to the encoding of values in the TOML specification a perfect
round trip from TOML to R is not always possible.

## Usage

``` r
from_toml(x)
```

## Arguments

- x:

  a `Toml` object.

## Value

a list

## Examples

``` r
from_toml(toml(hello = "world"))
#> $hello
#> [1] "world"
#> 
```
