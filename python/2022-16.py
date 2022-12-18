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

def simplify(G, me, el):
	for node in no_flow_rate_nodes(G):
		if node in [me, el]:
			continue
		replace_node(G, node)

def max_possible_remaining(G, steps_left):
	max_remaining = 0
	nodes_by_flow_rate = list(map(lambda nv: nv[1], G.nodes(data='flow_rate')))
	nodes_by_flow_rate.sort()
	steps_left.sort(reverse=True)
	while len(nodes_by_flow_rate) > 0 and len(list(filter(lambda sl: sl >= 2, steps_left))) > 0:
		for i in range(len(steps_left)):
			if steps_left[i] < 2:
				continue
			if len(nodes_by_flow_rate) == 0:
				break

			max_remaining += nodes_by_flow_rate.pop() * (steps_left[i] - 1)
			steps_left[i] -= 2
	return max_remaining

def neighbors_by_flow_rate(G, node):
	neighbors = list(filter(lambda nv: nv[0] in G.neighbors(node), G.nodes.data('flow_rate')))
	neighbors.sort(key = lambda nv: nv[1], reverse = True)
	return list(map(lambda nv: nv[0], neighbors))

def F(G, me, me_steps_left, el, el_steps_left, total_pressure_released):
	global global_max, best_me_path, best_el_path, number_of_actors
	simplify(G, me[-1], el[-1])

	assert me_steps_left >= 0 and el_steps_left >= 0
	if me_steps_left == 0 and el_steps_left == 0:
		return

	mpr = max_possible_remaining(G, [me_steps_left] if number_of_actors == 1 else [me_steps_left, el_steps_left])
	if total_pressure_released + mpr < global_max:
		return

	turn = None
	if me_steps_left >= el_steps_left:
		turn = 'me'
	else:
		turn = 'el'

	if turn == 'me':
		actor = me[-1]
		other_actor = el[-1]
	elif turn == 'el':
		actor = el[-1]
		other_actor = me[-1]
	else:
		assert False
	
	for (action, next_position) in [('open', actor), ('move', actor)] + [('open', n) for n in neighbors_by_flow_rate(G, actor)] + [('move', n) for n in neighbors_by_flow_rate(G, actor)]:
		if turn == 'me':
			actor_steps_left = me_steps_left
		elif turn == 'el':
			actor_steps_left = el_steps_left
		else:
			assert False
	
		H = G.copy()
		pressure_added = 0
		if action == 'open' and H.nodes[actor]['flow_rate'] > 0:
			pressure_added += H.nodes[actor]['flow_rate'] * (actor_steps_left - 1)
			nx.set_node_attributes(H, {actor: 0}, name='flow_rate')
			actor_steps_left -= 1
		elif action == 'move':
			pass
		else:
			continue

		if next_position == actor:
			actor_steps_left = 0
			next_position = 'ZZZ'
		elif H[actor][next_position]['weight'] + 1 < actor_steps_left:
			actor_steps_left -= H[actor][next_position]['weight']
		else:
			continue
		
		next_total_pressure_released = total_pressure_released + pressure_added
		if next_total_pressure_released > global_max:
			global_max = next_total_pressure_released
			print(f"global_max update: {global_max}")
			best_me_path = me[:]
			best_el_path = el[:]

		if turn == 'me':
			F(H, me + [next_position], actor_steps_left, el, el_steps_left, next_total_pressure_released)
		elif turn == 'el':
			F(H, me, me_steps_left, el + [next_position], actor_steps_left, next_total_pressure_released)
		else:
			assert False

global_max = 0
best_me_path = None
best_el_path = None
number_of_actors = 1

#F(G.copy(), ['AA'], 30, ['ZZZ'], 0, 0)
#print(f"Part 1 Answer: {global_max} {best_me_path}")

#global_max = 2138
global_max = int(sys.argv[1])
number_of_actors = 2
F(G.copy(), ['AA'], 26, ['AA'], 26, 0)
print(f"Part 2 Answer: {global_max}\n{best_me_path}\n{best_el_path}")

