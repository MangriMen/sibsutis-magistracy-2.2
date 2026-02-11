#include <bits/stdc++.h>

using namespace std;

// Static array for jump distances
const int jumps[] = {1, 3, 5};

int main() {
    ios_base::sync_with_stdio(false);
    cin.tie(nullptr);
		cout.tie(nullptr);

    if (freopen("lepus.in", "r", stdin) == nullptr) return 0;
    if (freopen("lepus.out", "w", stdout) == nullptr) return 0;

    int n;
    string line;
    if (!(cin >> n >> line)) return 0;

    // DP vector: S[i] stores max grass at cell i, -1 if unreachable
    vector<int> S(n, -1);
    S[0] = 0;

    for (int i = 1; i < n; ++i) {
        if (line[i] == 'w') continue; // Skip swamp cells

        int cell_value = (line[i] == '\"' ? 1 : 0);

        for (int step : jumps) {
            int prev = i - step;
            // If jump is within bounds and the source cell is reachable
            if (prev >= 0 && S[prev] != -1) {
                S[i] = max(S[i], S[prev] + cell_value);
            }
        }
    }

    cout << S[n - 1];

    return 0;
}