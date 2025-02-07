x <- Toml$new()

x$insert_array_of_tables(
  "iris",
  iris[1:2, ]
)

x$insert_table("iris-table", iris[1:2, ])

x$print()

x$insert_item(
  "table-name",
  list(
    id = "123e4567-e89b-12d3-a456-426614174000",
    summary = "Filling out a detailed summary field",
    include = c("data/*.csv", "config.yml")
  )
)

n <- 5
x$insert_item("starwars", dplyr::starwars[1:n, ])
x$insert_array_of_tables("starwars-array", dplyr::starwars[1:n, ])


tst <- r"-(
[testing]
values = [1, 2, "three", { four = true }, 2025-02-06, 2.5, false]
dates = [2025-02-06, 2025-01-11]
numeric = [1.0, 0]
ints = [1, 1]
)-"

Toml$parse_toml(tst)$get_key("testing")
Toml$parse_toml(tst)$get_key(c("testing", "dates"))
Toml$parse_toml(tst)$get_key(c("testing", "numeric"))
Toml$parse_toml(tst)$get_key(c("testing", "ints"))

x$insert_item("mixed", list(x = 1, ))
y <- x$print()


invisible(x$get_key("starwars-array"))

arr_toml <- Toml$new()
arr_toml$insert_array_of_tables("starwars-array", dplyr::starwars[1:n, ])
RcppTOML::parseTOML(arr_toml$print(), fromFile = FALSE)

bench::mark(
  rcpp = 
    RcppTOML::parseTOML(raw_toml, fromFile = FALSE),
  rust = tomledit::Toml$parse_toml(raw_toml)$to_robj(),
  check = FALSE
)


x <- Toml$new()

y <- x$insert_list(list(x = 1))
y

print.Toml <- function(x, ...) {
  cat("<Toml>\n")
  cat(format(x$format()))
  invisible(x)
}

x$insert_array_of_tables("Iris", iris[1:4,])
