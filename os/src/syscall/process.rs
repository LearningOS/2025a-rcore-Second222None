//! Process management syscalls
use crate::config::PAGE_SIZE;
use crate::task::{change_program_brk, current_user_token, exit_current_and_run_next, suspend_current_and_run_next, TASK_MANAGER};

use crate::mm::{MapPermission, VirtAddr, translated_byte_buffer};
use crate::timer::get_time_us;

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");

    let length = core::mem::size_of::<TimeVal>();
    let mut timeval_buf = translated_byte_buffer(current_user_token(), ts as *const u8, length);

    let us = get_time_us();

    let timeval = unsafe { &mut *(timeval_buf[0].as_mut_ptr() as *mut TimeVal) };
    timeval.sec = us / 1_000_000;
    timeval.usec = us % 1_000_000;

    0
}

/// TODO: Finish sys_trace to pass testcases
/// HINT: You might reimplement it with virtual memory management.
pub fn sys_trace(trace_request: usize, id: usize, data: usize) -> isize {
    trace!("kernel: sys_trace");
    TASK_MANAGER.set_nr_syscall(id);
    match trace_request {
        0 => {
            // enable tracing for syscall with _id
            let kernel_id = translated_byte_buffer(current_user_token(), id as *const u8, 1);
            (unsafe { *(kernel_id[0].as_ptr()) }).into()
        },
        1 => {
            // disable tracing for syscall with _id
            let mut kernel_id = translated_byte_buffer(current_user_token(), id as *mut u8, 1);
            (unsafe { *(kernel_id[0].as_mut_ptr()) = data as u8 });
            0
        },
        2 => {
            // get the number of times syscall with _id has been called
            TASK_MANAGER.get_nr_syscall(id) as isize
        },
        _ => -1,
    }
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(start: usize, len: usize, prot: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    let mut perm = crate::mm::MapPermission::empty();
    // check start alignment
    if start % PAGE_SIZE != 0 {
        return -1;
    }

    // prot validity: only low 3 bits allowed and must not be zero
    if prot & !0x7 != 0 || prot & 0x7 == 0 {
        return -1;
    } else {
        // build permissions from prot bits: bit0 read, bit1 write, bit2 exec
        if prot & 0x1 != 0 { perm |= MapPermission::R; }
        if prot & 0x2 != 0 { perm |= MapPermission::W; }
        if prot & 0x4 != 0 { perm |= MapPermission::X; }
        // user bit usually needed for user mappings
        perm |= crate::mm::MapPermission::U;
    }

    // nothing to do
    if len == 0 {
        return 0;
    }

    // --- Phase 1: detect overlap with already mapped pages ---


    TASK_MANAGER.add_mmap_area(
        VirtAddr::from(start),
        VirtAddr::from(start + len),
        perm,
    );

    // number of pages to map (round up)
    // let npages = (len + PAGE_SIZE - 1) / PAGE_SIZE;
    // let token = current_user_token();

    // --- Phase 2: allocate physical frames and map them ---
    // The following uses typical rCore-like helpers. Replace with your project's frame allocator
    // and page-table map call if names differ.
    // for i in 0..npages {
    //     let va = start + i * PAGE_SIZE;

    //     // allocate one physical frame/page
    //     // expected API: crate::mm::frame_alloc() -> Option<FrameTracker>
    //     let frame = match frame_alloc() {
    //         Some(f) => f,
    //         None => return -1, // physical memory exhausted
    //     };

    //     // build permissions from prot bits: bit0 read, bit1 write, bit2 exec
    //     let mut perm = crate::mm::MapPermission::empty();
    //     if prot & 0x1 != 0 { perm |= MapPermission::R; }
    //     if prot & 0x2 != 0 { perm |= MapPermission::W; }
    //     if prot & 0x4 != 0 { perm |= MapPermission::X; }
    //     // user bit usually needed for user mappings
    //     perm |= crate::mm::MapPermission::U;

    //     // map the frame to VA.
    //     // let token = current_user_token();
    //     let mut page_table = PageTable::from_token(token);
    //     let pte_flags = PTEFlags::from_bits(perm.bits()).unwrap();

    //     page_table.map(
    //         VirtAddr::from(va).into(),
    //         frame.ppn,
    //         pte_flags,
    //     );
    // }
    0
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    -1
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
