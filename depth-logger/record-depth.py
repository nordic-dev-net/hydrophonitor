#!/usr/bin/python3

import board
import time
import busio
import adafruit_ads1x15.ads1015 as ADS
from adafruit_ads1x15.analog_in import AnalogIn
from time import sleep, strftime

# Create the I2C bus
i2c = busio.I2C(board.SCL, board.SDA)

# Create the ADC object using the I2C bus
ads = ADS.ADS1015(i2c)

# Create single-ended input on channel 0
# tmp36 = AnalogIn(ads, ADS.P0)

# Attempting to create a single-ended input on channel 1
depthS = AnalogIn(ads, ADS.P1)

# Subtract the offset from the sensor voltage
# and convert chan.voltage * (1 degree C / 0.01V) = Degrees Celcius
# temperatureC = (tmp36.voltage - 0.5) / 0.01

# Open the file to write down the results
timestr = time.strftime("%Y-%m-%dT%H-%M-%S")
filename = "/home/shared/logger-raspi-setup/data/depth/" + timestr + "_depth_data.csv"

#depthM = ((depthS.voltage * 31.848) - 22.93)

#Attempting to round the figure to a more intelligible figure
#rounddepth = round(depthM, ndigits)
#psi = depthS.voltage * 104.1666667 - 75

#bar = psi * 14.503773800722

with open(filename, "w", 1) as f:
    f.write("time and date, Voltage of depth sensor (V), Depth (m)\n")

    while True:
        voltage = depthS.voltage
        depthM = ((voltage * 31.848) - 22.93)
        rounddepth = round(depthM, 2)
        # roundtemp = round(temperatureC, 2)
        roundvolts = round(voltage, 3)

        print((str(voltage) + " V  ") + (str(depthM) + " m ") + (str(roundvolts) + " V  ") + (str(rounddepth) + " m"))

        f.write(time.strftime("%Y-%m-%dT%H:%M:%S") + ",")
        f.write(str(roundvolts) + "," + str(rounddepth) + "\n")
        
        time.sleep(3)
