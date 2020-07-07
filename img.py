import numpy as np
from PIL import Image
import os

s = 32
n = len(os.listdir("./assets/frames/"))
print(n)
master_arr = None
for i in range(1, n + 1):
    t = Image.open("./assets/frames/%06d.png" % i).resize((s, s))
    t = np.asarray(t, dtype=np.uint32)
    f = np.zeros((s, s))
    f += (t[:, :, 2] << 16)
    f += t[:, :, 1] << 8
    f += t[:, :, 0]
    f += 0xFF000000
    if not master_arr is None:
        master_arr = np.hstack([master_arr, f.reshape((1024, 1))])
    else:
        master_arr = f.reshape((1024, 1))
np.savetxt("out.txt", master_arr, fmt="%d")
