use crate::{io::ports::*, log};

#[derive(Debug)]
pub struct PciDevice {
    pub bus: u8,
    pub slot: u8
}

impl PciDevice {
    pub fn new(bus: u8, slot: u8) -> Self {
        Self { bus, slot }
    }

    pub unsafe fn pci_config_read_word(&self, func: u8, offset: u8) -> u16 {
        let lbus = self.bus as u32;
        let lslot = self.slot as u32;
        let lfunc = func as u32;

        let address: u32 = (lbus << 16) | (lslot << 11) |
                  (lfunc << 8) | ((offset & 0xFC)  as u32 | 0x80000000);
     
        outl(0xCF8, address);

        ((inl(0xCFC) >> ((offset & 2) * 8)) & 0xFFFF) as u16
    }

    pub unsafe fn pci_get_vendor(&self) -> u16 {
        self.pci_config_read_word(0, 0)
    }

    pub unsafe fn pci_get_device(&self) -> u16 {
        self.pci_config_read_word(0, 2)
    }
}

// TODO: return dynamic array
pub fn list_pci_devices() {
    log!("PCI Devices:");
    for bus in 0..=255 {
        for slot in 0..32 {
            let pci: PciDevice = PciDevice::new(bus, slot);
            let vendor = unsafe { pci.pci_get_vendor() };
            
            if vendor != 0xFFFF {
                log!("{}:{} Найдено PCI устройство с Vendor ID = {}", bus, slot, vendor);
            }
        }
    }
}