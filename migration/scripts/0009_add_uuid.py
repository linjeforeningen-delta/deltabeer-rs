# /// script
# requires-python = ">=3.15"
# dependencies = [
#   "polars>=1.43.2",
# ]
# ///

import secrets
import uuid
from pathlib import Path

import polars as pl

CSV_PATH = Path("../csv")
SOURCE = CSV_PATH / "0008_names_merged.csv"
SINK = CSV_PATH / "0009_uuid_added.csv"


def uuid7_from_unix(timestamp: int) -> uuid.UUID:
    """Create a UUIDv7 using a Unix timestamp in seconds."""

    timestamp_ms = timestamp * 1000

    if not 0 <= timestamp_ms < (1 << 48):
        raise ValueError("Timestamp out of UUIDv7 range")

    rand_a = secrets.randbits(12)
    rand_b = secrets.randbits(62)

    value = (timestamp_ms << 80) | (0x7 << 76) | (rand_a << 64) | (0b10 << 62) | rand_b

    return uuid.UUID(int=value)


df = pl.read_csv(
    SOURCE,
    encoding="utf8-lossy",
)

df = df.with_columns(
    pl.col("Creation date")
    .map_elements(
        lambda ts: str(uuid7_from_unix(ts)),
        return_dtype=pl.String,
    )
    .alias("id"),
)

df.write_csv(SINK)
