import sys
from pathlib import Path

from typeguard import typechecked


@typechecked
def print_files(root: Path) -> None:
    """
    Print the source code of Rust files under a root directory.

    - Recursively discover *.rs files beneath root.
    - Sort paths to ensure stable output across runs/platforms.
    - Print each file path exactly once.

    Args:
        root (Path): Root directory to search for Rust files.

    """
    for path in sorted(root.rglob("*.rs")):
        if path.is_file():
            print(path.as_posix())
            print("```rust")
            try:
                print(path.read_text(encoding="utf-8"))
            except Exception as err:
                print(f"# [Error reading file: {err}]")
            print("```\n")


if __name__ == "__main__":
    if len(sys.argv) != 2:
        print(f"Usage: {sys.argv[0]} <directory>")
        sys.exit(1)
    root = Path(sys.argv[1])
    if not root.is_dir():
        print(f"Error: {root} is not a directory.")
        sys.exit(1)
    print_files(root)
