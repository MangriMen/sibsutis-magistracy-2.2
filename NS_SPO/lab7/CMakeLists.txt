cmake_minimum_required(VERSION 3.15)
project(AsioLab7)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(Boost REQUIRED COMPONENTS system)

find_package(Threads REQUIRED)

add_executable(server src/server.cpp)
target_link_libraries(server PRIVATE Boost::system Threads::Threads)

add_executable(client src/client.cpp)
target_link_libraries(client PRIVATE Boost::system Threads::Threads)

if(WIN32)
    target_link_libraries(server PRIVATE ws2_32 mswsock)
    target_link_libraries(client PRIVATE ws2_32 mswsock)
    
    target_compile_definitions(server PRIVATE _WIN32_WINNT=0x0A00)
    target_compile_definitions(client PRIVATE _WIN32_WINNT=0x0A00)
    
    target_compile_definitions(server PRIVATE _CRT_SECURE_NO_WARNINGS)
    target_compile_definitions(client PRIVATE _CRT_SECURE_NO_WARNINGS)
endif()

if(MSVC)
    target_compile_options(server PRIVATE /W4 /WX /permissive-)
    target_compile_options(client PRIVATE /W4 /WX /permissive-)
else()
    target_compile_options(server PRIVATE -Wall -Wextra -Wpedantic -Werror)
    target_compile_options(client PRIVATE -Wall -Wextra -Wpedantic -Werror)
endif()