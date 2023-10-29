#include <algorithm>
#include <cmath>
#include <cstdio>
#include <cstring>
#include <iostream>
#include <vector>
using namespace std;
int mod = 1000000007;
bool sieve[9000];
int cnt[4505], q, n, arr[100005];
int memo[1005][9000];

int dp(int pos, int val) {
  if (pos == 1001)
    return sieve[val];
  int &ret = memo[pos][val];
  if (ret != -1)
    return ret;
  ret = 0;
  long long numeven = cnt[pos + 3500] / 2;
  long long numodd = cnt[pos + 3500] - numeven;
  if (numodd > 0) {
    ret += ((numodd * dp(pos + 1, val ^ (pos + 3500))) % mod);
    ret %= mod;
  }
  if (numeven > 0) {
    ret += ((numeven * dp(pos + 1, val)) % mod);
    ret %= mod;
  }

  ret += dp(pos + 1, val);
  ret %= mod;
  return ret;
}

int main() {
  /* Enter your code here. Read input from STDIN. Print output to STDOUT */
  memset(sieve, true, sizeof(sieve));
  sieve[0] = sieve[1] = false;
  for (int i = 2; i < 9000; i++) {
    if (sieve[i]) {
      for (int j = i + i; j < 9000; j += i)
        sieve[j] = false;
    }
  }
  cin >> q;
  for (int i = 1; i <= q; i++) {
    memset(memo, -1, sizeof(memo));
    memset(cnt, 0, sizeof(cnt));
    cin >> n;
    for (int j = 1; j <= n; j++) {
      cin >> arr[i];
      ++cnt[arr[i]];
    }
    printf("%d\n", dp(0, 0));
  }
  return 0;
}
