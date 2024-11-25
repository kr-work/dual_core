use esp_idf_hal::delay::FreeRtos;
use esp_idf_sys::{self as _};
use esp_idf_sys::xTaskCreatePinnedToCore;
use std::ffi::CString;
use esp_idf_svc::sys::link_patches;
static TASK1_NAME: &str = "Task 1";
static TASK2_NAME: &str = "Task 2";


unsafe extern "C" fn task1(_: *mut core::ffi::c_void) {
    loop {
        for i in 0..10 {
            log::info!("Task 1 Entered {}", i);
            FreeRtos::delay_ms(1000);
        }
    }
}
unsafe extern "C" fn task2(_: *mut core::ffi::c_void) {
    loop {
        for i in 0..10 {
            log::info!("Task 2 Entered {}", i);
            FreeRtos::delay_ms(3000);
        }
    }
}

fn main() -> anyhow::Result<()> {
    link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    // ログレベルを警告レベルに設定
    unsafe {
        esp_idf_sys::esp_log_level_set(
            b"uart\0".as_ptr() as *const _,
            esp_idf_sys::esp_log_level_t_ESP_LOG_WARN,
        );
    }
    
    let task1_name = CString::new(TASK1_NAME).unwrap();
    unsafe {
        xTaskCreatePinnedToCore(
            Some(task1),
            task1_name.as_ptr(),
            4096,
            std::ptr::null_mut(),
            10,
            std::ptr::null_mut(),
            0,
        );
    }
    let task2_name = CString::new(TASK2_NAME).unwrap();
    unsafe {
        xTaskCreatePinnedToCore(
            Some(task2),
            task2_name.as_ptr(),
            4096,
            std::ptr::null_mut(),
            9,
            std::ptr::null_mut(),
            1,
        );
    }
    for i in 0..10 {
        log::info!("Main Entered {}", i);
        FreeRtos::delay_ms(5000);
    }
    Ok(())
}
