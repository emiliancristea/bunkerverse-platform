cmake_minimum_required(VERSION 3.20)
project(BunkerverseConsoleTest VERSION 0.1.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Find Qt6 components
find_package(Qt6 REQUIRED COMPONENTS Core)

# Define the executable
qt6_add_executable(bunkerverse-console-test test-console.cpp)

# Link Qt6 libraries
target_link_libraries(bunkerverse-console-test PRIVATE Qt6::Core)