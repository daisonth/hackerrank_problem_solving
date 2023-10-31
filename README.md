# Hackerrank Solutions In RUST

## 1. Prim's (MST) : Special Subtree
  - [Problem](https://www.hackerrank.com/challenges/primsmstsub/copy-from/353032139) 
  - [Solution](./prims_special_subtree/src/main.rs)
  - **Explanation** :  The `lazy_prims` function implements the lazy version of Prim's algorithm for finding the minimum spanning tree (MST) in a weighted graph. It takes as input the number of nodes `num`, a list of edges represented as tuples with their endpoints and weights, and a starting node `start`. The function initializes a priority queue (`pq`) to track edges, a boolean array(vis) to mark visited nodes, and an adjacency list(g) to represent the weighted graph. It starts from the specified node, continually adds the minimum-weight edge connecting the MST to the unvisited nodes, and updates the priority queue accordingly. This process continues until the MST has n-1 edges (where n is the number of nodes). The function returns the total cost of the MST. In summary, the code constructs an MST in a graph by iteratively selecting the shortest edge connected to the growing MST.
 
## 2. Dijkstra: Shortest Reach 2
  - [Problem](https://www.hackerrank.com/challenges/dijkstrashortreach/problem) 
  - [Solution](./dijkstra_shortest_reach_2/src/main.rs)
  - **Explanation** : This `shortestReach` function, implements Dijkstra's algorithm to find the shortest distances from a specified source node 's' to all other nodes in a weighted graph. It takes three parameters: 'n' representing the number of nodes in the graph, 'edges' as a vector of vectors representing the edges and their weights, and 's' as the source node. The function initializes vector vis to keep track of visited nodes, vector dist for distances, and priority queue (pq) for node traversal. It constructs an adjacency list for the graph based on the input edges. The algorithm then iteratively explores the graph, starting from the source node and updating the shortest distances as it goes. It uses a priority queue to choose the node with the minimum distance, explores its neighbors, and relaxes the distances if a shorter path is found. Finally, it returns a vector containing the shortest distances from the source node to all other nodes in the graph.
   
## 3. Morgan and a String
  - [Problem](https://www.hackerrank.com/challenges/morgan-and-a-string/problem) 
  - [Solution](./morgan_and_a_string/src/main.rs)
  - **Explanation** : function, morganAndString, takes two input strings 'a' and 'b' and merges them to create a new string. The goal is to make sure the new string is as small as possible when compared lexicographically.
> Here's how it works: It starts with the first characters of 'a' and 'b'. It picks the smaller character and adds it to the result 'r'. Then, it removes that character from its respective input string. This process continues until it's done. To ensure it can compare until the end, it adds a 'z' to both 'a' and 'b'. Finally, it removes any trailing 'z' and gives you the merged string 'r'. This way, it ensures the result is as small as possible while respecting the order of 'a' and 'b'.
   
## 4. Repetitive K-Sums
  - [Problem]() 
  - [Solution]()
  - **Explanation** : 
   
## 5. String Similarity
  - [Problem]() 
  - [Solution]()
  - **Explanation** : 
   
## 6. Determining DNA Health
  - [Problem]() 
  - [Solution]()
  - **Explanation** : 
   
## 7. Roads in HackerLand
  - [Problem]() 
  - [Solution]()
  - **Explanation** : 
   
## 8. The Maximum Subarray
  - [Problem]() 
  - [Solution]()
  - **Explanation** : 
   
## 9. Connected Cells in a Grid
  - [Problem]() 
  - [Solution]()
  - **Explanation** : 
   
## 10. Almost Sorted
  - [Problem]() 
  - [Solution]()
  - **Explanation** : 
   
