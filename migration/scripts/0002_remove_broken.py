# /// script
# requires-python = ">=3.15"
# dependencies = [
#     "polars>=1.43.2",
# ]
# ///


from pathlib import Path

import polars as pl

CSV_PATH = Path("../csv")
SOURCE = CSV_PATH / "0001_used.csv"
SINK = CSV_PATH / "0002_unbroken.csv"

df = pl.read_csv(
    SOURCE,
    encoding="utf8-lossy",
)


unbroken_users = df.filter(~pl.col("Username").str.contains(r"^x+$"))


unbroken_users.write_csv(SINK)
