#!/usr/bin/env python3
import pygraphviz as pgv

def make_graph(d):
    G=pgv.AGraph(strict=False,directed=True,rankdir="LR")
    G.graph_attr["label"]="Metro Project"
    G.graph_attr["nodesep"] = 0.5
    G.graph_attr["mode"] ="kk"
    G.node_attr["shape"]="circle"
    G.edge_attr["color"]="gray"
    G.add_nodes_from(d.keys())
    # OP1=G.get_node(1)
    # OP2=G.get_node(5)
    # OP1.attr["shape"]="doublecircle"
    # OP2.attr["shape"]="doublecircle"
    for key in d.keys():
        for kkey in d[key].keys():
            G.add_edge(key,kkey,label=d[key][kkey])
    G.graph_attr["epsilon"]="0.001"
    G.layout("dot") # layout with dot
    # print(G.string()) # print dot file to standard output
    # G.write("metro.dot") # create .dot file 
    G.draw("metro_plan.png")

if __name__=='__main__':
    d = {"a": {"b": "A", "c": "C"},
    "b": {"d": "8 min"}, 
    "c": {"f": "6 min", "g": "D"},
    "d": {"e": "6 min", "h": "B"},
    "e": {"j": "3 min"},
    "f": {"i": "5 min"},
    "g": {"i": "4 min"},
    "h": {"j": "1.5 €"},
    "i": {},
    "j": {"k": "E", "l": "F"},
    "k": {"m": "3 min"},
    "l": {"m": "5 min"},
    "m": {}}
    make_graph(d)