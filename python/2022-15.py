#!/usr/bin/env python3

import sys, re

coordinates_re = re.compile(r".*x=(-?\d+), y=(-?\d+)")

target_row = 2000000
target_row_hits = set()
known_beacons = set()

def manhattan_distance(a, b):
	return abs(a[0] - b[0]) + abs(a[1] - b[1])

information = []

for line in sys.stdin:
	[sensor, closest_beacon] = line.rstrip().split(":")
	sensor = tuple(map(lambda c: int(c), coordinates_re.match(sensor).groups()))
	closest_beacon = tuple(map(lambda c: int(c), coordinates_re.match(closest_beacon).groups()))
	known_beacons.add(closest_beacon)
	information.append({'sensor': sensor, 'beacon': closest_beacon, 'distance': manhattan_distance(sensor, closest_beacon)})

for target_row in range(4000000 + 1):
	covered_intervals = []
	for info in information:
		sensor = info['sensor']
		distance = info['distance']
		beacon = info['beacon']
		d = abs(sensor[1] - target_row)
		if d > distance:
			continue
		begin = sensor[0] - distance + d
		end = sensor[0] + distance - d
		covered_intervals.append((begin, end))
		if beacon[1] == target_row:
			covered_intervals.append((beacon[0], beacon[0]))

	covered_intervals.sort()
	last_end = -1
	done = False
	for interval in covered_intervals:
		if interval[1] < 0 or interval[0] > 4000000:
			continue
		if interval[0] >= last_end + 2:
			print(f"Part 2 Answer: {(last_end + 1) * 4000000 + target_row} ({last_end + 1},{target_row})")
			done = True
			break
		last_end = max(last_end, interval[1])

	if done:
		break


	## Part 1
	#d = abs(sensor[1] - target_row) 
	#if d <= distance:
	#	target_row_hits.update(range(sensor[0] - distance + d, sensor[0] + distance - d + 1))

#print(f"Part 1 Answer: {len(target_row_hits - beacons_in_row)}")
