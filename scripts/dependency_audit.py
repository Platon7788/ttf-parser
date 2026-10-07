"""Compare all resolved registry crates with current non-yanked stable releases."""
import concurrent.futures
import json
from pathlib import Path
import tomllib
import urllib.request

ROOT = Path(__file__).resolve().parents[1]
MANIFESTS = ["Cargo.toml", "c-api/Cargo.toml", "benches/Cargo.toml",
             "testing-tools/ttf-fuzz/Cargo.toml", "testing-tools/downstream-smoke/Cargo.toml"]


def latest(entry):
    name, current = entry
    req = urllib.request.Request("https://crates.io/api/v1/crates/" + name,
                                 headers={"User-Agent": "DataForge-dependency-audit/1.0"})
    with urllib.request.urlopen(req, timeout=30) as response:
        data = json.load(response)
    stable = next(v for v in data["versions"] if not v["yanked"] and "-" not in v["num"])
    return {"name": name, "resolved": sorted(current), "latest_stable": stable["num"],
            "rust_version": stable.get("rust_version"), "matches_latest": current == {stable["num"]}}


def main():
    packages = {}
    for manifest in MANIFESTS:
        lock = ROOT / Path(manifest).with_name("Cargo.lock")
        for package in tomllib.loads(lock.read_text(encoding="utf-8"))["package"]:
            if package.get("source", "").startswith("registry+"):
                packages.setdefault(package["name"], set()).add(package["version"])
    with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool:
        results = list(pool.map(latest, sorted(packages.items())))
    from datetime import datetime, timezone
    report = {"checked_at": datetime.now(timezone.utc).isoformat(), "packages": results}
    (ROOT / "dependency-audit.json").write_text(json.dumps(report, indent=2), encoding="utf-8")
    for row in results:
        print(row["name"], ",".join(row["resolved"]), "latest=" + row["latest_stable"],
              "OK" if row["matches_latest"] else "REVIEW")
    return 0 if all(row["matches_latest"] for row in results) else 1


if __name__ == "__main__":
    raise SystemExit(main())
