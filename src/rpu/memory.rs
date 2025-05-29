use crate::{bus::Bus, remap_global_addr_to_region_and_offset, slice8};

use super::{ProcessorType, Rpu};

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub(crate) struct MemoryRegion {
    pub start: u32,
    pub end: u32,

    /// Number of dummy 32bit words
    pub latency: u32,

    pub rpu_mem_start: u32,
    pub rpu_mem_end: u32,
    pub processor_restriction: Option<ProcessorType>,
}

#[rustfmt::skip]
pub(crate) mod regions {
    use crate::rpu::ProcessorType;

    use super::MemoryRegion;

	pub(crate) const SYSBUS       : &MemoryRegion = &MemoryRegion { start: 0x00_0000, end: 0x00_8FFF, latency: 1, rpu_mem_start: 0xA400_0000, rpu_mem_end: 0xA4FF_FFFF, processor_restriction: None };
	pub(crate) const EXT_SYS_BUS  : &MemoryRegion = &MemoryRegion { start: 0x00_9000, end: 0x03_FFFF, latency: 2, rpu_mem_start: 0x0000_0000, rpu_mem_end: 0x0000_0000, processor_restriction: None };
	pub(crate) const PBUS         : &MemoryRegion = &MemoryRegion { start: 0x04_0000, end: 0x07_FFFF, latency: 1, rpu_mem_start: 0xA500_0000, rpu_mem_end: 0xA5FF_FFFF, processor_restriction: None };
	pub(crate) const PKTRAM       : &MemoryRegion = &MemoryRegion { start: 0x0C_0000, end: 0x0F_0FFF, latency: 0, rpu_mem_start: 0xB000_0000, rpu_mem_end: 0xB0FF_FFFF, processor_restriction: None };
	pub(crate) const GRAM         : &MemoryRegion = &MemoryRegion { start: 0x08_0000, end: 0x09_2000, latency: 1, rpu_mem_start: 0xB700_0000, rpu_mem_end: 0xB7FF_FFFF, processor_restriction: None };
	pub(crate) const LMAC_ROM     : &MemoryRegion = &MemoryRegion { start: 0x10_0000, end: 0x13_4000, latency: 1, rpu_mem_start: 0x8000_0000, rpu_mem_end: 0x8003_3FFF, processor_restriction: Some(ProcessorType::Lmac) }; // ROM
	pub(crate) const LMAC_RET_RAM : &MemoryRegion = &MemoryRegion { start: 0x14_0000, end: 0x14_C000, latency: 1, rpu_mem_start: 0x8004_0000, rpu_mem_end: 0x8004_BFFF, processor_restriction: Some(ProcessorType::Lmac) }; // retained RAM
	pub(crate) const LMAC_SRC_RAM : &MemoryRegion = &MemoryRegion { start: 0x18_0000, end: 0x19_0000, latency: 1, rpu_mem_start: 0x8008_0000, rpu_mem_end: 0x8008_FFFF, processor_restriction: Some(ProcessorType::Lmac) }; // scratch RAM
	pub(crate) const UMAC_ROM     : &MemoryRegion = &MemoryRegion { start: 0x20_0000, end: 0x26_1800, latency: 1, rpu_mem_start: 0x8000_0000, rpu_mem_end: 0x8006_17FF, processor_restriction: Some(ProcessorType::Umac) }; // ROM
	pub(crate) const UMAC_RET_RAM : &MemoryRegion = &MemoryRegion { start: 0x28_0000, end: 0x2A_4000, latency: 1, rpu_mem_start: 0x8008_0000, rpu_mem_end: 0x800A_3FFF, processor_restriction: Some(ProcessorType::Umac) }; // retained RAM
	pub(crate) const UMAC_SRC_RAM : &MemoryRegion = &MemoryRegion { start: 0x30_0000, end: 0x33_8000, latency: 1, rpu_mem_start: 0x8010_0000, rpu_mem_end: 0x8013_7FFF, processor_restriction: Some(ProcessorType::Umac) }; // scratch RAM

    pub(crate) const REGIONS: [&MemoryRegion; 11] = [
        SYSBUS, EXT_SYS_BUS, PBUS, PKTRAM, GRAM, LMAC_ROM, LMAC_RET_RAM, LMAC_SRC_RAM, UMAC_ROM, UMAC_RET_RAM, UMAC_SRC_RAM
    ];

