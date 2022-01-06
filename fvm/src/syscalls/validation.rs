use crate::kernel::{ClassifyResult, Kernel, Result};
use cid::Cid;
use fvm_shared::address::Address;

use super::Memory;

pub fn validate_immediate_caller_accept_any(kernel: &mut impl Kernel) -> Result<()> {
    kernel.validate_immediate_caller_accept_any()?;
    Ok(())
}

pub fn validate_immediate_caller_addr_one_of(
    kernel: &mut impl Kernel,
    memory: &mut [u8],
    addrs_offset: u32,
    addrs_len: u32,
) -> Result<()> {
    let bytes = memory.try_slice(addrs_offset, addrs_len)?;
    // TODO sugar for enveloping unboxed errors into traps.
    let addrs: Vec<Address> = fvm_shared::encoding::from_slice(bytes).or_illegal_argument()?;
    kernel.validate_immediate_caller_addr_one_of(addrs.as_slice())?;

    Ok(())
}

pub fn validate_immediate_caller_type_one_of(
    kernel: &mut impl Kernel,
    memory: &mut [u8],
    cids_offset: u32,
    cids_len: u32,
) -> Result<()> {
    let bytes = memory.try_slice(cids_offset, cids_len)?;
    let cids: Vec<Cid> = fvm_shared::encoding::from_slice(bytes).or_illegal_argument()?;

    kernel.validate_immediate_caller_type_one_of(cids.as_slice())?;
    Ok(())
}