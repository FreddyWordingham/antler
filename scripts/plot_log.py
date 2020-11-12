import matplotlib.pyplot as plt
import numpy as np
import sys


X_AXIS_LABEL = 'time per pixel [milliseconds]'
Y_AXIS_LABEL = 'counts'


def quit_figure(event):
    if event.key == 'escape':
        plt.close(event.canvas.figure)


cid = plt.gcf().canvas.mpl_connect('key_press_event', quit_figure)

filename = sys.argv[1]
print("Loading file", filename)

# data = np.genfromtxt(filename, delimiter=',', names=True)

# for name in data.dtype.names[1:]:
# plt.plot(data['time'], data[name], label=name)

data = np.genfromtxt(filename, delimiter=',', names=['x', 'counts'])
plt.plot(data['x'], data['counts'], '.')
# plt.bar(data['x'], data['counts'], width=0.8)
plt.yscale("log")

plt.xlabel(X_AXIS_LABEL)
plt.ylabel(Y_AXIS_LABEL)
plt.title(filename)
plt.legend()

plt.show()
plt.close()
