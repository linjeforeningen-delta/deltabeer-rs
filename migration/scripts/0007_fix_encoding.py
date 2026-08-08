# /// script
# requires-python = ">=3.15"
# dependencies = [
#     "polars>=1.43.2",
# ]
# ///


from pathlib import Path

import polars as pl

CSV_PATH = Path("../csv")
SOURCE = CSV_PATH / "0006_birthdays_fixed.csv"
SINK = CSV_PATH / "0007_encoding_fixed.csv"

df = pl.read_csv(
    SOURCE,
    encoding="utf8-lossy",
)


df.write_csv(SINK)
