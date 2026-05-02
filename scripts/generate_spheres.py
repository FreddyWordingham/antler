import colorsys
import math
import random
from pathlib import Path


def rgb_hex(r: float, g: float, b: float) -> str:
    return f"0x{int(r * 255):02X}{int(g * 255):02X}{int(b * 255):02X}"


def complementary_gradient(rng: random.Random) -> tuple[str, str]:
    base_hue = rng.random()
    opposite_hue = (base_hue + 0.2) % 1.0

    saturation = rng.uniform(0.7, 1.0)
    value = rng.uniform(0.8, 1.0)

    colour_a = colorsys.hsv_to_rgb(base_hue, saturation, value)
    colour_b = colorsys.hsv_to_rgb(opposite_hue, saturation, value)

    return rgb_hex(*colour_a), rgb_hex(*colour_b)


def sphere_object(
    *,
    x: float,
    y: float,
    z: float,
    scale: float,
    colour_a: str,
    colour_b: str,
    mirror: bool,
) -> str:
    return f"""        (
          geometry: Sphere ( ),
          material: {"Opaque" if not mirror else "Mirror"},
          shader: Gradient ( gradient: [{colour_a}, {colour_b}], power: 1.0 ),
          transform: (
            translation: ({x:.2f}, {y:.2f}, {z:.2f}),
            scale: {scale:.2f}
          )
        )"""


def generate_spheres(
    *,
    count: int = 72,
    base_radius: float = 60.0,
    radius_variation: float = 5.0,
    min_scale: float = 2.0,
    max_scale: float = 7.0,
    seed: int = 32,
    mirror_chance: float = 0.1,
) -> str:
    rng = random.Random(seed)
    objects: list[str] = []

    for _ in range(count):
        theta = rng.uniform(0.0, 2.0 * math.pi)
        z_unit = rng.uniform(0.0, 1.0)
        xy_radius = math.sqrt(1.0 - z_unit * z_unit)

        t = rng.random()
        scale = min_scale + t * (max_scale - min_scale)
        radius = base_radius + (t - 0.5) * 2.0 * radius_variation

        x = radius * xy_radius * math.cos(theta)
        y = radius * xy_radius * math.sin(theta)
        z = radius * z_unit

        colour_a, colour_b = complementary_gradient(rng)

        objects.append(
            sphere_object(
                x=x,
                y=y,
                z=z,
                scale=scale,
                colour_a=colour_a,
                colour_b=colour_b,
                mirror=rng.random() < mirror_chance,
            )
        )

    return ",\n".join(objects)


def main() -> None:
    output_path = Path("spheres.ron")
    output_path.write_text(generate_spheres(), encoding="utf-8")
    print(f"Wrote {output_path}")


if __name__ == "__main__":
    main()
