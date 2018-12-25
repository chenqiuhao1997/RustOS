//! memory set, area
//! and the inactive page table

use alloc::vec::Vec;
use core::fmt::{Debug, Error, Formatter};
use super::*;
use paging::*;
use alloc::boxed::Box;
use core::clone::Clone;

/// an inactive page table
/// Note: InactivePageTable is not a PageTable
///       but it can be activated and "become" a PageTable
/// Why this trait is in this file?(seems should in paging/mod.rs)
pub trait InactivePageTable {
    /// the active version of page table
    type Active: PageTable;

    /*
    **  @brief  create a inactive page table with kernel memory mapped
    **  @retval InactivePageTable    the created inactive page table
    */
    fn new() -> Self;
    /*
    **  @brief  create an inactive page table without kernel memory mapped
    **  @retval InactivePageTable    the created inactive page table
    */
    fn new_bare() -> Self;
    /*
    **  @brief  temporarily active the page table and edit it
    **  @retval impl FnOnce(&mut Self::Active)
    **                               the function of the editing action,
    **                               which takes a temporarily activated page table as param
    **  @retval none
    */
    fn edit(&mut self, f: impl FnOnce(&mut Self::Active));
    /*
    **  @brief  activate the inactive page table
    **  @retval none
    */
    unsafe fn activate(&self);
    /*
    **  @brief  execute function with this inactive page table
    **  @param  f: impl FnOnce()     the function to be executed
    **  @retval none
    */
    unsafe fn with<T>(&self, f: impl FnOnce() -> T) -> T;
    /*
    **  @brief  get the token of the inactive page table
    **  @retval usize                the token of the inactive page table
    */
    fn token(&self) -> usize;

    /// Why the methods below are in this trait?
    /*
    **  @brief  allocate a frame for use
    **  @retval Option<PhysAddr>     the physics address of the beginning of allocated frame, if present
    */
    fn alloc_frame() -> Option<PhysAddr>;
    /*
    **  @brief  deallocate a frame for use
    **  @param  PhysAddr             the physics address of the beginning of frame to be deallocated
    **  @retval none
    */
    fn dealloc_frame(target: PhysAddr);
}
