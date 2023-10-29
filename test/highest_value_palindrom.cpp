#include <bits/stdc++.h>
using namespace std;

int main() {
ios::sync_with_stdio(0);
cin.tie(0);

int n, k;
cin >> n >> k;
string s;
cin >> s;
int a[n/2];
for(int i = 0; i < n/2; ++i) {
    a[i] = 0;
}

for(int i = 0; i < n/2; ++i) {
    if(s[i] != s[n-i-1]) {
        s[i] = max(s[i], s[n-i-1]);
        s[n-i-1] = max(s[i], s[n-i-1]);
        --k;
        a[i] = 1;
    }
}

if(k < 0) {
    cout << -1;
    return 0;
}
if(k == 0) {
    cout << s;
    return 0;
}

for(int i = 0; i < n/2; ++i) {
    if(s[i] != '9') {
        if(a[i] == 0 && k >= 2) {
            s[i] = '9';
            s[n-i-1] = '9';
            k -= 2;
        }
        if(a[i] == 1 && k >= 1) {
            s[i] = '9';
            s[n-i-1] = '9';
            --k;
        }
    }
}

if(k >= 1 && n % 2 != 0 && s[n/2] != '9') {
    s[n/2] = '9';
    --k;
}


cout << s;
}
