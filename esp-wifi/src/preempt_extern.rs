use crate::binary::c_types;

extern "Rust" {
    fn esp_wifi_preempt_setup(timer: crate::TimeBase);
    fn esp_wifi_preempt_disable();
    fn esp_wifi_preempt_yield_task();
    fn esp_wifi_preempt_current_task() -> *mut c_types::c_void;
    fn esp_wifi_preempt_task_create(
        task: extern "C" fn(*mut c_types::c_void),
        param: *mut c_types::c_void,
        task_stack_size: usize,
    ) -> *mut c_types::c_void;
    fn esp_wifi_preempt_schedule_task_deletion(task_handle: *mut c_types::c_void);
    fn esp_wifi_preempt_current_task_thread_semaphore() -> *mut c_types::c_void;
}

pub(crate) fn setup(timer: crate::TimeBase) {
    unsafe { esp_wifi_preempt_setup(timer) }
}

pub(crate) fn disable() {
    unsafe { esp_wifi_preempt_disable() }
}

pub(crate) fn yield_task() {
    unsafe { esp_wifi_preempt_yield_task() }
}

pub(crate) fn current_task() -> *mut c_types::c_void {
    unsafe { esp_wifi_preempt_current_task() }
}

pub(crate) fn task_create(
    task: extern "C" fn(*mut c_types::c_void),
    param: *mut c_types::c_void,
    task_stack_size: usize,
) -> *mut c_types::c_void {
    unsafe { esp_wifi_preempt_task_create(task, param, task_stack_size) }
}

pub(crate) fn schedule_task_deletion(task_handle: *mut c_types::c_void) {
    unsafe { esp_wifi_preempt_schedule_task_deletion(task_handle) }
}

pub(crate) fn current_task_thread_semaphore() -> *mut c_types::c_void {
    unsafe { esp_wifi_preempt_current_task_thread_semaphore() }
}
