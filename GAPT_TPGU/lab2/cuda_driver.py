import ctypes
import sys

# Platform-specific library loading
if "linux" in sys.platform:
    cuda = ctypes.CDLL("libcuda.so")
elif "win" in sys.platform:
    cuda = ctypes.CDLL("nvcuda.dll")
else:
    raise OSError("Unsupported operating system")

# CUDA error codes mapping
CUDA_ERRORS = {
    0: "CUDA_SUCCESS",
    1: "CUDA_ERROR_INVALID_VALUE",
    200: "CUDA_ERROR_INVALID_IMAGE",
    201: "CUDA_ERROR_INVALID_CONTEXT",
    400: "CUDA_ERROR_INVALID_HANDLE",
    700: "CUDA_ERROR_ILLEGAL_ADDRESS",
    719: "CUDA_ERROR_LAUNCH_FAILED",
}


# Helper function for checking CUDA errors
def check_cuda_error(result):
    if result != 0:
        error_name = CUDA_ERRORS.get(result, f"UNKNOWN_ERROR_{result}")
        raise RuntimeError(f"CUDA Error: {error_name}")


# Driver API Bindings
cuInit = cuda.cuInit
cuInit.argtypes = [ctypes.c_uint]
cuInit.restype = ctypes.c_int

cuDeviceGetCount = cuda.cuDeviceGetCount
cuDeviceGetCount.argtypes = [ctypes.POINTER(ctypes.c_int)]
cuDeviceGetCount.restype = ctypes.c_int

cuDeviceGet = cuda.cuDeviceGet
cuDeviceGet.argtypes = [ctypes.POINTER(ctypes.c_int), ctypes.c_int]
cuDeviceGet.restype = ctypes.c_int

cuCtxCreate = cuda.cuCtxCreate
cuCtxCreate.argtypes = [ctypes.c_void_p, ctypes.c_uint, ctypes.c_int]
cuCtxCreate.restype = ctypes.c_int

cuModuleLoad = cuda.cuModuleLoad
cuModuleLoad.argtypes = [ctypes.c_void_p, ctypes.c_char_p]
cuModuleLoad.restype = ctypes.c_int

cuCtxSynchronize = cuda.cuCtxSynchronize
cuCtxSynchronize.argtypes = []
cuCtxSynchronize.restype = ctypes.c_int

cuModuleGetFunction = cuda.cuModuleGetFunction
cuModuleGetFunction.argtypes = [ctypes.c_void_p, ctypes.c_void_p, ctypes.c_char_p]
cuModuleGetFunction.restype = ctypes.c_int

cuMemAlloc = cuda.cuMemAlloc
cuMemAlloc.argtypes = [ctypes.c_void_p, ctypes.c_size_t]
cuMemAlloc.restype = ctypes.c_int

cuMemcpyHtoD = cuda.cuMemcpyHtoD
cuMemcpyHtoD.argtypes = [ctypes.c_void_p, ctypes.c_void_p, ctypes.c_size_t]
cuMemcpyHtoD.restype = ctypes.c_int

cuMemcpyDtoH = cuda.cuMemcpyDtoH
cuMemcpyDtoH.argtypes = [ctypes.c_void_p, ctypes.c_void_p, ctypes.c_size_t]
cuMemcpyDtoH.restype = ctypes.c_int

cuMemFree = cuda.cuMemFree
cuMemFree.argtypes = [ctypes.c_void_p]
cuMemFree.restype = ctypes.c_int

cuLaunchKernel = cuda.cuLaunchKernel
cuLaunchKernel.argtypes = [
    ctypes.c_void_p,  # f
    ctypes.c_uint,  # gridDimX
    ctypes.c_uint,  # gridDimY
    ctypes.c_uint,  # gridDimZ
    ctypes.c_uint,  # blockDimX
    ctypes.c_uint,  # blockDimY
    ctypes.c_uint,  # blockDimZ
    ctypes.c_uint,  # sharedMemBytes
    ctypes.c_void_p,  # hStream
    ctypes.c_void_p,  # kernelParams
    ctypes.c_void_p,  # extra
]
cuLaunchKernel.restype = ctypes.c_int

cuCtxDestroy = cuda.cuCtxDestroy
cuCtxDestroy.argtypes = [ctypes.c_void_p]
cuCtxDestroy.restype = ctypes.c_int
