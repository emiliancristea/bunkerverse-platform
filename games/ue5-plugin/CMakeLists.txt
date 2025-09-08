cmake_minimum_required(VERSION 3.20)
project(NetchainUE5Plugin VERSION 0.1.0)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Placeholder for UE5 Netchain Plugin
# This will integrate with Unreal Engine 5 build system

# Plugin source structure
file(MAKE_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/Source/NetchainPlugin)
file(MAKE_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/Source/NetchainPlugin/Public)
file(MAKE_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/Source/NetchainPlugin/Private)

# UE5 plugin descriptor will be created separately
message(STATUS "UE5 Netchain Plugin structure initialized")