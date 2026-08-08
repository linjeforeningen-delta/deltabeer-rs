# /// script
# requires-python = ">=3.15"
# dependencies = [
#   "polars>=1.43.2",
# ]
# ///

from pathlib import Path

import polars as pl

CSV_PATH = Path("../csv")
SOURCE = CSV_PATH / "0009_uuid_added.csv"
SINK = CSV_PATH / "0010_columns_renamed.csv"

df = pl.read_csv(
    SOURCE,
    encoding="utf8-lossy",
)

df = df.rename(
    {
        "Name": "name",
        "Username": "username",
        "Program": "program",
        "Card ID": "card_number",
        "Birthday": "birthdate",
        "Cash": "balance",
        "Spent": "spent",
        "Creation date": "created_at",
    }
)

df.write_csv(SINK)
