#!/usr/bin/python3

import gpsd
import time
import argparse

parser = argparse.ArgumentParser(description='GPS Logger')
parser.add_argument('-o', '--output', help='Output directory', required=True)
parser.add_argument('-i', '--interval', help='Interval in seconds', required=False)

args = parser.parse_args()

filename = args.output + "/" + time.strftime("%Y-%m-%dT%H-%M-%S") + "_GPS_data.csv"

interval = int(args.interval) if args.interval else 5

with open(filename, "w", 1) as f:
	gpsd.connect()

	print(f"Writing GPS output to {filename}, interval {interval} seconds", flush=True)
	f.write("system_time,gps_time_utc,latitude,longitude,speed,sats_in_view\n")

	while True:
		try:
			packet = gpsd.get_current()
			gps_time_utc =  str(packet.time) if packet.mode >= 2 else "-"
			lat = str(packet.lat) if packet.mode >= 2 else "0.0"
			lon = str(packet.lon) if packet.mode >= 2 else "0.0"
			speed =  str(packet.hspeed) if packet.mode >= 2 else "0.0"
			sats = str(packet.sats)
			system_time = time.strftime("%Y-%m-%dT%H-%M-%S")
			f.write(f"{system_time},{gps_time_utc},{lat},{lon},{speed},{sats}\n")
		except (KeyboardInterrupt, SystemExit): # when you press ctrl+c
			print("Exiting GPS recording.", flush=True)
			break
		except Exception as e:
			print(f"GPS error: {e}, trying again...", flush=True)
		
		time.sleep(interval)
