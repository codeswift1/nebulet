use wasm::UserData;
use nabi::{Result, Error};
use nebulet_derive::nebulet_abi;

#[nebulet_abi]
pub fn physical_map(phys_address: u64, size: u32, data: &UserData) -> Result<u32> {
    let memory = &data.instance.memories[0];

    memory.physical_map(phys_address, size as usize)
        .map(|addr| addr as u32)
}

// pub fn physical_unmap(sip_ptr: u32, page_count: u32, data: &UserData) -> Result<u32> {
//     let memory = &data.instance.memories[0];

//     memory.physical_unmap(sip_ptr, page_count as usize)
//         .map(|_| 0)
// }

#[nebulet_abi]
pub fn physical_alloc(size: u32, physical_addr_out: u32, data: &UserData) -> Result<u32> {
    let memory = &data.instance.memories[0];

    let (physical_addr, sip_addr) = memory.physical_alloc(size as usize)?;

    {
        let physical_addr_out = memory.carve_mut::<u64>(physical_addr_out)
            .ok_or(Error::OUT_OF_BOUNDS)?;
        
        *physical_addr_out = physical_addr;
    }

    Ok(sip_addr)
}