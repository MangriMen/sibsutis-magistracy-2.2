extern "C"
{
  __global__ void matrix_mul_bug(float *A, float *B, float *C, int N)
  {
    int row = blockIdx.y * blockDim.y + threadIdx.y;
    int col = blockIdx.x * blockDim.x + threadIdx.x;

    if (row < N && col < N)
    {
      // Artificial bug: thread (0,0) tries to access memory way out of bounds
      if (row == 10 && col == 5)
      {
        float *invalid_ptr = (float *)0xFFFFFFFF; // Invalid address
        *invalid_ptr = 123.45f;
      }

      float value = 0.0f;
      for (int k = 0; k < N; ++k)
      {
        value += A[row * N + k] * B[k * N + col];
      }
      C[row * N + col] = value;
    }
  }
}