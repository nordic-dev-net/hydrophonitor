#!/usr/bin/python3

from gps import *
from time import sleep, strftime 

filename = "/home/shared/logger-raspi-setup/data/gps/" + time.strftime("%Y-%m-%dT%H-%M-%S") + "_GPS_data.csv"
# filename = "/mnt/myssd/GPS_Data" + timestr +".csv"

with open(filename, "w", 1) as f:
	gpsd = gps(mode=WATCH_ENABLE|WATCH_NEWSTYLE)
	f.write("GPStime utc,latitude,longitude,speed,sats in view\n")

	try:
		while True:
			report = gpsd.next()
			if report["class"] == "TPV":
				GPStime =  str(getattr(report,"time",""))
				lat = str(getattr(report,"lat",0.0))
				lon = str(getattr(report,"lon",0.0))
				speed =  str(getattr(report,"speed","nan"))
				sats = str(len(gpsd.satellites))

				f.write(GPStime + "," + lat +"," + lon + "," + speed + "," + sats + "\n")
	
				time.sleep(5)
	
	except (KeyboardInterrupt, SystemExit): # when you press ctrl+c
		print("Done.\nExiting.")
		f.close()
