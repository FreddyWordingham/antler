import glob
import math
import os
import sys


def stitch(input_dir, kind):
    """
    Stich together the individual render tiles at a given patturn.
    """

    pattern = os.path.join(input_dir, kind)

    tiles = glob.glob(f"{pattern}_*")
    width = 0
    height = 0
    for name in tiles:
        parts = name.split("_")
        xi = int(parts[-2])
        yi = int(parts[-1].split(".")[0])
        if xi > width:
            width = xi
        if yi > height:
            height = yi

    print_width = int(math.log10(max(width, height))) + 1
    for n in range(width + 1):
        slice_patturn = f"{pattern}_{n:0{print_width}}_*"
        os.system(
            f"convert -append {slice_patturn} {pattern}-slice-{n:0{print_width}}.png"
        )

    os.system(f"convert +append {pattern}-slice-* {os.path.join(input_dir, kind)}.png")
    # os.system(f"open {pattern}.png")


if __name__ == "__main__":
    input_dir = sys.argv[1]
    stitch(input_dir, "colour")
