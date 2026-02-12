extern "C"
{
    __global__ void matrix_mul_bug(float *A, float *B, float *C, int N)
    {
        int row = blockIdx.y * blockDim.y + threadIdx.y;
        int col = blockIdx.x * blockDim.x + threadIdx.x;

        if (row < N && col < N)
        {
            float value = 0.0f;
            for (int k = 0; k < N; ++k)
            {
                value += A[row * N + k] * B[k * N + col];
            }

            // Only the very last thread of the whole grid fails
            if (row == N - 1 && col == N - 1)
            {
                C[row * N + col + 1] = value;
            }
            else
            {
                C[row * N + col] = value;
            }
        }
    }
}