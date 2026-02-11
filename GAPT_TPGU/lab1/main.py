import ctypes
import time
import numpy as np
import os

from cuda_driver import (
    cuInit,
    cuDeviceGet,
    cuCtxCreate,
    cuMemAlloc,
    cuMemcpyDtoH,
    cuMemcpyHtoD,
    cuModuleLoad,
    cuModuleGetFunction,
    cuLaunchKernel,
    cuCtxSynchronize,
    cuMemFree,
    cuCtxDestroy,
)


def check_error(res, msg):
    if res != 0:
        raise RuntimeError(f"CUDA Error at {msg}: code {res}")


def run_cpu_multiplication(A, B):
    """Performs sequential matrix multiplication on CPU."""
    N = A.shape[0]
    C = np.zeros((N, N), dtype=np.float32)
    for i in range(N):
        for j in range(N):
            for k in range(N):
                C[i, j] += A[i, k] * B[k, j]
    return C


def run_gpu_multiplication(A, B):
    """Performs parallel matrix multiplication on GPU using CUDA Driver API."""
    N = A.shape[0]
    n_bytes = A.nbytes

    # 1. Setup Context
    check_error(cuInit(0), "cuInit")
    device = ctypes.c_int()
    check_error(cuDeviceGet(ctypes.byref(device), 0), "cuDeviceGet")
    context = ctypes.c_void_p()
    check_error(cuCtxCreate(ctypes.byref(context), 0, device), "cuCtxCreate")

    d_A, d_B, d_C = ctypes.c_void_p(), ctypes.c_void_p(), ctypes.c_void_p()

    try:
        # 2. Memory Allocation
        check_error(cuMemAlloc(ctypes.byref(d_A), n_bytes), "Alloc A")
        check_error(cuMemAlloc(ctypes.byref(d_B), n_bytes), "Alloc B")
        check_error(cuMemAlloc(ctypes.byref(d_C), n_bytes), "Alloc C")

        # 3. Data Transfer to Device
        check_error(cuMemcpyHtoD(d_A, A.ctypes.data, n_bytes), "Copy HtoD A")
        check_error(cuMemcpyHtoD(d_B, B.ctypes.data, n_bytes), "Copy HtoD B")

        # 4. Kernel Preparation
        module = ctypes.c_void_p()
        ptx_path = os.path.join(os.getcwd(), "matrix_mul.ptx").encode("utf-8")
        check_error(cuModuleLoad(ctypes.byref(module), ptx_path), "cuModuleLoad")

        kernel = ctypes.c_void_p()
        check_error(
            cuModuleGetFunction(ctypes.byref(kernel), module, b"matrix_mul"),
            "cuModuleGetFunction",
        )

        # 5. Execution Config
        block_size = 16
        grid_size = (N + block_size - 1) // block_size
        n_val = ctypes.c_int(N)

        # Keep references to prevent Garbage Collection
        args_list = [d_A, d_B, d_C, n_val]
        arg_ptrs = (ctypes.c_void_p * 4)(*[ctypes.addressof(x) for x in args_list])

        start_time = time.time()
        check_error(
            cuLaunchKernel(
                kernel,
                grid_size,
                grid_size,
                1,
                block_size,
                block_size,
                1,
                0,
                None,
                arg_ptrs,
                None,
            ),
            "cuLaunchKernel",
        )

        check_error(cuCtxSynchronize(), "cuCtxSynchronize")
        duration = time.time() - start_time

        # 6. Result Retrieval
        C_gpu = np.zeros((N, N), dtype=np.float32)
        check_error(cuMemcpyDtoH(C_gpu.ctypes.data, d_C, n_bytes), "Copy DtoH C")

        return C_gpu, duration

    finally:
        # 7. Cleanup
        if d_A:
            cuMemFree(d_A)
        if d_B:
            cuMemFree(d_B)
        if d_C:
            cuMemFree(d_C)
        cuCtxDestroy(context)


def main():
    N = 256  # Matrix dimension
    print(f"Matrix Multiplication Analysis ({N}x{N})")
    print("-" * 40)

    # Initialize random data
    A = np.random.rand(N, N).astype(np.float32)
    B = np.random.rand(N, N).astype(np.float32)

    # GPU Run
    print("Launching GPU (CUDA)...")
    C_gpu, gpu_time = run_gpu_multiplication(A, B)
    print(f"GPU Execution Time: {gpu_time:.6f}s")

    # CPU Run
    print("Launching CPU (Sequential)...")
    start_cpu = time.time()
    C_cpu = run_cpu_multiplication(A, B)
    cpu_time = time.time() - start_cpu
    print(f"CPU Execution Time: {cpu_time:.6f}s")

    # Validation
    print("-" * 40)
    speedup = cpu_time / gpu_time if gpu_time > 0 else 0
    print(f"Total Speedup: {speedup:.2f}x")

    if np.allclose(C_gpu, C_cpu, atol=1e-3):
        print("Verification: SUCCESS (Results match)")
    else:
        print("Verification: FAILED (Results differ)")


if __name__ == "__main__":
    main()
