raw_toml <- r"-([content]
id = "123e4567-e89b-12d3-a456-426614174000"
name = "My Analysis Report"
entrypoint = "script.R"
access_type = "private"
content_type = "quarto-r"
summary = "A detailed report on financial trends."
thumbnail = "thumbnails/report.png"
tags = ["finance", "data-analysis", "R"]
include = ["data/*.csv", "config.yml"]

[language]
name = "r"
version = "4.2.0"
packages = "renv.lock"

# schedule comment
[schedule]
enabled = true
cron = "0 0 * * 1"
parameterized = true

[serve]
min_instances = 1
max_instances = 5
spawn_threshold = 80
max_connections = 10
max_connection_age = 3600
inactive_timeout = 300
connection_timeout = 30

[static]
output_dir = "reports"
render_fn = "bookdown::render_book"
)-"

toml <- tomledit::Toml$parse_toml(raw_toml)

toml$print()

wrap__Toml__parse_toml

toml$update_key("schedule", list(enabled = FALSE, cron = NULL, parameterized = FALSE))

toml$insert_item(
  "schedule",
)





toml$insert_table(
  "schedule",
  list(enabled = FALSE, cron = NULL, parameterized = FALSE)
)
toml$print()
