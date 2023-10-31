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
    Here's how it works: It starts with the first characters of 'a' and 'b'. It picks the smaller character and adds it to the result 'r'. Then, it removes that character from its respective input string. This process continues until it's done. To ensure it can compare until the end, it adds a 'z' to both 'a' and 'b'. Finally, it removes any trailing 'z' and gives you the merged string 'r'. This way, it ensures the result is as small as possible while respecting the order of 'a' and 'b'.
   
## 4. Pairs
  - [Problem](https://www.hackerrank.com/challenges/pairs/problem) 
  - [Solution](./pairs/src/main.rs)
  - **Explanation** : function, 'pairs', counts the number of pairs in an array of integers that have a specific target difference 'k'. It achieves this by sorting the input array, iterating through each pair of elements, and checking if their difference equals 'k'. If a pair with the desired difference is found, it increments a count. The function then returns the count, which represents the number of pairs with the specified difference 'k' in the array.
   
## 5. String Similarity
  - [Problem](https://www.hackerrank.com/challenges/string-similarity/problem) 
  - [Solution](./string_similarity/src/main.rs)
  - **Explanation** : function, 'stringSimilarity', figures out how much the beginning of a word is similar to its suffixes. It does this by checking the number of characters at the start of the word that are also at the start of its different suffixes. The more characters they have in common, the higher the similarity. It adds up these similarities for all suffixes of the word and returns the total similarity score. 
   
## 6. Determining DNA Health
  - [Problem](https://www.hackerrank.com/challenges/determining-dna-health) 
  - [Solution](./determining_dna_health/src/main.rs)
  - **Explanation** : here we begins by storing genes and their health values in a Trie data structure for efficient search. Then, for each DNA strand, we searches the Trie to find matching genes character by character and calculates the total health by summing up the associated health values. we keeps track of the minimum and maximum health values among all the DNA strands and prints them, helping determine the healthiest and least healthy DNA strands based on the provided genes and their health values.
   
## 7. Common Child
  - [Problem](https://www.hackerrank.com/challenges/common-child) 
  - [Solution](./common_child/src/main.rs)
  - **Explanation** : function commonChild,finds the length of the longest common subsequence between two input strings, s1 and s2, using dynamic programming. It compares each character of s1 with each character of s2 and constructs a two-dimensional matrix dp to track the length of common subsequences. When characters match, it increments the count, and when they differ, it takes the maximum value from adjacent cells. The final value in dp[s1.len()][s2.len()] represents the length of the longest common subsequence. Essentially, it efficiently computes the length of the longest shared sequence of characters between the two input strings.
   
## 8. The Maximum Subarray
  - [Problem]() 
  - [Solution]()
  - **Explanation** : 
   
## 9. Time Conversion
  - [Problem](https://www.hackerrank.com/challenges/time-conversion/problem) 
  - [Solution](./Time_conversion/src/main.rs)
  - **Explanation** : function, timeConversion, takes a 12-hour time string and converts it to a 24-hour format. It first checks whether it's morning or evening by examining the last character ('A' or 'P'). If it's morning ('A'), and the hour is '12', it changes it to '00'; otherwise, it leaves it as is. If it's evening ('P'), it adds 12 to the hour, except if it's already '12'. The minutes and seconds are retained, and the result is returned as a 24-hour time string
   
## 10. Almost Sorted
  - [Problem]() 
  - [Solution]()
  - **Explanation** : 
   
