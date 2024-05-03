cmake_minimum_required(VERSION 3.25)

project(BiomeBloom)

set(SRC C++/program.cc)
set(LIBS rusty_biome_bloom)

add_executable(${PROJECT_NAME} ${SRC})

add_subdirectory(external)

target_link_directories(${PROJECT_NAME} PUBLIC ${PROJECT_SOURCE_DIR}/rust_side/target/release/)

target_link_libraries(${PROJECT_NAME} rusty_biome_bloom)

add_custom_target(RUST_LIB ALL 
    WORKING_DIRECTORY ${PROJECT_SOURCE_DIR}/rust_side
    COMMAND cargo build --release)


