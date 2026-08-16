# /// script
# requires-python = ">=3.15"
# dependencies = [
#   "polars>=1.43.2",
# ]
# ///

from pathlib import Path

import polars as pl

CSV_PATH = Path("../csv")
SOURCE = CSV_PATH / "0011_encodings_fixed.csv"
SINK = CSV_PATH / "0012_final.csv"

df = pl.read_csv(
    SOURCE,
    encoding="utf8-lossy",
)

df = df.with_columns(
    pl.col("name").str.strip_chars(),
    pl.col("username").str.strip_chars(),
    pl.col("program").fill_null("").str.strip_chars(),
    pl.lit("").alias("comments"),
    pl.col("balance").cast(pl.Int64),
    pl.col("spent").cast(pl.Int64),
    pl.col("created_at").cast(pl.Int64),
).select(
    "id",
    "name",
    "username",
    "program",
    "card_number",
    "birthdate",
    "comments",
    "balance",
    "spent",
    "created_at",
)

df.write_csv(SINK)
