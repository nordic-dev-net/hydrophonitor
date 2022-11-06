#!/usr/bin/python3

import argparse
import time
import board
import busio
import adafruit_ads1x15.ads1015 as ADS
from adafruit_extended_bus import ExtendedI2C as I2C
from adafruit_ads1x15.analog_in import AnalogIn
# from rpi_lcd import LCD

parser = argparse.ArgumentParser(description='GPS Logger')
parser.add_argument('-o', '--output', help='Output directory', required=True)
parser.add_argument('-i', '--interval', help='Interval in seconds', required=False)
parser.add_argument('-b', '--bus', help='Custom i2c bus to use', required=False)

args = parser.parse_args()

# try:
#     lcd = LCD(bus=2)
# except OSError:
#     lcd = None

# Create the I2C bus
if args.bus:
	i2c_bus = args.bus
	i2c = I2C(i2c_bus)
else:
	i2c = busio.I2C(board.SCL, board.SDA)

# Create the ADC object using the I2C bus
ads = ADS.ADS1015(i2c)

# Create a single-ended input on channel 1
depthS = AnalogIn(ads, ADS.P1)

# Create single-ended input on channel 0
# tmp36 = AnalogIn(ads, ADS.P0)

# Subtract the offset from the sensor voltage
# and convert chan.voltage * (1 degree C / 0.01V) = Degrees Celcius
# temperatureC = (tmp36.voltage - 0.5) / 0.01

# File to write down the results
filename = args.output + "/" + time.strftime("%Y-%m-%dT%H-%M-%S") + "_depth_data.csv"

interval = int(args.interval) if args.interval else 5

#Attempting to round the figure to a more intelligible figure
#rounddepth = round(depthM, ndigits)
#psi = depthS.voltage * 104.1666667 - 75
#bar = psi * 14.503773800722

with open(filename, "w", 1) as f:
    print(f"Writing pressure/depth output to {filename}, interval {interval} seconds", flush=True)
    f.write("time and date, Voltage of depth sensor (V), Depth (m)\n")

    try:
        while True:
            voltage = depthS.voltage
            depthM = ((voltage * 31.848) - 22.93)
            rounddepth = round(depthM, 2)
            roundvolts = round(voltage, 3)
            # roundtemp = round(temperatureC, 2)

            print((str(voltage) + " V  ") + (str(depthM) + " m ") + (str(roundvolts) + " V  ") + (str(rounddepth) + " m"), flush=True)

            # if lcd:
            #     lcd.clear()
            #     lcd.text((str(roundvolts) + " V  ") + (str(rounddepth) + " m"), 1)
            
            f.write(time.strftime("%Y-%m-%dT%H:%M:%S") + "," + str(roundvolts) + "," + str(rounddepth) + "\n")

            time.sleep(interval)

    except (KeyboardInterrupt, SystemExit): # when you press ctrl+c
        print("Exiting depth recording.", flush=True)
