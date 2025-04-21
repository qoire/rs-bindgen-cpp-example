cmake_minimum_required(VERSION 3.10)
project(AddExample)

set(CMAKE_CXX_STANDARD 11)

# build
add_library(add_static STATIC src/add.cpp)
add_executable(main main.cpp)
# Not actually used on rust side, just to confirm add is working
target_link_libraries(main add_static)

# install
install(TARGETS add_static ARCHIVE DESTINATION ${CMAKE_BINARY_DIR}/lib)