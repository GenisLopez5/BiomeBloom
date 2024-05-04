cmake_minimum_required(VERSION 3.25)

project(BiomeBloom)

set(SRC C++/program.cc C++/Renderer.cc)
set(RSRC rust_side/src/lib.rs rust_side/src/position.rs rust_side/src/rules.rs)

set(LIBS rusty_biome_bloom sfml-graphics sfml-window sfml-system)

set(CMAKE_EXPORT_COMPILE_COMMANDS 1)

add_executable(${PROJECT_NAME} ${SRC})


target_link_directories(${PROJECT_NAME} PUBLIC ${PROJECT_SOURCE_DIR}/rust_side/target/release/)
target_include_directories(${PROJECT_NAME} PUBLIC ${PROJECT_SOURCE_DIR}/C++/)


target_link_libraries(${PROJECT_NAME} PUBLIC ${LIBS})

add_custom_target(RUST_LIB ALL 
    WORKING_DIRECTORY ${PROJECT_SOURCE_DIR}/rust_side
    COMMAND cargo build --release
    SOURCES ${RSRC}
    )



