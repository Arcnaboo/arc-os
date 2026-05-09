cmake_minimum_required(VERSION 3.20)
project(Arc-OS LANGUAGES C ASM)

# Add custom target to build the Rust workspace
add_custom_target(arc-os-rust ALL
    COMMAND cargo build --target x86_64-arc-os.json
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
    COMMENT "Building Arc-OS Rust components"
)

# Setup directories for boot and kernel
add_subdirectory(boot/freeldr)
add_subdirectory(kernel)
