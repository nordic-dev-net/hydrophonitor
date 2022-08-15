# PamGuard

PamGuard seems to be a modular software package with a developer library and good documentation. It's unclear if it's intended for real time use, but I think the first step should be to test if PamGuard itself can be run on the Raspberry Pi. According to its documentation it involves modules for the whole audio acquisition and processing signal-chain.

[PamGuard Website](https://www.pamguard.org/)
[PamGuard Modules](https://www.pamguard.org/11_PluginModules.html)
[PamGuard Developer Documentation (core API)](http://pamguard.sourceforge.net/API_1_15_17/doc/index.html)

**Devil's Advocacy**

* Using PamGuard could further restrict our ability to ungracefully shut down the device.

* A more complex signal processing chain on the device increases the possibility of crashes during recording.

* Might be too heavy for the Raspi.

# Hardware Schema

```

					[GPS] ---------               ---> [Micro SD]
								    \            /
					[Depth] -------- >[RasPi] -- ---> optional [Ext. Disk]
								    /            \
[Hydrophone] ---> [Sound Card] ----               ---> optional [RT-Stream]
							  \
							    --[Headphones]
```

# Data

* PamGuard plug-in types describes the different data-types expected by the system [described here](https://www.pamguard.org/11_PluginModules.html)

* GPS sensor data will be output in `.nmea` format [described here](https://www.gpsworld.com/what-exactly-is-gps-nmea-data/). The format includes a time-stamp and various metadata. This is already handled in the current implementation.

* Depth-data is mentioned in `Hydrophone Depth Readout` under `Sensors`. It doesn't specify the exact format so far as I can find. I guess we can assume signed integers. A thing to investigate is if the depth info should be appended to the `.nmea` format which has extensions. This would also put it under the same time synchronization automatically.

* How do we handle time synchronization between the `.wav` and `.nmea` formats? I assume we want to know accurately where a sound was recorded, when and at what depth. Simple way would be to store `.wav` timestamps in a separate `.xml` file.

* There are also a plug-in for `Array Accelerometer` in relation to hydrophone. Is accelerometer (used to calculate the angle of the hydrophone) relevant as a future feature?

* Possible output of the system would be as follows:

```
timestamp = dd-mm-yyyy-H-M-S

output/
	[timestamp].nmea			// GPS, timestamp etc. National Marine Electronics Association format.
	audio/						// Audio is recorded in chunks and combined at the end to avoid file 						   corruption on shutdown.
		[idx]-[timestamp].wav		// Raw audio recording chunk.
		[timestamp]-metadata.xml	// Recording start time for .wav file (just in case
								       someone decides to rename the files),
	[timestamp]-depth.csv		// In case depth can't be included in the .nmea file.
```

# Locating whales

These are some notes about the possibility of doing processing on the Raspberry Pi in order to do real-time location. These are to be considered as future development ideas, not part of the initial plan.

I can derive the following signal processing steps (roughly) for locating whales:

```

1. Sound Acquisition => audio data
2. (optional) Filtering, processing => audio data
3. FFT (Spectrogram) Engine => FFT data
4. (optional) Long Term Spectral Average => FFT data
5. (optional) Spectrogram smoothing kernel => FFT data
6. Detector => Measurement

```

These seem to be modules or `plug-ins` to PamGuard. We can investigate the possibility of handling (some) of these processing steps on the device. PamGuard seems to have a well documented development library:

[PamGuard Library](http://pamguard.sourceforge.net/API_1_15_17/doc/overview-summary.html)

**Open Questions**

* Can we run PamGuard itself on the Raspberry Pi? It seems to have modules for audio acquisition, GPS acquisition etc. We should test the performance of using PamGuard itself on the device and if the process can be scripted in a way that adheres to the use-case conditions.

* If we acquire and process data in real time on the device, how do we output and display it? I presume the most convenient way would be to display the data on a Marine GPS Unit or a mobile device through an app assuming we want to see the location on the map.