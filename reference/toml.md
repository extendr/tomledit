# Create Toml objects

Use `as_toml()` to convert a named list to a `Toml` object. Or, create a
`Toml` object by passing in named values to `toml()`.

## Usage

``` r
as_toml(x, df_as_array = TRUE)

toml(..., df_as_array = TRUE)
```

## Arguments

- x:

  a named list

- df_as_array:

  default `TRUE`. Creates an array of tables from a `data.frame`. When
  `FALSE`, creates a single table with an array for each column in the
  data.frame.

- ...:

  named items to be serialized to TOML.

## Value

an object of class `Toml`

## Details

If you are serializing a `data.frame` to a single table with
`df_as_array = FALSE`, note that **missing values are omitted** when
serializing a vector to an array as there is no concept of missing
values in TOML.

## Examples

``` r
toml(person = list(age = 30L, name = "Wilma"))
#> <Toml>
#> [person]
#> age = 30
#> name = "Wilma"

as_toml(
  list(
    person = list(age = 30L, name = "Wilma")
  )
)
#> <Toml>
#> [person]
#> age = 30
#> name = "Wilma"
```
