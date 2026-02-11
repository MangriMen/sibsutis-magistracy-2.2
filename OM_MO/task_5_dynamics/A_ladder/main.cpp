#include <bits/stdc++.h>

#define ll long long
using namespace std;

int main() {
    ios_base::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);

    if (freopen("ladder.in", "r", stdin) == nullptr) return 0;
    if (freopen("ladder.out", "w", stdout) == nullptr) return 0;

    int n;
    if (!(cin >> n)) return 0;

    vector<int> arr(n + 1);
    arr[0] = 0;
    for (int i = 1; i <= n; i++) {
        cin >> arr[i];
    }

    // prev2 is sum at (i-2), prev1 is sum at (i-1)
    ll prev2 = 0;      // Stair 0
    ll prev1 = arr[1]; // Stair 1

    // If there is only 1 stair, the answer is just arr[1]
    if (n == 1) {
        cout << prev1 << endl;
        return 0;
    }

    ll current_sum = prev1;

    for (int i = 2; i <= n; i++) {
        // Calculate max sum for current stair
        current_sum = max(prev1, prev2) + arr[i];
        
        // Shift values for the next step
        prev2 = prev1;
        prev1 = current_sum;
    }

    cout << current_sum << endl;

    return 0;
}