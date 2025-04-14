
import pandas as pd
import matplotlib.pyplot as plt
import numpy as np

# Load the CSV data into a pandas DataFrame
data = pd.read_csv('../logs/spacecraft_state.csv')

# Plotting the 'w' (wx, wy, wz) values against time
plt.figure(figsize=(10, 6))
plt.plot(data['time'], data['wx'], label='wx', marker='o')
plt.plot(data['time'], data['wy'], label='wy', marker='o')
plt.plot(data['time'], data['wz'], label='wz', marker='o')
plt.xlabel('Time (s)')
plt.ylabel('w values')
plt.title('Angular velocity components over time')
plt.legend()
plt.grid(True)
plt.show()

# Plotting the 'q' (qw, qi, qj, qk) values against time
plt.figure(figsize=(10, 6))
plt.plot(data['time'], data['qw'], label='qw', marker='o')
plt.plot(data['time'], data['qi'], label='qi', marker='o')
plt.plot(data['time'], data['qj'], label='qj', marker='o')
plt.plot(data['time'], data['qk'], label='qk', marker='o')
plt.xlabel('Time (s)')
plt.ylabel('q values')
plt.title('Quaternion components over time')
plt.legend()
plt.grid(True)
plt.show()

