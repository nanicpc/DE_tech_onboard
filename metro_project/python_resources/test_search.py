#!/usr/bin/env python3
__author__      = "nanicpc"
# This file implements the algorithms for path search that will return all the possible paths
# that connect one node with another. It does it by using the deep-first approach dfs and the
# breath first approach.
# a basic graph is used as example and both algoritmns are timeit to check performance on the
# provided graph

import time
from functools import wraps


def timeit(f):
    @wraps(f)
    def timed(*args, **kw):
        ts = time.time()
        result = f(*args, **kw)
        te = time.time()
        print('func:%r args:[%r, %r] took: %2.8f msec' % (f.__name__, args, kw, (te-ts)*1000))
        return result
    return timed


def dfs(graph, dfs_stack, target_value, visited=None):
    current_vertex = dfs_stack[-1]
    if visited is None:
        visited = []
    visited.append(current_vertex)
    if current_vertex is target_value:
        all_paths.append(dfs_stack.copy())
        return dfs_stack
    for neighbor in graph[current_vertex]:
        # if neighbor not in visited:
        dfs_stack.append(neighbor)
        path = dfs(graph, dfs_stack, target_value, visited)
        if neighbor == dfs_stack[-1]:
            dfs_stack.pop()
            # print("removing ", )
        elif neighbor is target_value:
            dfs_stack.append(neighbor)
            all_paths.append(dfs_stack.copy())
    # print("removing ", )
    dfs_stack.pop()


def bfs(graph, start_vertex, target_value):
    path = [start_vertex]
    bfs_queue = [[start_vertex, path]]
    visited = set()
    while bfs_queue:
        current_vertex, path = bfs_queue.pop(0)
        visited.add(current_vertex)
        for neighbor in graph[current_vertex]:
            if neighbor not in visited:
                if neighbor == target_value:
                    all_paths.append(path + [neighbor])
                else:
                    bfs_queue.append([neighbor, path + [neighbor]])


@timeit
def call_search(fun, graph, start, end):
    global all_paths
    all_paths = []
    fun(graph, start, end)
    print(fun.__name__, all_paths)


some_hazardous_graph = {
    'a': set(('b', 'c')),
    'b': set(('d')),
    'c': set(('f', 'g')),
    'd': set(('e', 'h')),
    'e': set(('j')),
    'f': set(('i')),
    'g': set(('i')),
    'h': set(('j')),
    'i': set(),
    'j': set(('k', 'l')),
    'k': set(('m')),
    'l': set(('m')),
    'm': set(),
  }   

call_search(dfs, some_hazardous_graph, ['a'], 'm')
call_search(bfs, some_hazardous_graph, 'a', 'm')