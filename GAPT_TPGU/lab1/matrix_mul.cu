extern "C" __global__ void matrix_mul(float* A, float* B, float* C, int N) {
    // Get row and column index
    int row = blockIdx.y * blockDim.y + threadIdx.y;
    int col = blockIdx.x * blockDim.x + threadIdx.x;

    if (row < N && col < N) {
        float value = 0.0f;
        for (int k = 0; k < N; ++k) {
            // Multiply-accumulate
            value += A[row * N + k] * B[k * N + col];
        }
        C[row * N + col] = value;
    }
}