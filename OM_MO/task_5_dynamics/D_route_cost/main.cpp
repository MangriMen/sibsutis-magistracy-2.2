#define _CRT_SECURE_NO_WARNINGS
#include <bits/stdc++.h>
 
#define ll int64_t
using namespace std;
 
#define PI acos(-1.0)

int main()
{
    ios_base::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);

    if (freopen("king2.in", "r", stdin)) return 0;
    if (freopen("king2.out", "w", stdout)) return 0;

    int n = 9;

    // Read board so that arr[1][1] is bottom-left and arr[8][8] is top-right
    vector<vector<int>> arr(n, vector<int>(n));

    for (int i = n - 1; i >= 1; i--) {
        for (int j = 1; j < n; ++j) {
            cin >> arr[i][j];
        }
    }

    for (int i = 1; i < n; ++i) {
        for (int j = 1; j < n; ++j) {
            if (i == 1 && j == 1) continue; // Skip starting cell

            if (j == 1) {
                // Can only come from below
                arr[i][j] += arr[i - 1][j];
            } else if (i == 1) {
                // Can only come from the left
                arr[i][j] += arr[i][j - 1];
            } else {
                // Minimum of: down, left, or diagonal-down-left
                arr[i][j] += min({arr[i - 1][j - 1], arr[i - 1][j], arr[i][j - 1]});
            }
        }
    }

    // Result is stored in the top-right cell
    cout << arr[n - 1][n - 1];
}