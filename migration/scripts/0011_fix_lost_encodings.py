# /// script
# requires-python = ">=3.15"
# dependencies = [
#   "polars>=1.43.2",
# ]
# ///
import re
from pathlib import Path

import polars as pl

CSV_PATH = Path("../csv")
SOURCE = CSV_PATH / "0010_columns_renamed.csv"
SINK = CSV_PATH / "0011_encodings_fixed.csv"

df = pl.read_csv(
    SOURCE,
    encoding="utf8-lossy",
)
CORRECTIONS = {
    "?": "?",
    "?KYB": "?KYB",
    "?L": "?L",
    "?MAT": "?MAT",
    "?ien": "?ien",
    "?istein": "?istein",
    "?lli?tt": "?lli?tt",
    "?shamar": "?shamar",
    "?smund": "?smund",
    "?sne": "?sne",
    "?stby": "?stby",
    "?verdal": "?verdal",
    "?verlier": "?verlier",
    "?yvind": "?yvind",
    "Aamb?": "Aamb?",
    "Andr?": "Andr?",
    "B?rd": "B?rd",
    "B?rve": "B?rve",
    "BS?K": "BS?K",
    "Bj?rke": "Bj?rke",
    "Bj?rn?degaard": "Bj?rn?degaard",
    "Bj?rnar": "Bj?rnar",
    "Bjellv?g": "Bjellv?g",
    "Bod?": "Bod?",
    "Bolsg?rd": "Bolsg?rd",
    "D?nvold": "D?nvold",
    "Edstr?m": "Edstr?m",
    "Elkj?r": "Elkj?r",
    "Enges?t": "Enges?t",
    "F?r?y": "F?r?y",
    "F?rde": "F?rde",
    "Fidjest?l": "Fidjest?l",
    "G?testam": "G?testam",
    "Gj?lmesli": "Gj?lmesli",
    "H?g?sen": "H?g?sen",
    "H?gbo": "H?gbo",
    "H?gseth": "H?gseth",
    "H?kon": "H?kon",
    "H?konland": "H?konland",
    "H?vard": "H?vard",
    "J?rgen": "J?rgen",
    "J?rgenB": "J?rgenB",
    "J?rnes": "J?rnes",
    "K?rstein": "K?rstein",
    "Konvergensm?te": "Konvergensm?te",
    "Kr?ger": "Kr?ger",
    "Kvams?e": "Kvams?e",
    "Kyrkjeb?": "Kyrkjeb?",
    "L?nvik": "L?nvik",
    "Ladeg?rd": "Ladeg?rd",
    "Manns?ker": "Manns?ker",
    "Marits?nn": "Marits?nn",
    "N?dtvedt": "N?dtvedt",
    "Nordg?rd": "Nordg?rd",
    "P?l": "P?l",
    "P?l-Anders": "P?l-Anders",
    "R?e": "R?e",
    "R?hneb?k": "R?hneb?k",
    "R?nning": "R?nning",
    "R?nningen": "R?nningen",
    "R?nseth": "R?nseth",
    "R?rvik": "R?rvik",
    "Rolfs?n": "Rolfs?n",
    "Rust?en": "Rust?en",
    "S?le": "S?le",
    "S?nderland": "S?nderland",
    "S?nsthagen": "S?nsthagen",
    "S?rbotten": "S?rbotten",
    "S?rensen": "S?rensen",
    "S?rli": "S?rli",
    "S?th": "S?th",
    "S?vik": "S?vik",
    "Sandsbr?ten": "Sandsbr?ten",
    "Sj?borg": "Sj?borg",
    "Sm?r?s": "Sm?r?s",
    "Sol?s": "Sol?s",
    "Synn?ve": "Synn?ve",
    "T?nseth": "T?nseth",
    "Tj?tta": "Tj?tta",
    "Torbj?rn": "Torbj?rn",
    "Trov?g": "Trov?g",
    "V?gslid": "V?gslid",
    "Val?s": "Val?s",
    "Vebj?rn": "Vebj?rn",
    "olemartinedstr?m": "olemartinedstr?m",
}


patterns = [
    (
        re.compile(rf"(?<!\w){re.escape(bad)}(?!\w)"),
        corrected,
    )
    for bad, corrected in CORRECTIONS.items()
]


def fix_text(value: str | None) -> str | None:
    if value is None:
        return None

    for pattern, corrected in patterns:
        value = pattern.sub(
            lambda _: corrected,
            value,
        )

    return value


df = df.with_columns(
    [
        pl.col(column)
        .map_elements(
            fix_text,
            return_dtype=pl.String,
        )
        .alias(column)
        if dtype == pl.String
        else pl.col(column)
        for column, dtype in zip(df.columns, df.dtypes)
    ]
)

df.write_csv(SINK)


# Find anything still containing "?"
remaining_words = set()
remaining_question_marks = 0

for column, dtype in zip(df.columns, df.dtypes):
    if dtype != pl.String:
        continue

    values = df.select(pl.col(column).drop_nulls()).to_series().to_list()

    for value in values:
        remaining_question_marks += value.count("?")

        for word in re.findall(r"\S*\?\S*", value):
            remaining_words.add(word)


print()
print(f"Remaining ?: {remaining_question_marks}")
print(f"Distinct words containing ?: {len(remaining_words)}")

if remaining_words:
    print()
    print("Remaining words:")
    for word in sorted(remaining_words):
        print(repr(word))
else:
    print("No words containing ? remain.")
