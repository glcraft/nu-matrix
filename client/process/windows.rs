
// #[link(name = "Kernel32")]
// extern {
//     type DWORD = u32;
//     type HANDLE = u32;
//     type LPPROCESSENTRY32 = u32;
//     fn CreateToolhelp32Snapshot(dwflags: DWORD, th32processid: DWORD) -> HANDLE;
//     fn Process32First(hSnapshot: HANDLE, lppe: *mut u32) -> size_t;
//     fn Process32Next() -> size_t;
//     fn CloseHandle(source_length: size_t) -> size_t;
// }
use windows::Win32::{
    System::Diagnostics::ToolHelp::*, 
    Foundation::CloseHandle
};
pub fn parent_id() -> Option<u32> {
    unsafe {
        let mut proc_entry = PROCESSENTRY32 {
            dwSize: std::mem::size_of::<PROCESSENTRY32>() as u32,
            ..Default::default()
        };
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0).ok()?;
        let pid = std::process::id();
        let ppid = 'snap: {    
            Process32First(snapshot, &mut proc_entry).ok()?;
            loop {
                if proc_entry.th32ProcessID == pid {
                    break 'snap Some(proc_entry.th32ParentProcessID);
                }
                if Process32Next(snapshot, &mut proc_entry).is_err() {
                    break 'snap None;
                }
            }
        };
        let _ = CloseHandle(snapshot);
        ppid
    }
}

