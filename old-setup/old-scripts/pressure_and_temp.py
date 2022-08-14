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
tmp36 = AnalogIn(ads, ADS.P0)

# Attempting to create a single-ended input on channel 1
depthS = AnalogIn(ads, ADS.P1)

# Subtract the offset from the sensor voltage
# and convert chan.voltage * (1 degree C / 0.01V) = Degrees Celcius
TemperatureC = (tmp36.voltage - 0.5) / 0.01

timestr = time.strftime("%Y%m%d-%H%M%S")


#Open the file to write down the results

f = open("/mnt/myssd/Temp_and_depth_Data" + timestr +'.csv', 'w')
#f = open("/mnt/myssd/('%Y%m%d-%H%M%S')+_Temp_Depth.csv",'w')
#print('Temperature\depthSensor voltage\depth')

#f.write("Temperature C,Voltage,Depth")

#depthM = ((depthS.voltage * 31.848) - 22.93)
#Attempting to round the figure to a more intelligible figure
#rounddepth = round(depthM, ndigits)
#psi = depthS.voltage * 104.1666667 - 75

#bar = psi * 14.503773800722

##################This is a test for making the pi write on external ssd ########

#with open("mnt/myssd/tempDepth.csv", "a") as a log:
with open("/mnt/myssd/Temp_and_depth_Data" + timestr +'.csv', 'w' as f:
    f.write('time and date, temperature (C), Voltage of depth sensor (V), Depth (m)\n')

    while True:

        depthM = ((depthS.voltage * 31.848) - 22.93)

        rounddepth = round(depthM, 2)

        roundtemp = round(TemperatureC, 2)

        roundvolts = round(depthS.voltage, 3)

        print((str(roundtemp) + ' °C  ') + (str(roundvolts) + ' V  ') + (str(rounddepth) + ' m'))
        #print(str(roundvolts) + ' V')
        #print(str(rounddepth) + ' m')
        #print(str(rounddepth) + ' m')
        f.write(time.strftime("%Y-%m-%d %H:%M:%S") + ',')
        f.write((str(roundtemp) + ' °C  ') + ',' + (str(roundvolts) + ' V  ') + ',' + (str(rounddepth) + ' m\n'))
        
        time.sleep(3)





