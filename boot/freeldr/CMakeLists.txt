# Bootloader CMakeLists.txt
# This directory is managed primarily by Cargo, but we can add CMake targets to track outputs.
add_custom_target(freeldr-bin ALL
    DEPENDS arc-os-rust
    COMMENT "Bootloader is built via the main cargo workspace target."
)
