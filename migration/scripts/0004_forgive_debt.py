# /// script
# requires-python = ">=3.15"
# dependencies = [
#     "polars>=1.43.2",
# ]
# ///


from pathlib import Path

import polars as pl

CSV_PATH = Path("../csv")
SOURCE = CSV_PATH / "0003_deduped.csv"
SINK = CSV_PATH / "0004_forgiven.csv"

df = pl.read_csv(
    SOURCE,
    encoding="utf8-lossy",
)

forgiven = df.filter(pl.col("Cash") < 0).select((-pl.col("Cash")).sum()).item()

df = df.with_columns(
    pl.when(pl.col("Cash") < 0).then(0).otherwise(pl.col("Cash")).alias("Cash")
)

# print(f"Debt forgiven: {forgiven}")

df.write_csv(SINK)
