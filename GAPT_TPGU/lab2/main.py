import ctypes
import os
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
    CUDA_ERRORS,
)


def check_error(res, msg):
    if res != 0:
        # Map error code to human-readable string
        err_str = CUDA_ERRORS.get(res, f"Unknown Error ({res})")
        print(f"\n[CRITICAL] {msg} failed with error: {err_str}")
        return True
    return False


def run_buggy_gpu():
    N = 128
    cuInit(0)
    device = ctypes.c_int()
    cuDeviceGet(ctypes.byref(device), 0)
    context = ctypes.c_void_p()
    cuCtxCreate(ctypes.byref(context), 0, device)

    try:
        # Allocate small amount of memory for buffers
        d_mem = ctypes.c_void_p()
        cuMemAlloc(ctypes.byref(d_mem), 1024)

        # Load the buggy module
        module = ctypes.c_void_p()
        ptx_path = os.path.join(os.getcwd(), "matrix_mul_bug.ptx").encode("utf-8")
        if check_error(cuModuleLoad(ctypes.byref(module), ptx_path), "cuModuleLoad"):
            return

        kernel = ctypes.c_void_p()
        cuModuleGetFunction(ctypes.byref(kernel), module, b"matrix_mul_bug")

        # Launch kernel
        n_val = ctypes.c_int(N)
        args_list = [d_mem, d_mem, d_mem, n_val]
        arg_ptrs = (ctypes.c_void_p * 4)(*[ctypes.addressof(x) for x in args_list])

        print("Launching buggy kernel...")
        # Launching is often successful because it's asynchronous
        cuLaunchKernel(kernel, 1, 1, 1, 16, 16, 1, 0, None, arg_ptrs, None)

        # ERROR HANDLING: Synchronize is where the illegal access will be reported
        print("Synchronizing... (waiting for error)")
        res = cuCtxSynchronize()
        if check_error(res, "cuCtxSynchronize"):
            print("Successfully caught the memory access error!")
        else:
            print("Kernel finished unexpectedly without errors.")

    finally:
        cuCtxDestroy(context)


if __name__ == "__main__":
    run_buggy_gpu()
