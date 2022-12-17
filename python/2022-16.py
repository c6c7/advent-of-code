#!/usr/bin/env python3

import networkx as nx, sys, re

G = nx.Graph()

rough_line_re = re.compile(r"Valve (..) has flow rate=(\d+); tunnels? leads? to valves? (.*)")

input = []
no_flow_rate_nodes = []
for line in sys.stdin:
	[valve_name, flow_rate, neighbors] = rough_line_re.match(line.rstrip()).groups()
	neighbors = neighbors.split(", ")

	input.append((valve_name, int(flow_rate), neighbors))

for (valve_name, flow_rate, _) in input:
	G.add_node(valve_name, flow_rate=flow_rate)
	if valve_name != 'AA' and flow_rate == 0:
		no_flow_rate_nodes.append(valve_name)

for (valve_name, _, neighbor_list) in input:
	for neighbor in neighbor_list:
		G.add_edge(valve_name, neighbor, weight=1)

print("Before simplification")
print(f"{[(node, nodedata) for (node, nodedata) in G.nodes.items()]}")

graph_simplified = False

def replace_node(G, node):
	neighbors = list(G.neighbors(node))
	for i in range(len(neighbors)):
		x = neighbors[i]
		for j in range(i + 1, len(neighbors)):
			y = neighbors[j]
			new_edge_weight = G[x][node]['weight'] + G[node][y]['weight']
			if not G.has_edge(x, y):
				G.add_edge(x, y, weight=new_edge_weight)	
			elif new_edge_weight < G[x][y]['weight']:
				G.remove_edge(x, y)
				G.add_edge(x, y, weight=new_edge_weight)
	G.remove_node(node)

def no_flow_rate_nodes(G):
	return list(map(lambda n: n[0], filter(lambda n: n[1] == 0, G.nodes(data='flow_rate'))))

def simplify(G, omit=None):
	for node in no_flow_rate_nodes(G):
		if node == omit:
			continue
		replace_node(G, node)

simplify(G, omit='AA')
print("After simplification")
print(f"{[(node, nodedata) for (node, nodedata) in G.nodes.items()]}")

def max_possible_remaining(G, steps_left):
	max_remaining = 0
	if steps_left <= 1:
		return max_remaining

	nodes_by_flow_rate = list(G.nodes(data='flow_rate'))
	nodes_by_flow_rate.sort(key = lambda n: -n[1])
	for (_, flow_rate) in nodes_by_flow_rate:
		max_remaining += flow_rate * (steps_left - 1)
		steps_left -= 2
		if steps_left <= 1:
			break
	return max_remaining

global_max = 0
global_best_path = None
def update_best(local_max, local_best_path):
	global global_max, global_best_path
	if local_max > global_max:
		global_max = local_max
		global_best_path = local_best_path

def max_pressure_released(G, path, location, total_pressure_released, steps_left):
	# print(f" --------- ")
	# print(f"  path: {path}\n  location: {location}\n  total_pressure_released: {total_pressure_released}\n  steps_left: {steps_left}")N
	# print(f"{[(node, nodedata) for (node, nodedata) in G.nodes.items()]}")

	assert steps_left >= 0
	if steps_left <= 1:
		return

	new_flow_rate = G.nodes[location]['flow_rate']
	assert new_flow_rate > 0 or location == 'AA'
	if total_pressure_released + max_possible_remaining(G, steps_left) < global_max:
		return

	# Stay put
	update_best(total_pressure_released + new_flow_rate * (steps_left - 1), path)

	# Nowhere left to go open
	if len(G) == 1:
		return

	for neighbor in G.neighbors(location):
		steps_to_next = G[location][neighbor]['weight']

		# Opening and moving is no better than opening and staying put
		if steps_to_next + 1 > steps_left:
			continue

		# Open the valve
		H = G.copy()
		replace_node(H, location)
		assert not H.has_node(location)
		# Always "open" valve AA, but it doesn't cost a step
		aa_bias = 1 if location == 'AA' else 0
		max_pressure_released(
				H,
				path + [neighbor],
				neighbor,
				total_pressure_released + new_flow_rate * (steps_left - 1),
				steps_left - steps_to_next - 1 + aa_bias)

		# Skip the valve
		if location != 'AA':
			max_pressure_released(
					G,
					path + [neighbor],
					neighbor,
					total_pressure_released,
					steps_left - steps_to_next)

max_pressure_released(G, ['AA'], 'AA', 0, 30)

print(f"Part 1 Answer: {global_max}, {global_best_path}")

