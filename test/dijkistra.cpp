#include <bits/stdc++.h>

using namespace std;

string ltrim(const string &);
string rtrim(const string &);
vector<string> split(const string &);

vector<int> shortestReach(int n, vector<vector<int>> edges, int s) {

    vector<bool> visited(n+1, false);
    
    vector<int> distances(n+1, -1);
    distances[s] = 0;
    
    /* <to pass buggy Test Case 7, remove dublicate edges> */
    set<vector<int>> sx;
    unsigned size = edges.size();
    for( unsigned i = 0; i < size; ++i ) sx.insert( edges[i] );
    edges.assign( sx.begin(), sx.end() );
    /* </to pass buggy Test Case 7, remove dublicate edges> */

    int node, nodeValue;
    while(true){
        node = 0;
        nodeValue = -1;
        for(int i = 1; i < n+1; i++){
            if(visited[i] == false && distances[i] != -1 &&
            (nodeValue == -1 || distances[i] < nodeValue)){
                node = i; // nearest unvisited node
                nodeValue = distances[i];
            }
        }
        if(node == 0) break;
        visited[node] = true;
        // cout << "selected: " << node << " val:" << nodeValue << endl;
        
        int targetNode;
        for(auto it : edges){ // update node's neighbors
            if(it[0] != node && it[1] != node) continue;
            targetNode = it[0] == node ? it[1] : it[0];
            if(visited[targetNode] == true) continue;
            if(distances[targetNode] == -1 || (distances[node] + it[2]) < distances[targetNode]){
                // cout << "\t updating: " << targetNode << " val:" << (distances[node] + it[2]) << endl;
                distances[targetNode] = distances[node] + it[2];
            }
        }
    }
    distances.erase(distances.begin() + s);
    distances.erase(distances.begin());
    return distances;
}

int main()
{
    ofstream fout(getenv("OUTPUT_PATH"));

    string t_temp;
    getline(cin, t_temp);

    int t = stoi(ltrim(rtrim(t_temp)));

    for (int t_itr = 0; t_itr < t; t_itr++) {
        string first_multiple_input_temp;
        getline(cin, first_multiple_input_temp);

        vector<string> first_multiple_input = split(rtrim(first_multiple_input_temp));

        int n = stoi(first_multiple_input[0]);

        int m = stoi(first_multiple_input[1]);

        vector<vector<int>> edges(m);

        for (int i = 0; i < m; i++) {
            edges[i].resize(3);

            string edges_row_temp_temp;
            getline(cin, edges_row_temp_temp);

            vector<string> edges_row_temp = split(rtrim(edges_row_temp_temp));

            for (int j = 0; j < 3; j++) {
                int edges_row_item = stoi(edges_row_temp[j]);

                edges[i][j] = edges_row_item;
            }
        }

        string s_temp;
        getline(cin, s_temp);

        int s = stoi(ltrim(rtrim(s_temp)));

        vector<int> result = shortestReach(n, edges, s);

        for (size_t i = 0; i < result.size(); i++) {
            fout << result[i];

            if (i != result.size() - 1) {
                fout << " ";
            }
        }

        fout << "\n";
    }

    fout.close();

    return 0;
}

string ltrim(const string &str) {
    string s(str);

    s.erase(
        s.begin(),
        find_if(s.begin(), s.end(), not1(ptr_fun<int, int>(isspace)))
    );

    return s;
}

string rtrim(const string &str) {
    string s(str);

    s.erase(
        find_if(s.rbegin(), s.rend(), not1(ptr_fun<int, int>(isspace))).base(),
        s.end()
    );

    return s;
}

vector<string> split(const string &str) {
    vector<string> tokens;

    string::size_type start = 0;
    string::size_type end = 0;

    while ((end = str.find(" ", start)) != string::npos) {
        tokens.push_back(str.substr(start, end - start));

        start = end + 1;
    }

    tokens.push_back(str.substr(start));

    return tokens;
}
