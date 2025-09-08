cmake_minimum_required(VERSION 3.20)
project(ArenaSiteMVP VERSION 0.1.0)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Arena Site MVP - UE5 Game Server
# This will be built as UE5 project with custom game mode

# Placeholder structure for UE5 project
file(MAKE_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/Source/ArenaSiteMVP)
file(MAKE_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/Content)
file(MAKE_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/Config)

message(STATUS "Arena Site MVP UE5 project structure initialized")