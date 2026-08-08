# /// script
# requires-python = ">=3.15"
# dependencies = [
#     "polars>=1.43.2",
# ]
# ///


from datetime import date, datetime
from pathlib import Path

import polars as pl

CSV_PATH = Path("../csv")
SOURCE = CSV_PATH / "0005_columns_dropped.csv"
SINK = CSV_PATH / "0006_birthdays_fixed.csv"

df = pl.read_csv(
    SOURCE,
    encoding="utf8-lossy",
)

new_birthdays = {
    "olemartinedstr?m": "090598",
    "antonywm": "130799",
    "sigmunda": "160389",
}

df = df.with_columns(
    pl.col("Username")
    .replace_strict(new_birthdays, default=pl.col("Birthday"))
    .alias("Birthday")
)

df = df.filter(pl.col("Birthday").is_not_null() & (pl.col("Birthday") != "unknown"))


def parse_birthday(value: str) -> str:
    """
    Convert legacy birthdays to YYYY-MM-DD.

    Accepted:
        060189      -> 1989-01-06
        170306      -> 2006-03-17
        1970-01-01  -> 1970-01-01

    Raises ValueError for invalid values.
    """
    value = value.strip()

    # Already in the new format
    try:
        parsed = datetime.strptime(value, "%Y-%m-%d").date()
        return parsed.isoformat()
    except ValueError:
        pass

    # Legacy format must be DDMMYY
    if len(value) != 6 or not value.isdigit():
        raise ValueError(f"Invalid birthday: {value!r}")

    day = int(value[0:2])
    month = int(value[2:4])
    short_year = int(value[4:6])

    current_short_year = date.today().year % 100

    # Birth years later than the current 2-digit year are assumed
    # to belong to the previous century.
    #
    # In 2026:
    #   89 -> 1989
    #   97 -> 1997
    #   00 -> 2000
    #   06 -> 2006
    #   25 -> 2025
    #   27 -> 1927
    if short_year <= current_short_year:
        year = 2000 + short_year
    else:
        year = 1900 + short_year

    try:
        parsed = date(year, month, day)
    except ValueError as e:
        raise ValueError(f"Invalid birthday: {value!r}") from e

    return parsed.isoformat()


df = df.with_columns(
    pl.col("Birthday")
    .map_elements(parse_birthday, return_dtype=pl.String)
    .alias("Birthday")
)


df.write_csv(SINK)
