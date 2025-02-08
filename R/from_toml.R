#' Convert `Toml` to an R object
#'
#' Use `from_toml()` to convert a `Toml` document to an R object.
#' Note that that due to the encoding of values in the TOML specification
#' a perfect round trip from TOML to R is not always possible.
#'
#' @param x a `Toml` object.
#' @returns a list
#' @examples
#' from_toml(toml(hello = "world"))
#' @export
from_toml <- function(x) {
  .catch(x$from_toml())
}
