# /// script
# requires-python = ">=3.15"
# dependencies = [
#     "polars>=1.43.2",
# ]
# ///

import shutil
import subprocess
import sys
from pathlib import Path

BASE_DIR = Path(__file__).resolve().parent
SCRIPTS_DIR = BASE_DIR / "scripts"
CSV_DIR = BASE_DIR / "csv"

CALL_DIR = Path.cwd()

scripts = sorted(SCRIPTS_DIR.glob("*.py"))

for script in scripts:
    print(f"Running {script.name}...")
    subprocess.run(
        [sys.executable, script.name],
        cwd=script.parent,
        check=True,
    )

final_csv = CSV_DIR / "0011_final.csv"
destination = CALL_DIR / "users.csv"

shutil.copy2(final_csv, destination)

print(f"Preparation complete.")
print(f"Copied {final_csv.name} to {destination}")
