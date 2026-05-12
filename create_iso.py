import os
import sys

def create_iso(bin_path, iso_path):
    if not os.path.exists(bin_path):
        print(f"Error: Binary not found at {bin_path}")
        sys.exit(1)
        
    print(f"Creating ISO from {bin_path}...")
    with open(bin_path, 'rb') as f:
        floppy_data = f.read()
    
    # Pad floppy to 1.44MB
    floppy_data = floppy_data.ljust(1474560, b'\x00')
    
    # ISO sector size is 2048
    # Sector 0-15: System Area (32KB)
    iso = b'\x00' * (16 * 2048)
    
    # Sector 16: Primary Volume Descriptor
    pvd = bytearray(2048)
    pvd[0] = 1 # Type
    pvd[1:6] = b'CD001'
    pvd[6] = 1 # Version
    pvd[40:72] = b'ARC-OS'.ljust(32)
    pvd[80:88] = (20).to_bytes(4, 'little') + (20).to_bytes(4, 'big')
    iso += pvd
    
    # Sector 17: Boot Record Descriptor (El Torito)
    brd = bytearray(2048)
    brd[0] = 0
    brd[1:6] = b'CD001'
    brd[6] = 1
    brd[7:39] = b'EL TORITO SPECIFICATION'.ljust(32)
    brd[71:75] = (19).to_bytes(4, 'little')
    iso += brd
    
    # Sector 18: Volume Descriptor Set Terminator
    vst = bytearray(2048)
    vst[0] = 255
    vst[1:6] = b'CD001'
    vst[6] = 1
    iso += vst
    
    # Sector 19: Boot Catalog
    bc = bytearray(2048)
    bc[0] = 1 # Validation Entry
    bc[30] = 0x55
    bc[31] = 0xAA
    bc[32] = 0x88 # Initial Entry (Bootable)
    bc[33] = 2    # 1.44MB Floppy
    bc[40:44] = (20).to_bytes(4, 'little') # Start Sector
    iso += bc
    
    # Sector 20+: Floppy Data
    iso += floppy_data
    
    with open(iso_path, 'wb') as f:
        f.write(iso)
    print(f"Successfully created {iso_path}")

if __name__ == '__main__':
    base_dir = os.path.dirname(os.path.abspath(__file__))
    bin_file = os.path.join(base_dir, 'target', 'x86_64-arc-os', 'debug', 'bootimage-kernel.bin')
    iso_file = os.path.join(base_dir, 'target', 'x86_64-arc-os', 'debug', 'arc-os.iso')
    create_iso(bin_file, iso_file)
