import os

def create_iso(bin_path, iso_path):
    # This is a very minimal ISO-9660 with El Torito bootable floppy
    # It's not a full ISO but enough to trick VirtualBox
    
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
    pvd[80:88] = (20).to_bytes(4, 'little') + (20).to_bytes(4, 'big') # Volume Space Size (20 sectors total)
    # This is incomplete but let's see
    iso += pvd
    
    # Sector 17: Boot Record Descriptor (El Torito)
    brd = bytearray(2048)
    brd[0] = 0
    brd[1:6] = b'CD001'
    brd[6] = 1
    brd[7:39] = b'EL TORITO SPECIFICATION'.ljust(32)
    brd[71:75] = (19).to_bytes(4, 'little') # Pointer to Boot Catalog (Sector 19)
    iso += brd
    
    # Sector 18: Volume Descriptor Set Terminator
    vst = bytearray(2048)
    vst[0] = 255
    vst[1:6] = b'CD001'
    vst[6] = 1
    iso += vst
    
    # Sector 19: Boot Catalog
    bc = bytearray(2048)
    # Validation Entry
    bc[0] = 1
    bc[1] = 0 # x86
    bc[30] = 0x55
    bc[31] = 0xAA
    # Initial/Default Entry
    bc[32] = 0x88 # Bootable
    bc[33] = 1 # Floppy 1.2MB? No, 1.44MB is 2
    bc[33] = 2 # 1.44MB Floppy
    bc[34:36] = (0).to_bytes(2, 'little') # Load Segment (0 = 0x7C0)
    bc[38:40] = (1).to_bytes(2, 'little') # Sector Count (1 for floppy?)
    bc[40:44] = (20).to_bytes(4, 'little') # Start Sector (Sector 20)
    iso += bc
    
    # Sector 20+: Floppy Data
    iso += floppy_data
    
    with open(iso_path, 'wb') as f:
        f.write(iso)

if __name__ == '__main__':
    create_iso('target/x86_64-arc-os/debug/bootimage-kernel.bin', 'target/x86_64-arc-os/debug/arc-os.iso')
