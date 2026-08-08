# /// script
# requires-python = ">=3.15"
# dependencies = [
#   "polars>=1.43.2",
# ]
# ///

from pathlib import Path

import polars as pl

CSV_PATH = Path("../csv")
SOURCE = CSV_PATH / "0007_encoding_fixed.csv"
SINK = CSV_PATH / "0008_names_merged.csv"

df = pl.read_csv(
    SOURCE,
    encoding="utf8-lossy",
)

df = df.with_columns(
    pl.concat_str(
        [
            pl.col("First name").str.strip_chars(),
            pl.col("Last name").str.strip_chars(),
        ],
        separator=" ",
    ).alias("Name"),
).drop(
    "First name",
    "Last name",
)

df.write_csv(SINK)
