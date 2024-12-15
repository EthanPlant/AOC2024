from PIL import Image
import random
import sys

if len(sys.argv) < 2:
    print("Usage: image_gen <PATH>")

bitmap = Image.open(sys.argv[1])
pixels = bitmap.load()

if bitmap.size > (101, 103):
    print("Bitmap too large!")

robots = set()
vels = []
steps = random.randint(1000, 9000)

offset_x = random.randint(0, 101 - bitmap.size[0])
offset_y = random.randint(0, 103 - bitmap.size[1])

for i in reversed(range(bitmap.size[1])):
    for j in range(bitmap.size[0]):
        if pixels[j, i] == 255:
            robots.add((j + offset_y, i + offset_x))
            vels.append((random.randint(-50, 50), random.randint(-50, 50)))

for (i, robot) in enumerate(robots):
    x = (robot[0] - (vels[i][0] * steps)) % 101
    y = (robot[1] - (vels[i][1] * steps)) % 103
    print(f"p={x},{y} v={vels[i][0]},{vels[i][1]}")

