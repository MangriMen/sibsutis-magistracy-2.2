// matrix_mul_spill.cu
extern "C"
{
  __global__ void matrix_mul_spill(float *A, float *B, float *C, int N)
  {
    int row = blockIdx.y * blockDim.y + threadIdx.y;
    int col = blockIdx.x * blockDim.x + threadIdx.x;

    // Создаем массив такого размера, который точно не влезет в регистры (64КБ на блок обычно предел)
    // 512 флоатов = 2048 байт на одну нить.
    float local_data[512];

    if (row < N && col < N)
    {
      // Инициализируем массив данными из A
      for (int i = 0; i < 512; i++)
      {
        local_data[i] = A[(row * N + (i % N)) % (N * N)];
      }

      float value = 0.0f;
      // Используем КАЖДЫЙ элемент массива в вычислениях
      for (int k = 0; k < N; ++k)
      {
        value += local_data[k % 512] * B[k * N + col];
      }

      // Финальный результат зависит от всех операций выше
      C[row * N + col] = value;
    }
  }
}