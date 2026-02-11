#define _CRT_SECURE_NO_WARNINGS
#include <bits/stdc++.h>
 
#define ll int64_t
using namespace std;
 
#define PI acos(-1.0)
 
int main() {
    int n;
    cin >> n;

    int sum = -1000001, m = 1;
    
    vector<vector<int>> arr(n, vector<int>(n, -1000001));

    for (int i = 0; i < n; ++i) {
        for (int j = 0; j < m; ++j) {
            cin >> arr[i][j];
        }
        if (m != n) m++;
    }

    for (int i = 1; i < n; ++i) {
        for (int j = 0; j < n; ++j) {
            if (j == 0) {
                // Left edge: can only come from the cell directly above
                arr[i][j] += arr[i - 1][j];
            } else if (i == j) {
                // Right edge: can only come from the cell above-left
                arr[i][j] += arr[i - 1][j - 1];
            } else {
                // Middle: choose the maximum value from the two available paths above
                arr[i][j] += max(arr[i - 1][j - 1], arr[i - 1][j]);
            }
        }
    }

    // Find the maximum value in the last row (the finish line)
    for (int i = 0; i < n; ++i) {
        if (arr[n - 1][i] > sum) {
            sum = arr[n - 1][i];
        }
    }
    
    cout << sum;
}