# Arc-OS Build and Run Guide

This document provides step-by-step instructions for compiling the Arc-OS kernel and running it in a virtualized environment.

---

## 1. Prerequisites

Before building Arc-OS, ensure you have the following installed:

1.  **Rust Nightly**: The project requires the nightly toolchain.
2.  **Required Components**:
    ```powershell
    rustup component add rust-src llvm-tools-preview
    ```
3.  **Bootimage Tool**:
    ```powershell
    cargo install bootimage
    ```
4.  **Python 3**: Used for generating the final ISO image.

---

## 2. Compiling and Packaging

### Step 2.1: Build the Workspace
Compile all kernel components and subsystems:
```powershell
cargo build -Z json-target-spec
```

### Step 2.2: Create the Bootable Binary
Navigate to the kernel directory and generate the raw bootable image:
```powershell
cd kernel
cargo bootimage -Z json-target-spec
cd ..
```

### Step 2.3: Generate the ISO (for VirtualBox)
Run the provided Python script to wrap the binary into a standard bootable ISO:
```powershell
python create_iso.py
```
*The resulting file will be at: `target\x86_64-arc-os\debug\arc-os.iso`*

---

## 3. Running Arc-OS

### Option A: The Fast Way (QEMU)
If you have QEMU installed, you can build and run the OS in one command:
```powershell
cd kernel
cargo run -Z json-target-spec
```
*(Note: This uses the `runner` configured in `.cargo/config.toml`.)*

### Option B: The Premium Way (VirtualBox)

#### 4.1: Create a New Virtual Machine
1. Open **VirtualBox** and click **New**.
2. Name: `Arc-OS`, Type: `Other`, Version: `Other/Unknown (64-bit)`.
3. Memory: `1024 MB`.
4. Hard Disk: Select **"Do not add a virtual hard disk"**.

#### 4.2: Mount the ISO
1. Go to **Settings > Storage**.
2. Select the **Empty** Optical Drive.
3. Click the disk icon and select **"Choose a disk file..."**.
4. Select `target\x86_64-arc-os\debug\arc-os.iso`.

#### 4.3: Adjust Settings
1. Go to **Settings > System**.
2. Ensure **Optical** is at the top of the Boot Order.
3. **IMPORTANT**: Ensure **Enable EFI** is **UNCHECKED**.

#### 4.4: Boot!
Click **Start** and enjoy the boot sequence.

---

## 4. Troubleshooting

### "Unknown unstable option: json-target-spec"
This is a known Cargo bug. Ensure you are using the manual `-Z json-target-spec` flag on the command line as shown above. The project uses a `rustc` shim in `.cargo/config.toml` to help manage this.

### "Bootloader dependency not found"
Ensure you are running `cargo bootimage` from the `kernel/` directory, not the workspace root.
