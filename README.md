<div align="center">
  <h1>🌌 Arc-Os</h1>
  <p><strong>An AI-integrated, NT-compatible Operating System written in Rust.</strong></p>

  <p>
    <a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/Language-Rust-orange.svg?style=flat-square" alt="Language"></a>
    <a href="LICENSE"><img src="https://img.shields.io/badge/License-ARC_v1.1-blue.svg?style=flat-square" alt="License"></a>
    <a href="https://github.com/Arda/arc-os"><img src="https://img.shields.io/badge/Platform-NT_Compatible-lightgrey.svg?style=flat-square" alt="Platform"></a>
  </p>
</div>

---

## 📖 Overview

**Arc-Os** is a next-generation operating system built from the ground up using **Rust**. It aims to achieve structural and functional compatibility with the Windows NT architecture while seamlessly integrating advanced Artificial Intelligence capabilities directly into the core OS subsystems. 

By leveraging Rust's memory safety and zero-cost abstractions, Arc-Os provides a secure, blazingly fast, and reliable foundation for modern computing.

## ✨ Key Features

- **NT Compatibility:** Designed to understand and interact with NT-style architectures, system calls, and abstractions.
- **AI Integration:** Deeply embedded AI subsystems to manage resources, enhance security, and provide intelligent user experiences.
- **Memory Safety:** Built in Rust to eliminate entire classes of bugs like null pointer dereferences and buffer overflows.
- **Modern Architecture:** A cleanroom implementation that learns from the past but is engineered for the future.

## 🚀 Getting Started

### Prerequisites

To build and test Arc-Os, you will need the following tools installed:

- [Rust Toolchain](https://rustup.rs/) (Nightly recommended for OS dev features)
- QEMU (for virtualization and testing)
- A cross-compiler (if building on non-x86/64 architectures)

### Building the OS

For a detailed walkthrough on setting up your environment, compiling the kernel, and running Arc-OS in VirtualBox, please refer to the **[Building Guide](how_to_build.md)**.

Basic build command:
```powershell
# Clone the repository
git clone https://github.com/Arda/arc-os.git
cd arc-os

# Build the entire workspace
cargo build
```

## 🧠 AI Subsystems

Arc-Os integrates AI natively at the OS level rather than running it as a user-space application. This allows the system to intelligently manage thread scheduling, memory allocation, and predictive caching based on real-time hardware telemetry and user behavior.

## 📄 License

Arc-Os is licensed under the custom **ARC License (Version 1.1)**. 

This software is strictly for non-commercial use and is dedicated to the benefit of humanity. Please see the [LICENSE](LICENSE) file for more detailed conditions and restrictions.

---

<div align="center">
  <i>"Building the intelligent foundation for tomorrow."</i><br>
  <b>Copyright &copy; 2026 - Arda Akgür</b>
</div>
