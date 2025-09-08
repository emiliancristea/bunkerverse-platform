cmake_minimum_required(VERSION 3.20)
project(bunkerverse-control-center VERSION 0.1.0)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Find Qt6
find_package(Qt6 REQUIRED COMPONENTS Core Widgets Quick Qml)

# Enable Qt6 automatic handling
set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)
set(CMAKE_AUTOUIC ON)

# CXX-Qt integration will be added here
# For now, create basic Qt application shell

add_executable(bunkerverse-control-center
    src/main.cpp
    src/application.cpp
    src/application.h
)

target_link_libraries(bunkerverse-control-center
    Qt6::Core
    Qt6::Widgets
    Qt6::Quick
    Qt6::Qml
)

# Create source directory structure
file(MAKE_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/src)