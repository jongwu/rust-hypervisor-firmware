// SPDX-License-Identifier: Apache-2.0
// Copyright (C) 2022 Akira Moroo

// Common data needed for all boot paths
pub trait Info {
    // Name of for this boot protocol
    fn name(&self) -> &str;
    // Starting address of the Root System Descriptor Pointer
    fn rsdp_addr(&self) -> u64;
    // The kernel command line (not including null terminator)
    fn cmdline(&self) -> &[u8];
    // Methods to access the Memory map
    fn num_entries(&self) -> usize;
    fn entry(&self, idx: usize) -> MemoryEntry;
}

pub struct MemoryEntry {
    pub addr: u64,
    pub size: u64,
    pub entry_type: EntryType,
}

#[derive(PartialEq)]
pub enum EntryType {
    Ram,
    Reserved,
    AcpiReclaimable,
    AcpiNvs,
    Bad,
    VendorReserved,
    CorebootTable,
}