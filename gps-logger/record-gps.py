#!/usr/bin/python3

from gps import *
import time
import argparse

parser = argparse.ArgumentParser(description='GPS Logger')
parser.add_argument('-o', '--output', help='Output file', required=True)
parser.add_argument('-i', '--interval', help='Interval in seconds', required=False)

args = parser.parse_args()

filename = args.output + time.strftime("%Y-%m-%dT%H-%M-%S") + "_GPS_data.csv"

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

				if args.interval:
					time.sleep(int(args.interval))
				else:
					time.sleep(5)
	
	except (KeyboardInterrupt, SystemExit): # when you press ctrl+c
		print("Done.\nExiting.")
		f.close()
