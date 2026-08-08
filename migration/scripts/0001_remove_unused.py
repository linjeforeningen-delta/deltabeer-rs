# /// script
# requires-python = ">=3.15"
# dependencies = [
#     "polars>=1.43.2",
# ]
# ///


from pathlib import Path

import polars as pl

CSV_PATH = Path("../csv")
SOURCE = CSV_PATH / "0000_original.csv"
SINK = CSV_PATH / "0001_used.csv"

df = pl.read_csv(
    SOURCE,
    separator=";",
    encoding="utf8-lossy",
)


used_users = df.filter((pl.col("Cash") != 0) | (pl.col("Spent") != 0))


used_users.write_csv(SINK)
