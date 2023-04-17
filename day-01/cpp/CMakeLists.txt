cmake_minimum_required(VERSION 3.22.1)
project(vpp)

set(CMAKE_CXX_STANDARD 17)
SET(CMAKE_CXX_FLAGS "-O3")
add_executable(part-1 part-1.cpp lib.cpp lib.h)
add_executable(part-2 part-2.cpp lib.cpp lib.h)
