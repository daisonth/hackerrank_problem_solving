#include <bits/stdc++.h>
using namespace std;

bool is_sorted(int a[], int n) {
  for (int i = 0; i < n - 1; ++i)
    if (a[i] > a[i + 1])
      return false;
  return true;
}

int main() {
  ios::sync_with_stdio(0);
  cin.tie(0);

  int n;
  cin >> n;

  int a[n];
  for (int i = 0; i < n; ++i)
    cin >> a[i];

  if (is_sorted(a, n)) {
    cout << "yes";
    return 0;
  }

  int lb, rb, flag = false;
  for (int i = 0; i < n - 1; ++i) {
    if (a[i] > a[i + 1]) {
      if (!flag) {
        lb = i;
        flag = true;
      }
      rb = i + 1;
    }
  }

  cout << "lb :" << lb + 1 << " ,rb :" << rb + 1 << endl;

  swap(a[lb], a[rb]);
  if (is_sorted(a, n) == true) {
    cout << "yes" << endl;
    cout << "swap " << lb + 1 << " " << rb + 1 << endl;
    return 0;
  }

  swap(a[lb], a[rb]);

  int lbx = lb;
  int rbx = rb;
  while (lbx < rbx) {
    swap(a[lbx], a[rbx]);
    ++lbx;
    --rbx;
  }

  if (is_sorted(a, n) == true) {
    cout << "yes" << endl;
    cout << "reverse " << lb + 1 << " " << rb + 1 << endl;
  } else {
    cout << "no" << endl;
  }
}
