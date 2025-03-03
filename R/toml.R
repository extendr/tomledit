#' Create Toml objects
#'
#' Use `as_toml()` to convert a named list to a `Toml` object.
#' Or, create a `Toml` object by passing in named values to `toml()`.
#'
#' @param x a named list
#' @param ... named items to be serialized to TOML.
#' @param df_as_array default `TRUE`. Creates an array of tables from a `data.frame`.
#'  When `FALSE`, creates a single table with an array for each column in the data.frame.
#'
#' @details
#'
#' If you are serializing a `data.frame` to a single table with `df_as_array = FALSE`,
#' note that **missing values are omitted** when serializing a vector to an array as there is no
#' concept of missing values in TOML.
#'
#' @export
#' @returns an object of class `Toml`
#' @rdname toml
#' @examples
#' toml(person = list(age = 30L, name = "Wilma"))
#'
#' as_toml(
#'   list(
#'     person = list(age = 30L, name = "Wilma")
#'   )
#' )
as_toml <- function(x, df_as_array = TRUE) {
  # check_list_named(x)
  .catch(Toml$new()$insert_list(x, df_as_array))
}

#' @export
#' @rdname toml
toml <- function(..., df_as_array = TRUE) {
  dots <- rlang::list2(...)
  # check_list_named(dots)
  .catch(Toml$new()$insert_list(dots, df_as_array))
}