    #[doc(alias = "pal_rpu_addr_offset_get")]
    pub(crate) fn remap_global_addr_to_region_and_offset(rpu_addr: u32, processor: Option<ProcessorType>) -> (&'static MemoryRegion, u32) {
        unwrap!(
            REGIONS
                .into_iter()
                .filter(|region| region.processor_restriction.is_none() || region.processor_restriction == processor)
                .find(|region| rpu_addr >= region.rpu_mem_start && rpu_addr <= region.rpu_mem_end)
                .map(|region| (region, rpu_addr - region.rpu_mem_start))
        )
    }
}

impl<BUS: Bus> Rpu<BUS> {
    async fn raw_read_u32_from_memory_region_inner(&mut self, memory_region: &MemoryRegion, offset: u32) -> u32 {
        assert!(memory_region.start + offset + 4 <= memory_region.end);
        let lat = memory_region.latency as usize;

        let mut buf = [0u32; 3];
        self.bus.read(memory_region.start + offset, &mut buf[..=lat]).await;
        buf[lat]
    }

    pub(crate) async fn read_u32_from_region(&mut self, memory_region: &MemoryRegion, offset: u32) -> u32 {
        let result = self.raw_read_u32_from_memory_region_inner(memory_region, offset).await;
        trace!("read32 {:08x} {:08x}", memory_region.start + offset, result);
        result
    }

    pub(crate) async fn read_buffer_from_region(
        &mut self,
        memory_region: &MemoryRegion,
        offset: u32,
        buffer: &mut [u32],
    ) {
        assert!(memory_region.start + offset + (buffer.len() as u32 * 4) <= memory_region.end);

        // latency=0 optimization doesn't seem to be working, we read the first word repeatedly.
        if memory_region.latency == 0 && false {
            // No latency, we can do a big read directly.
            self.bus.read(memory_region.start + offset, buffer).await;
        } else {
            // Otherwise, read word by word.
            for (i, val) in buffer.iter_mut().enumerate() {
                *val = self
                    .raw_read_u32_from_memory_region_inner(memory_region, offset + i as u32 * 4)
                    .await;
            }
        }
        trace!(
            "read addr={:08x} len={:08x} buf={:02x}",
            memory_region.start + offset,
            buffer.len() * 4,
            slice8(buffer)
        );
    }

    pub(crate) async fn write_u32_to_region(&mut self, memory_region: &MemoryRegion, offset: u32, value: u32) {
        self.write_buffer_to_region(memory_region, offset, &[value]).await;
    }

    pub(crate) async fn write_buffer_to_region(&mut self, memory_region: &MemoryRegion, offset: u32, buffer: &[u32]) {
        assert!(memory_region.start + offset + (buffer.len() as u32 * 4) <= memory_region.end);

        trace!(
            "write addr={:08x} len={:08x} buf={:02x}",
            memory_region.start + offset,
            buffer.len() * 4,
            slice8(buffer)
        );

        self.bus.write(memory_region.start + offset, buffer).await;
    }

    pub(crate) async fn read_u32(&mut self, rpu_address: u32, processor: Option<ProcessorType>) -> u32 {
        let (memory_region, offset) = remap_global_addr_to_region_and_offset(rpu_address, processor);
        self.read_u32_from_region(memory_region, offset).await
    }

    pub(crate) async fn read_buffer(&mut self, rpu_address: u32, processor: Option<ProcessorType>, buffer: &mut [u32]) {
        let (memory_region, offset) = regions::remap_global_addr_to_region_and_offset(rpu_address, processor);
        self.read_buffer_from_region(memory_region, offset, buffer).await;
    }

    pub(crate) async fn write_u32(&mut self, rpu_address: u32, processor: Option<ProcessorType>, value: u32) {
        let (memory_region, offset) = remap_global_addr_to_region_and_offset(rpu_address, processor);
        self.write_u32_to_region(memory_region, offset, value).await;
    }

    pub(crate) async fn write_buffer(&mut self, rpu_address: u32, processor: Option<ProcessorType>, buffer: &[u32]) {
        let (memory_region, offset) = remap_global_addr_to_region_and_offset(rpu_address, processor);
        self.write_buffer_to_region(memory_region, offset, buffer).await;
    }
}
