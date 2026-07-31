# tomledit (development)

* extend capabilities of writing and printing to support formatting.

* Unnamed lists are now written as arrays wherever they appear. Previously an
  unnamed list passed as the value of a key was written as a table with a single
  `NA` key, keeping only the last element (#5).

* A list of named lists is now written as an array of tables, so an item read
  with `get_item()` can be inserted back into a document with `insert_items()`
  (#5).

# tomledit 0.1.1 

* Unnamed lists are now turned into an array of items. Any named listed are treated as inline tables. 

# tomledit 0.1.0

* Added a `NEWS.md` file to track changes to the package.
