import colorsys
import math
import random
from pathlib import Path


def rgb_hex(r: float, g: float, b: float) -> str:
    return f"0x{int(r * 255):02X}{int(g * 255):02X}{int(b * 255):02X}"


def complementary_gradient(rng: random.Random) -> tuple[str, str]:
    base_hue = rng.random()
    opposite_hue = (base_hue + 0.2) % 1.0  # 180° shift

    # Keep these tighter for nicer palettes
    saturation = rng.uniform(0.7, 1.0)
    value = rng.uniform(0.8, 1.0)

    colour_a = colorsys.hsv_to_rgb(base_hue, saturation, value)
    colour_b = colorsys.hsv_to_rgb(opposite_hue, saturation, value)

    return rgb_hex(*colour_a), rgb_hex(*colour_b)


def pyramid_object(
    *,
    x: float,
    y: float,
    scale: float,
    rotation: float,
    colour_a: str,
    colour_b: str,
    mirror: bool,
) -> str:
    return f"""        (
          geometry: Mesh ( path: "assets/meshes/pyramid.obj" ),
          material: {"Opaque" if not mirror else "Mirror"},
          shader: Gradient ( gradient: [{colour_a}, {colour_b}], power: 1.0 ),
          transform: (
            translation: ({x:.2f}, {y:.2f}, 0.0),
            rotation: (0.0, 0.0, {rotation:.1f}),
            scale: {scale:.2f}
          )
        )"""


def generate_pyramids(
    *,
    count: int = 72,
    base_radius: float = 40.0,
    radius_variation: float = 15.0,
    min_scale: float = 2.0,
    max_scale: float = 7.0,
    seed: int = 32,
    mirror_chance: float = 0.1,
) -> str:
    rng = random.Random(seed)
    objects: list[str] = []

    for i in range(count):
        angle = 2.0 * math.pi * i / count

        t = rng.random()
        scale = min_scale + t * (max_scale - min_scale)
        radius = base_radius + (t - 0.5) * 2.0 * radius_variation

        x = radius * math.cos(angle)
        y = radius * math.sin(angle)

        colour_a, colour_b = complementary_gradient(rng)

        objects.append(
            pyramid_object(
                x=x,
                y=y,
                scale=scale,
                rotation=rng.uniform(0.0, 360.0),
                colour_a=colour_a,
                colour_b=colour_b,
                mirror=rng.random() < mirror_chance,
            )
        )

    return ",\n".join(objects)


def main() -> None:
    output_path = Path("pyramids.ron")
    output_path.write_text(generate_pyramids(), encoding="utf-8")
    print(f"Wrote {output_path}")


if __name__ == "__main__":
    main()
