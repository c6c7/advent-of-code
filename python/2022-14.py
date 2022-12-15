#!/usr/bin/env python3

import sys, re

coordinates_re = re.compile(r"(\d+),(\d+)")

def print_locations(rock_locations, y_range, x_range):
	for j in y_range:
		row = ""
		for i in x_range:
			row += rock_locations[j][i]
		print(row)

rock_paths = []
for line in sys.stdin:
	rock_path = []
	for rock_corner in line.rstrip().split(" -> "):
		rock_corner = list(map(lambda c: int(c), coordinates_re.match(rock_corner).groups()))
		rock_path.append(rock_corner)
	rock_paths.append(rock_path)

rock_locations = [["." for i in range(1000)] for i in range(1000)]

lowest_y = 0
for rock_path in rock_paths:
	prev_corner = None
	for rock_corner in rock_path:
		if prev_corner is not None:
			if prev_corner[0] == rock_corner[0]:
				r = [prev_corner[1], rock_corner[1]]
				r.sort()
				if r[1] >= lowest_y:
					lowest_y = r[1]
				r[1] += 1
				for j in range(r[0], r[1]):
					print(f"{j},{rock_corner[0]}")
					rock_locations[j][rock_corner[0]] = "#"
			else:
				r = [prev_corner[0], rock_corner[0]]
				r.sort()
				r[1] += 1
				print(r)
				for i in range(r[0], r[1]):
					print(f"{rock_corner[1]},{i}")
					rock_locations[rock_corner[1]][i] = "#"
		prev_corner = rock_corner

		print_locations(rock_locations, range(0,10), range(494, 504))

print(f"lowest_y: {lowest_y}")

# Part 2
for i in range(1000):
	rock_locations[lowest_y + 2][i] = "#"

ans = -1
while True:
	# Part 2
	if rock_locations[0][500] == "o":
		break

	sand = [0, 500]
	ans += 1
	while True:
		##Part 1
		#if sand[0] == lowest_y + 1:
		#	break
		if rock_locations[sand[0] + 1][sand[1]] == ".":
			sand[0] += 1
			continue
		if rock_locations[sand[0] + 1][sand[1] - 1] == ".":
			sand[0] += 1
			sand[1] -= 1
			continue
		if rock_locations[sand[0] + 1][sand[1] + 1] == ".":
			sand[0] += 1
			sand[1] += 1
			continue
		rock_locations[sand[0]][sand[1]] = "o"
		break

	## Part 1
	#if sand[0] >= lowest_y:
	#	break


# Part 1
#print(f"Part 1 Answer: {ans}")

# Part 2
print(f"Part 2 Answer: {ans + 1}")

print_locations(rock_locations, range(184), range(250,750))
