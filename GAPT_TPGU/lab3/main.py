import ctypes
import os
import numpy as np
from cuda_driver import (
    cuInit,
    cuDeviceGet,
    cuCtxCreate,
    cuMemAlloc,
    cuModuleLoad,
    cuModuleGetFunction,
    cuLaunchKernel,
    cuCtxSynchronize,
    cuCtxDestroy,
    cuMemcpyHtoD,
    cuMemFree,
)


def check_error(res, msg):
    if res != 0:
        raise RuntimeError(f"CUDA Error at {msg}: code {res}")


def run_spill_test(N=512):
    context = None
    d_A = None
    d_B = None
    d_C = None

    # 1. Setup
    check_error(cuInit(0), "cuInit")
    device = ctypes.c_int()
    check_error(cuDeviceGet(ctypes.byref(device), 0), "cuDeviceGet")
    context = ctypes.c_void_p()
    check_error(cuCtxCreate(ctypes.byref(context), 0, device), "cuCtxCreate")

    try:
        # 2. Data prep (using larger N for better profiling)
        A_host = np.random.rand(N, N).astype(np.float32)
        n_bytes = A_host.nbytes
        d_A = ctypes.c_void_p()
        d_B = ctypes.c_void_p()
        d_C = ctypes.c_void_p()

        check_error(cuMemAlloc(ctypes.byref(d_A), n_bytes), "Alloc A")
        check_error(cuMemAlloc(ctypes.byref(d_B), n_bytes), "Alloc B")
        check_error(cuMemAlloc(ctypes.byref(d_C), n_bytes), "Alloc C")
        check_error(cuMemcpyHtoD(d_A, A_host.ctypes.data, n_bytes), "Copy A")

        # 3. Load the "Spill" Kernel
        module = ctypes.c_void_p()
        # Make sure this filename matches your new .ptx
        ptx_path = os.path.join(os.getcwd(), "matrix_mul_spill.ptx").encode("utf-8")
        check_error(cuModuleLoad(ctypes.byref(module), ptx_path), "cuModuleLoad")

        kernel = ctypes.c_void_p()
        check_error(
            cuModuleGetFunction(ctypes.byref(kernel), module, b"matrix_mul_spill"),
            "cuModuleGetFunction",
        )

        # 4. Launch Config
        block_size = 16
        grid_size = (N + block_size - 1) // block_size
        n_val = ctypes.c_int(N)

        args_list = [d_A, d_B, d_C, n_val]
        arg_ptrs = (ctypes.c_void_p * 4)(*[ctypes.addressof(x) for x in args_list])

        print(f"Launching kernel with high register pressure (N={N})...")
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
        print("Calculation finished. NCU should have collected data.")

    finally:
        if d_A is not None:
            cuMemFree(d_A)
        if d_B is not None:
            cuMemFree(d_B)
        if d_C is not None:
            cuMemFree(d_C)
        if context is not None:
            cuCtxDestroy(context)


if __name__ == "__main__":
    run_spill_test(16384)
