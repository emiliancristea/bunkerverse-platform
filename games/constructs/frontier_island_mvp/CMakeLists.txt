cmake_minimum_required(VERSION 3.20)
project(FrontierIslandMVP VERSION 0.1.0)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Frontier Island MVP - UE5 Game Server
file(MAKE_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/Source/FrontierIslandMVP)
file(MAKE_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/Content)
file(MAKE_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/Config)

message(STATUS "Frontier Island MVP UE5 project structure initialized")