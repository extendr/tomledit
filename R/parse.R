#' Read and parse TOML
#'
#' Use `parse_toml()` to parse a string into a `Toml` document.
#' Use `read_toml()` to read a `.toml` file from disk.
#'
#' @param x a character scalar containing valid TOML
#' @inheritParams write_toml
#' @export
#' @rdname read
#' @returns an object of class `Toml`
#' @examples
#' # TOML string
#' raw_toml <- '# Top-level table begins.
#' name = "Fido"
#' breed = "pug"
#'
#' # Top-level table ends.
#' [owner]
#' name = "Regina Dogman"
#' member_since = 1999-08-04'
#'
#' # write the TOML string to a temp file
#' tmp <- tempfile()
#' writeLines(raw_toml, tmp)
#'
#' # parse the TOML string
#' parse_toml(raw_toml)
#'
#' # read the TOML file
#' read_toml(tmp)
parse_toml <- function(x) {
  check_string(x, allow_empty = FALSE)
  .catch(Toml$parse_toml(x))
}

#' @export
#' @rdname read
read_toml <- function(file) {
  check_string(file, allow_empty = FALSE)
  .catch(Toml$read(file))
}
