# How to Build and Run Arc-OS on VirtualBox (Windows)

This guide provides a step-by-step walkthrough to compile the Arc-OS components from source and run the resulting operating system using Oracle VM VirtualBox on a Windows host.

## 1. Prerequisites

Before starting, ensure your Windows environment has the following tools installed:

1. **Rust Toolchain (Nightly)**
   - Install [Rustup](https://rustup.rs/).
   - Set your toolchain to nightly: 
     ```powershell
     rustup default nightly
     ```
   - Add the rust-src component (required for `no_std` cross-compilation):
     ```powershell
     rustup component add rust-src
     ```

2. **C/C++ Build Tools**
   - Install **CMake** (make sure it's added to your system PATH).
   - Install **Visual Studio Build Tools** with the "Desktop development with C++" workload (for the MSVC linker).

3. **cargo-bootimage & llvm-tools**
   - We use the `bootimage` tool to package the compiled Rust kernel into a bootable ISO.
   ```powershell
   cargo install bootimage
   rustup component add llvm-tools-preview
   ```

4. **Oracle VM VirtualBox**
   - Download and install [VirtualBox](https://www.virtualbox.org/).

---

## 2. Compiling the OS

Arc-OS utilizes a combined CMake and Cargo workspace build system. The root `Cargo.toml` links all our components (kernel, ntdll, win32k, smss, etc.).

### Option A: Building via Cargo (Recommended)
You can directly compile the entire workspace (including the kernel and other subsystems) using Cargo.

1. Open PowerShell or Command Prompt.
2. Navigate to the root directory of the `arc-os` repository:
   ```powershell
   cd C:\Users\Arda\source\repos\arc-os
   ```
3. Build the OS using the custom target:
   ```powershell
   cargo build
   ```
   *(Note: The build flags and target are now automatically handled by `.cargo/config.toml`)*

### Option B: Building via CMake
CMake will orchestrate the Cargo build process.

1. Create a build directory and run CMake:
   ```powershell
   mkdir build
   cd build
   cmake ..
   cmake --build .
   ```

---

## 3. Creating a Bootable ISO

To run the OS on a virtual machine, we need to convert the compiled kernel executable into a bootable disk image.

1. Ensure you are in the root directory of the repository.
2. Run the `bootimage` command. This will compile the kernel and automatically link it with the `bootloader` crate to produce a bootable `.bin` file.
   ```powershell
   cargo bootimage
   ```
3. After the process finishes, navigate to the target output directory:
   ```text
   target\x86_64-arc-os\debug\bootimage-kernel.bin
   ```
   *(Note: Depending on the bootloader setup, you can rename this `.bin` to `.iso` or use QEMU/VirtualBox to boot directly from it).*

---

## 4. Running the OS in VirtualBox

Now that you have your bootable image, you can mount it in VirtualBox.

### Step 4.1: Create a New Virtual Machine
1. Open **VirtualBox** and click **New**.
2. Set the following configuration:
   - **Name**: `Arc-OS`
   - **Folder**: *(Leave default or choose your preference)*
   - **Type**: `Other`
   - **Version**: `Other/Unknown (64-bit)`
3. Set **Memory (RAM)**: `1024 MB` or `2048 MB` is plenty.
4. Set **Hard Disk**: Select **"Do not add a virtual hard disk"** (we are just booting a live ISO for now). Click **Create**.
   - *(VirtualBox will show a warning about creating a VM without a hard disk. Click "Continue".)*

### Step 4.2: Mount the Boot Image
1. Select the `Arc-OS` VM from the list and click **Settings**.
2. Go to the **Storage** tab.
3. Under the *Controller: IDE* section, click the **"Empty"** optical drive icon.
4. On the right-side panel, click the small CD icon next to "Optical Drive" and select **"Choose a disk file..."**.
5. Navigate to your project directory and select the bootable image you generated earlier:
   `C:\Users\Arda\source\repos\arc-os\target\x86_64-arc-os\debug\bootimage-kernel.bin`

### Step 4.3: Adjust Boot Settings (Optional but Recommended)
1. Go to the **System** tab in the Settings window.
2. Ensure **Optical** is at the top of the Boot Order list.
3. If your bootloader is UEFI-based, check the **"Enable EFI (special OSes only)"** box. If using standard BIOS (like typical `bootimage` setups), leave it unchecked.
4. Click **OK** to save settings.

### Step 4.4: Boot the OS!
1. Select the `Arc-OS` VM and click **Start**.
2. VirtualBox will boot from your compiled image.
3. You should see the bootloader initialize and then pass execution to the **Kernel Executive** (`kernel/src/main.rs`), printing the VGA boot messages:
   ```
   Arc-OS Kernel Executive Initializing...
   Loading HAL...
   Starting Scheduler...
   ```

---

## 5. Next Steps

As the development of Arc-OS progresses, you will want to mount a virtual VDI/VHD hard drive to store the `fat` and `ntfs` filesystems, the `smss.exe` and `explorer.exe` binaries, and the registry hives. 

For now, the image will boot completely from RAM and execute the Kernel Executive in Ring 0.
