
import pandas as pd
import matplotlib.pyplot as plt
import numpy as np

# Load the CSV data into a pandas DataFrame
data = pd.read_csv('../logs/spacecraft_state.csv')

# Label helpers
w_idx_label = ['X','Y','Z']
q_idx_label = ['i', 'j', 'k', 'scalar']

# Stack data into state x N arrays
w = np.vstack((data['wx'], data['wy'], data['wz']))
q_i2b = np.vstack((data['qi'], data['qj'], data['qk'], data['qw']))
# plot angular velocity
plt.figure()
for ii in range(3):
    plt.plot(data['time'], w[ii], label=w_idx_label[ii])
plt.xlabel('Time (s)')
plt.ylabel('w (rad/s)')
plt.title('Spacecraft Angular Velocity')
plt.legend()
plt.grid(True)
# plot attitude
plt.figure()
for ii in range(4):
    plt.plot(data['time'], q_i2b[ii], label=q_idx_label[ii])
plt.xlabel('Time (s)')
plt.ylabel('q_i2b (--)')
plt.title('Attitude (Inertial to Body)')
plt.legend()
plt.grid(True)
# show plots
plt.show()

