#' Modify a Toml object
#'
#' `remove_items()` removes one or more items from the TOML document.
#' Alternatively, `insert_items()` inserts key value pairs into the TOML
#' document.
#'
#' @param x an object of class `Toml`.
#' @param keys a character vector of key names to remove. Cannot contain missing values.
#' @inheritParams toml
#' @rdname modify
#' @examples
#' x <- toml(
#'   date = list(
#'     full = as.Date("2025-02-07"),
#'     parts = list(year = 2015L, month = "February", day = 7L)
#'   ),
#'   season = "winter"
#' )
#'
#' # fetch the date table
#' get_item(x, "date")
#'
#' # fetch the month value
#' get_item(x, c("date", "parts", "month"))
#'
#' # remove an item based on name
#' remove_items(x, "season")
#'
#' # add multiple items
#' insert_items(x, temperature = 31, cloudy = TRUE)
#' @export
#' @returns an object of class `Toml`
remove_items <- function(x, keys) {
  check_character(keys, allow_na = FALSE, allow_null = FALSE)
  for (key in keys) {
    x <- .catch(x$remove_item(key))
  }
  x
}

#' @rdname modify
#' @export
insert_items <- function(x, ..., df_as_array = TRUE) {
  dots <- rlang::list2(...)
  # check_list_named(dots)
  .catch(x$insert_list(dots, df_as_array))
}

#' @param key a character vector of key values. The keys are used recursively. For example with `key = c("a", "b")` the item `a` is grabbed first, then `b` is searched for inside of `a`.
#' @rdname modify
#' @export
get_item <- function(x, key) {
  check_character(key, allow_na = FALSE, allow_null = FALSE)
  .catch(x$get_item(key))
}
