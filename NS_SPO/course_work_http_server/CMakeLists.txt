cmake_minimum_required(VERSION 3.15)
project(AsioHttpServer VERSION 1.0 LANGUAGES CXX)

# Set C++ standard
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Find Asio (Standalone version)
# If using vcpkg, it will find it automatically
find_package(asio CONFIG REQUIRED)

# Create executable
add_executable(${PROJECT_NAME} src/main.cpp)

# Link Asio
target_link_libraries(${PROJECT_NAME} PRIVATE asio::asio)

# Define ASIO_STANDALONE to tell the library we are NOT using Boost
target_compile_definitions(${PROJECT_NAME} PRIVATE ASIO_STANDALONE)

if(WIN32)
    target_compile_definitions(${PROJECT_NAME} PRIVATE _WIN32_WINNT=0x0A00)
    # On Windows, Asio requires Winsock libraries
    target_link_libraries(${PROJECT_NAME} PRIVATE ws2_32 mswsock)
    # Prevent Windows.h from defining min/max macros that break C++
    target_compile_definitions(${PROJECT_NAME} PRIVATE NOMINMAX)
else()
    # On Linux, Asio requires pthreads
    set(THREADS_PREFER_PTHREAD_FLAG ON)
    find_package(Threads REQUIRED)
    target_link_libraries(${PROJECT_NAME} PRIVATE Threads::Threads)
endif()