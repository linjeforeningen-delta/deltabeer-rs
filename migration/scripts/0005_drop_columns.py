# /// script
# requires-python = ">=3.15"
# dependencies = [
#     "polars>=1.43.2",
# ]
# ///


from pathlib import Path

import polars as pl

CSV_PATH = Path("../csv")
SOURCE = CSV_PATH / "0004_forgiven.csv"
SINK = CSV_PATH / "0005_columns_dropped.csv"

df = pl.read_csv(
    SOURCE,
    encoding="utf8-lossy",
)

df = df.drop(
    ["Membership", "Userlevel", "Password", "Tab", "Borrowed", "Comment", "Misc"]
)

df.write_csv(SINK)
