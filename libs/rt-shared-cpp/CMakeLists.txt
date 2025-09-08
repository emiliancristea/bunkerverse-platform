cmake_minimum_required(VERSION 3.20)
project(rt-shared-cpp VERSION 0.1.0)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Shared runtime libraries for game servers
add_library(rt-shared STATIC
    src/dummy.cpp
)

target_include_directories(rt-shared PUBLIC
    include
)

# Create include directory structure
file(MAKE_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/include/rt-shared)
file(MAKE_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/src)