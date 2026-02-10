# Read and parse TOML

Use `parse_toml()` to parse a string into a `Toml` document. Use
`read_toml()` to read a `.toml` file from disk.

## Usage

``` r
parse_toml(x)

read_toml(file)
```

## Arguments

- x:

  a character scalar containing valid TOML

- file:

  path to the file to write.

## Value

an object of class `Toml`

## Examples

``` r
# TOML string
raw_toml <- '# Top-level table begins.
name = "Fido"
breed = "pug"

# Top-level table ends.
[owner]
name = "Regina Dogman"
member_since = 1999-08-04'

# write the TOML string to a temp file
tmp <- tempfile()
writeLines(raw_toml, tmp)

# parse the TOML string
parse_toml(raw_toml)
#> <Toml>
#> # Top-level table begins.
#> name = "Fido"
#> breed = "pug"
#> 
#> # Top-level table ends.
#> [owner]
#> name = "Regina Dogman"
#> member_since = 1999-08-04

# read the TOML file
read_toml(tmp)
#> <Toml>
#> # Top-level table begins.
#> name = "Fido"
#> breed = "pug"
#> 
#> # Top-level table ends.
#> [owner]
#> name = "Regina Dogman"
#> member_since = 1999-08-04
```
