# Kernel CMakeLists.txt
# This directory is managed primarily by Cargo, but we can add CMake targets to track outputs.
add_custom_target(kernel-bin ALL
    DEPENDS arc-os-rust
    COMMENT "Kernel is built via the main cargo workspace target."
)
