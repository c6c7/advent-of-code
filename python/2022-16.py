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

for node in no_flow_rate_nodes(G):
	if node == 'AA':
		continue
	replace_node(G, node)

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

global_max = 1767
global_best_path = None
def F(G, me, el, total_pressure_released, steps_left):
	global global_max
	# print(f" --------- ")
	# print(f"  path: {path}\n  location: {location}\n  total_pressure_released: {total_pressure_released}\n  steps_left: {steps_left}")N
	# print(f"{[(node, nodedata) for (node, nodedata) in G.nodes.items()]}")

	if steps_left == 0:
		return
	if total_pressure_released + max_possible_remaining(G, steps_left) < global_max:
		return

	for me_action in [('open', None)] + [('move', n) for n in G.neighbors(me)]:
		for el_action in [('open', None)] + [('move', n) for n in G.neighbors(el)]:
			H = G.copy()
			additional_flow_rate = 0
			next_positions = []	
			for (actor, (action, neighbor)) in [(me, me_action), (el, el_action)]:
				if action == 'open' and H.nodes[actor]['flow_rate'] != 0:
					next_positions.append(actor)
					additional_flow_rate += H.nodes[actor]['flow_rate']
					nx.set_node_attributes(H, {actor: 0}, name='flow_rate')
				elif action == 'move':
					next_positions.append(neighbor)
				else:
					continue
			if len(next_positions) < 2:
				continue

			total_pressure_released += additional_flow_rate * (steps_left - 1)
			global_max = max(global_max, total_pressure_released) 
			F(H, next_positions[0], next_positions[1], total_pressure_released, steps_left - 1)

F(G, 'AA', 'AA', 0, 30)

print(f"Part 2 Answer: {global_max}")

