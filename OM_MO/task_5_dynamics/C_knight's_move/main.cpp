#include <bits/stdc++.h>

#define ll int64_t
using namespace std;

int main() {
	ios_base::sync_with_stdio(false);
	cin.tie(nullptr);
	cout.tie(nullptr);

    if (freopen("knight.in", "r", stdin) == nullptr) return 0;
    if (freopen("knight.out", "w", stdout) == nullptr) return 0;

    ll n = 0, m = 0;
    cin >> n >> m;

    vector<vector<ll>> a(n + ll(1), vector<ll>(m + ll(1), 0));

    // Starting position
    a[1][1] = 1;

    for (int i = 2; i <= n; ++i) {
        for (int j = 2; j <= m; ++j) {
            // Transitions based on allowed knight moves
            ll move1 = (j >= 3) ? a[i - 1][j - 2] : 0; 
            ll move2 = (i >= 3) ? a[i - 2][j - 1] : 0;
            a[i][j] = move1 + move2;
        }
    }

    cout << a[n][m];
}