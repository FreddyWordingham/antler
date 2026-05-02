from PIL import Image

base_path = "output/my_scene-my_capture-my_image-temporal.png"
overlay_path = "output/my_scene-my_capture-my_image.png"
out_path = "output.png"

base = Image.open(base_path).convert("RGBA")
overlay = Image.open(overlay_path).convert("RGBA")

# Standard alpha compositing: overlay over base
result = Image.alpha_composite(base, overlay)

# Save as PNG to preserve result quality
result.save(out_path)
