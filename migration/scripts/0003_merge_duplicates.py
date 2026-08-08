# /// script
# requires-python = ">=3.15"
# dependencies = [
#     "polars>=1.43.2",
# ]
# ///


from pathlib import Path

import polars as pl

CSV_PATH = Path("../csv")
SOURCE = CSV_PATH / "0002_unbroken.csv"
SINK = CSV_PATH / "0003_deduped.csv"

df = pl.read_csv(
    SOURCE,
    encoding="utf8-lossy",
)

original_columns = df.columns

USERNAME_ALIASES = {
    "Miasoh": "Petersoe",
    "Arjap": "Arja",
}

df = df.with_columns(pl.col("Username").replace(USERNAME_ALIASES).alias("Username"))


df = df.sort("Creation date")

canonical_usernames = df.group_by(
    ["First name", "Last name"],
    maintain_order=True,
).agg(pl.col("Username").last().alias("_canonical_username"))

df = (
    df.join(
        canonical_usernames,
        on=["First name", "Last name"],
        how="left",
    )
    .with_columns(pl.col("_canonical_username").alias("Username"))
    .drop("_canonical_username")
)

merged = df.group_by(
    "Username",
    maintain_order=True,
).agg(
    [
        # Newest/current account information
        pl.col("Card ID").last(ignore_nulls=True),
        pl.col("Last name").last(ignore_nulls=True),
        pl.col("First name").last(ignore_nulls=True),
        pl.col("Birthday").last(ignore_nulls=True),
        pl.col("Program").last(ignore_nulls=True),
        pl.col("Membership").last(ignore_nulls=True),
        pl.col("Userlevel").last(ignore_nulls=True),
        pl.col("Password").last(ignore_nulls=True),
        pl.col("Tab").last(ignore_nulls=True),
        # Combine financial state/history
        pl.col("Cash").sum(),
        pl.col("Spent").sum(),
        # Newest miscellaneous values
        pl.col("Borrowed").last(ignore_nulls=True),
        pl.col("Comment").last(ignore_nulls=True),
        pl.col("Misc").last(ignore_nulls=True),
        # Preserve original first creation date
        pl.col("Creation date").min(),
    ]
)


# print(f"Before: {df.height}")
# print(f"After:  {merged.height}")
# print(f"Merged: {df.height - merged.height}")

assert merged["Username"].n_unique() == merged.height
assert merged.height <= df.height
assert merged["Username"].null_count() == 0

assert merged["Cash"].sum() == df["Cash"].sum()
assert merged["Spent"].sum() == df["Spent"].sum()

merged = merged.select(original_columns)

merged.write_csv(SINK)
