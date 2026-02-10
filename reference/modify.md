# Modify a Toml object

`remove_items()` removes one or more items from the TOML document.
Alternatively, `insert_items()` inserts key value pairs into the TOML
document.

## Usage

``` r
remove_items(x, keys)

insert_items(x, ..., df_as_array = TRUE)

get_item(x, key)
```

## Arguments

- x:

  an object of class `Toml`.

- keys:

  a character vector of key names to remove. Cannot contain missing
  values.

- ...:

  named items to be serialized to TOML.

- df_as_array:

  default `TRUE`. Creates an array of tables from a `data.frame`. When
  `FALSE`, creates a single table with an array for each column in the
  data.frame.

- key:

  a character vector of key values. The keys are used recursively. For
  example with `key = c("a", "b")` the item `a` is grabbed first, then
  `b` is searched for inside of `a`.

## Value

an object of class `Toml`

## Examples

``` r
x <- toml(
  date = list(
    full = as.Date("2025-02-07"),
    parts = list(year = 2015L, month = "February", day = 7L)
  ),
  season = "winter"
)

# fetch the date table
get_item(x, "date")
#> $full
#> [1] "2025-02-07"
#> 
#> $parts
#> $parts$year
#> [1] 2015
#> 
#> $parts$month
#> [1] "February"
#> 
#> $parts$day
#> [1] 7
#> 
#> 

# fetch the month value
get_item(x, c("date", "parts", "month"))
#> [1] "February"

# remove an item based on name
remove_items(x, "season")
#> <Toml>
#> [date]
#> full = 2025-02-07
#> parts = { year = 2015, month = "February", day = 7 }

# add multiple items
insert_items(x, temperature = 31, cloudy = TRUE)
#> <Toml>
#> season = "winter"
#> temperature = 31.0
#> cloudy = true
#> 
#> [date]
#> full = 2025-02-07
#> parts = { year = 2015, month = "February", day = 7 }
```
