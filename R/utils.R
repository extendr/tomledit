#' @export
print.Toml <- function(x, ...) {
  cat("<Toml>\n")
  cat(x$format())
  invisible(x)
}

#' @export
as.character.Toml <- function(x, ...) {
  .catch(x$format_lines())
}


# Catch an error condition returned by extendr
.catch <- function(cnd) {
  catch_cnd(
    {
      if (is_condition(cnd)) {
        cnd[["message"]] <- cnd[["value"]]
        cnd_signal(cnd)
      }
      cnd
    },
    "extendr_err"
  )
  cnd
}

# Ensure a list contains only named elements
check_list_named <- function(dots, call = rlang::caller_env()) {
  if (!rlang::is_named2(dots)) {
    rlang::abort(
      "All arguments provided to {.arg ...} must be named",
      call = call
    )
  }
  invisible(dots)
}
