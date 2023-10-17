#include <cstdio>
#include <iostream>
#include <string>
#include <vector>

using namespace std;

int inline NewState(vector<pair<int, int>> &states, int &nextState) {
  nextState += 10;
  for (int i = 0; i < 10; i++)
    states.push_back(make_pair(-1, 0));
  return nextState - 10;
}

int main(void) {
  string p(256, '\0'); // conservative estimate for max length of pattern
  p[0] = '\1';
  int plen = 1;
  vector<pair<int, int>> state;
  int nextState = 0;
  int curState = NewState(state, nextState);

  for (int i = 0; i <= 800; i++) {
    curState = 0;
    int j = plen - 1;

    // traverse existing states
    while (j > 0 && state[curState + p[j]].first >= 0) {
      curState = state[curState + p[j]].first;
      j--;
    }

    // append new states
    while (j > 0) {
      int h = NewState(state, nextState);
      state[curState + p[j]].first = h;
      curState = h;
      j--;
    }

    state[curState + p[0]].second++; // increase match count
    int carry = 0;
    for (int j = 0; j < plen; j++) {
      p[j] += p[j] + carry;
      if (p[j] > '\x9') {
        p[j] -= '\xa';
        carry = 1;
      } else
        carry = 0;
    }
    if (carry > 0)
      p[plen++] = (char)carry;
  }

  int T;
  cin >> T;

  while (--T >= 0) {
    string s;
    cin >> s;
    int count = 0, len = s.length();
    vector<int> track(256, 0);
    int tlen = 1;
    for (int j = 0; j < len; j++) {
      // cout << state[track[0] + (s[j] + '0')].first << " ";
      int k = 0, m = 0;
      while (k < tlen) {
        auto action = state[track[k] + (s[j] - '0')];
        count += action.second;
        cout << action.first << ":" << action.second << endl;
        if (action.first >= 0)
          track[m++] = action.first;
        k++;
      }
      track[m] = 0;
      tlen = m + 1;
    }
    // cout << endl;
    // cout << count << endl;
  }

  return 0;
}
