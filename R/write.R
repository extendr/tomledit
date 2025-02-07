#' Generate TOML
#'
#' Write a `Toml` object to a file or to a string. Use `write_toml()` to
#' write to a file on disk. Or, use `to_toml()` to create a string
#' containing `TOML`.
#'
#' @inheritParams insert_items
#' @param file path to the file to  write.
#' @export
#' @rdname write
#' @returns `write_toml()` returns a `Toml` object invisibly. `to_toml()` returns a string.
#' @examples
#' tmp <- tempfile(fileext = ".toml")
#'
#' x <- toml(
#'   today = Sys.Date(),
#'   human = list(person = "Greg", age = 29, bday = "1969-07-02"),
#' )
#'
#' write_toml(x, tmp)
#' read_toml(tmp)
#' to_toml(x)
write_toml <- function(x, file) {
  .catch(x$write(file))
  invisible(x)
}


#' @export
#' @rdname write
to_toml <- function(x) {
  .catch(x$format())
}
