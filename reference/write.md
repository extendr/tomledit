# Generate TOML

Write a `Toml` object to a file or to a string. Use `write_toml()` to
write to a file on disk. Or, use `to_toml()` to create a string
containing `TOML`.

## Usage

``` r
write_toml(x, file, format = TRUE)

to_toml(x, format = TRUE)
```

## Arguments

- x:

  an object of class `Toml`.

- file:

  path to the file to write.

- format:

  whether to format the TOML output.

## Value

`write_toml()` returns a `Toml` object invisibly. `to_toml()` returns a
string.

## Examples

``` r
tmp <- tempfile(fileext = ".toml")

x <- toml(
  today = Sys.Date(),
  human = list(person = "Greg", age = 29, bday = "1969-07-02"),
)

write_toml(x, tmp)
read_toml(tmp)
#> <Toml>
#> today = 2026-02-10
#> 
#> [human]
#> person = "Greg"
#> age = 29.0
#> bday = "1969-07-02"
to_toml(x)
#> [1] "today = 2026-02-10\n\n[human]\nperson = \"Greg\"\nage = 29.0\nbday = \"1969-07-02\"\n"
```
