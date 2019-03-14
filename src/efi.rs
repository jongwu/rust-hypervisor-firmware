// Copyright © 2019 Intel Corporation
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use r_efi::efi;
use r_efi::efi::{
    AllocateType, Boolean, CapsuleHeader, Char16, Event, EventNotify, Guid, Handle, InterfaceType,
    LocateSearchType, MemoryDescriptor, MemoryType, OpenProtocolInformationEntry, PhysicalAddress,
    ResetType, Status, Time, TimeCapabilities, TimerDelay, Tpl,
};

use r_efi::protocols::simple_text_input::InputKey;
use r_efi::protocols::simple_text_input::Protocol as SimpleTextInputProtocol;
use r_efi::protocols::simple_text_output::Mode as SimpleTextOutputMode;
use r_efi::protocols::simple_text_output::Protocol as SimpleTextOutputProtocol;

use core::ffi::c_void;

#[cfg(not(test))]
pub extern "win64" fn stdin_reset(_: *mut SimpleTextInputProtocol, _: Boolean) -> Status {
    crate::log!("EFI_STUB: stdin_reset\n");
    Status::UNSUPPORTED
}

#[cfg(not(test))]
pub extern "win64" fn stdin_read_key_stroke(
    _: *mut SimpleTextInputProtocol,
    _: *mut InputKey,
) -> Status {
    crate::log!("EFI_STUB: stdin_read_key_stroke\n");
    Status::UNSUPPORTED
}

#[cfg(not(test))]
pub extern "win64" fn stdout_reset(_: *mut SimpleTextOutputProtocol, _: Boolean) -> Status {
    crate::log!("EFI_STUB: stdout_reset\n");
    Status::UNSUPPORTED
}

#[cfg(not(test))]
pub extern "win64" fn stdout_output_string(
    _: *mut SimpleTextOutputProtocol,
    message: *mut Char16,
) -> Status {
    let mut string_end = false;

    loop {
        let mut output: [u8; 128] = [0; 128];
        let mut i: usize = 0;
        while i < output.len() {
            output[i] = (unsafe { *message.add(i) } & 0xffu16) as u8;
            if output[i] == 0 {
                string_end = true;
                break;
            }
            i += 1;
        }
        crate::log!("{}", unsafe { core::str::from_utf8_unchecked(&output) });
        if string_end {
            break;
        }
    }
    Status::SUCCESS
}

#[cfg(not(test))]
pub extern "win64" fn stdout_test_string(
    _: *mut SimpleTextOutputProtocol,
    _: *mut Char16,
) -> Status {
    crate::log!("EFI_STUB: stdout_test_string\n");
    Status::UNSUPPORTED
}

#[cfg(not(test))]
pub extern "win64" fn stdout_query_mode(
    _: *mut SimpleTextOutputProtocol,
    _: usize,
    _: *mut usize,
    _: *mut usize,
) -> Status {
    crate::log!("EFI_STUB: stdout_query_mode\n");
    Status::UNSUPPORTED
}

#[cfg(not(test))]
pub extern "win64" fn stdout_set_mode(_: *mut SimpleTextOutputProtocol, _: usize) -> Status {
    crate::log!("EFI_STUB: stdout_set_mode\n");
    Status::UNSUPPORTED
}

#[cfg(not(test))]
pub extern "win64" fn stdout_set_attribute(_: *mut SimpleTextOutputProtocol, _: usize) -> Status {
    crate::log!("EFI_STUB: stdout_set_attribute\n");
    Status::UNSUPPORTED
}

#[cfg(not(test))]
pub extern "win64" fn stdout_clear_screen(_: *mut SimpleTextOutputProtocol) -> Status {
    crate::log!("EFI_STUB: stdout_clear_screen\n");
    Status::UNSUPPORTED
}

#[cfg(not(test))]
pub extern "win64" fn stdout_set_cursor_position(
    _: *mut SimpleTextOutputProtocol,
    _: usize,
    _: usize,
) -> Status {
    crate::log!("EFI_STUB: stdout_set_cursor_position\n");
    Status::UNSUPPORTED
}

#[cfg(not(test))]
pub extern "win64" fn stdout_enable_cursor(_: *mut SimpleTextOutputProtocol, _: Boolean) -> Status {
    crate::log!("EFI_STUB: stdout_enable_cursor\n");
    Status::UNSUPPORTED
}

#[cfg(not(test))]
pub extern "win64" fn get_time(_: *mut Time, _: *mut TimeCapabilities) -> Status {
    crate::log!("EFI_STUB: get_time\n");
    Status::UNSUPPORTED
}

#[cfg(not(test))]
pub extern "win64" fn set_time(_: *mut Time) -> Status {
    crate::log!("EFI_STUB: set_time\n");
    Status::UNSUPPORTED
}

#[cfg(not(test))]
pub extern "win64" fn get_wakeup_time(_: *mut Boolean, _: *mut Boolean, _: *mut Time) -> Status {
    crate::log!("EFI_STUB: get_wakeup_time\n");
    Status::UNSUPPORTED
}

#[cfg(not(test))]
pub extern "win64" fn set_wakeup_time(_: Boolean, _: *mut Time) -> Status {
    crate::log!("EFI_STUB: set_wakeup_time\n");
    Status::UNSUPPORTED
}

#[cfg(not(test))]
pub extern "win64" fn set_virtual_address_map(
    _: usize,
    _: usize,
    _: u32,
    _: *mut MemoryDescriptor,
) -> Status {
    crate::log!("EFI_STUB: set_virtual_address_map\n");
    Status::UNSUPPORTED
}

#[cfg(not(test))]
pub extern "win64" fn convert_pointer(_: usize, _: *mut *mut c_void) -> Status {
    crate::log!("EFI_STUB: convert_pointer\n");
    Status::UNSUPPORTED
}

#[cfg(not(test))]
pub extern "win64" fn get_variable(
    _: *mut Char16,
    _: *mut Guid,
    _: *mut u32,
    _: *mut usize,
    _: *mut core::ffi::c_void,
) -> Status {
    crate::log!("EFI_STUB: get_variable\n");
    Status::UNSUPPORTED
}

#[cfg(not(test))]
pub extern "win64" fn get_next_variable_name(
    _: *mut usize,
    _: *mut Char16,
    _: *mut Guid,
) -> Status {
    crate::log!("EFI_STUB: get_next_variable\n");
    Status::UNSUPPORTED
}

#[cfg(not(test))]
pub extern "win64" fn set_variable(
    _: *mut Char16,
    _: *mut Guid,
    _: u32,
    _: usize,
    _: *mut c_void,
) -> Status {
    crate::log!("EFI_STUB: set_variable\n");
    Status::UNSUPPORTED
}

#[cfg(not(test))]
pub extern "win64" fn get_next_high_mono_count(_: *mut u32) -> Status {
    crate::log!("EFI_STUB: get_next_high_mono_count\n");
    Status::UNSUPPORTED
}

#[cfg(not(test))]
pub extern "win64" fn reset_system(_: ResetType, _: Status, _: usize, _: *mut c_void) {
    crate::log!("EFI_STUB: reset_system\n");
}

#[cfg(not(test))]
pub extern "win64" fn update_capsule(
    _: *mut *mut CapsuleHeader,
    _: usize,
    _: PhysicalAddress,
) -> Status {
    crate::log!("EFI_STUB: update_capsule\n");
    Status::UNSUPPORTED
}

#[cfg(not(test))]
pub extern "win64" fn query_capsule_capabilities(
    _: *mut *mut CapsuleHeader,
    _: usize,
    _: *mut u64,
    _: *mut ResetType,
) -> Status {
    crate::log!("EFI_STUB: query_capsule_capabilities\n");
    Status::UNSUPPORTED
}

#[cfg(not(test))]
pub extern "win64" fn query_variable_info(_: u32, _: *mut u64, _: *mut u64, _: *mut u64) -> Status {
    crate::log!("EFI_STUB: query_variable_info\n");
    Status::UNSUPPORTED
}

#[cfg(not(test))]
pub extern "win64" fn raise_tpl(_: Tpl) -> Tpl {
    crate::log!("EFI_STUB: raise_tpl\n");
    0
}

#[cfg(not(test))]
pub extern "win64" fn restore_tpl(_: Tpl) {
    crate::log!("EFI_STUB: restore_tpl\n");
}

#[cfg(not(test))]
pub extern "win64" fn allocate_pages(
    _: AllocateType,
    _: MemoryType,
    _: usize,
    _: *mut PhysicalAddress,
) -> Status {
    crate::log!("EFI_STUB: allocate_pages\n");
    Status::UNSUPPORTED
}

#[cfg(not(test))]
pub extern "win64" fn free_pages(_: PhysicalAddress, _: usize) -> Status {
    crate::log!("EFI_STUB: free_pages\n");
    Status::UNSUPPORTED
}

#[cfg(not(test))]
pub extern "win64" fn get_memory_map(
    _: *mut usize,
    _: *mut MemoryDescriptor,
    _: *mut usize,
    _: *mut usize,
    _: *mut u32,
) -> Status {
    crate::log!("EFI_STUB: get_memory_map\n");
    Status::UNSUPPORTED
}

#[cfg(not(test))]
pub extern "win64" fn allocate_pool(_: MemoryType, _: usize, _: *mut *mut c_void) -> Status {
    crate::log!("EFI_STUB: allocate_pool\n");
    Status::UNSUPPORTED
}

#[cfg(not(test))]
pub extern "win64" fn free_pool(_: *mut c_void) -> Status {
    crate::log!("EFI_STUB: free_pool\n");
    Status::UNSUPPORTED
}

#[cfg(not(test))]
pub extern "win64" fn create_event(
    _: u32,
    _: Tpl,
    _: EventNotify,
    _: *mut c_void,
    _: *mut Event,
) -> Status {
    crate::log!("EFI_STUB: create_event\n");
    Status::UNSUPPORTED
}

#[cfg(not(test))]
pub extern "win64" fn set_timer(_: Event, _: TimerDelay, _: u64) -> Status {
    crate::log!("EFI_STUB: set_timer\n");
    Status::UNSUPPORTED
}

#[cfg(not(test))]
pub extern "win64" fn wait_for_event(_: usize, _: *mut Event, _: *mut usize) -> Status {
    crate::log!("EFI_STUB: wait_for_event\n");
    Status::UNSUPPORTED
}

#[cfg(not(test))]
pub extern "win64" fn signal_event(_: Event) -> Status {
    crate::log!("EFI_STUB: signal_event\n");
    Status::UNSUPPORTED
}

#[cfg(not(test))]
pub extern "win64" fn close_event(_: Event) -> Status {
    crate::log!("EFI_STUB: close_event\n");
    Status::UNSUPPORTED
}

#[cfg(not(test))]
pub extern "win64" fn check_event(_: Event) -> Status {
    crate::log!("EFI_STUB: check_event\n");
    Status::UNSUPPORTED
}

#[cfg(not(test))]
pub extern "win64" fn install_protocol_interface(
    _: *mut Handle,
    _: *mut Guid,
    _: InterfaceType,
    _: *mut c_void,
) -> Status {
    crate::log!("EFI_STUB: install_protocol_interface\n");
    Status::UNSUPPORTED
}

#[cfg(not(test))]
pub extern "win64" fn reinstall_protocol_interface(
    _: Handle,
    _: *mut Guid,
    _: *mut c_void,
    _: *mut c_void,
) -> Status {
    crate::log!("EFI_STUB: reinstall_protocol_interface\n");
    Status::UNSUPPORTED
}

#[cfg(not(test))]
pub extern "win64" fn uninstall_protocol_interface(
    _: Handle,
    _: *mut Guid,
    _: *mut c_void,
) -> Status {
    crate::log!("EFI_STUB: uninstall_protocol_interface\n");
    Status::UNSUPPORTED
}

#[cfg(not(test))]
pub extern "win64" fn handle_protocol(_: Handle, _: *mut Guid, _: *mut *mut c_void) -> Status {
    crate::log!("EFI_STUB: handle_protocol\n");
    Status::UNSUPPORTED
}

#[cfg(not(test))]
pub extern "win64" fn register_protocol_notify(
    _: *mut Guid,
    _: Event,
    _: *mut *mut c_void,
) -> Status {
    crate::log!("EFI_STUB: register_protocol_notify\n");
    Status::UNSUPPORTED
}

#[cfg(not(test))]
pub extern "win64" fn locate_handle(
    _: LocateSearchType,
    _: *mut Guid,
    _: *mut c_void,
    _: *mut usize,
    _: *mut Handle,
) -> Status {
    crate::log!("EFI_STUB: locate_handle\n");
    Status::UNSUPPORTED
}

#[cfg(not(test))]
pub extern "win64" fn locate_device_path(_: *mut Guid, _: *mut *mut c_void) -> Status {
    crate::log!("EFI_STUB: locate_device_path\n");
    Status::UNSUPPORTED
}

#[cfg(not(test))]
pub extern "win64" fn install_configuration_table(_: *mut Guid, _: *mut c_void) -> Status {
    crate::log!("EFI_STUB: install_configuration_table\n");

    Status::UNSUPPORTED
}

#[cfg(not(test))]
pub extern "win64" fn load_image(
    _: Boolean,
    _: Handle,
    _: *mut c_void,
    _: *mut c_void,
    _: usize,
    _: *mut Handle,
) -> Status {
    crate::log!("EFI_STUB: load_image\n");
    Status::UNSUPPORTED
}

#[cfg(not(test))]
pub extern "win64" fn start_image(_: Handle, _: *mut usize, _: *mut *mut Char16) -> Status {
    crate::log!("EFI_STUB: start_image\n");
    Status::UNSUPPORTED
}

#[cfg(not(test))]
pub extern "win64" fn exit(_: Handle, _: Status, _: usize, _: *mut Char16) -> Status {
    crate::log!("EFI_STUB: exit\n");
    Status::UNSUPPORTED
}

#[cfg(not(test))]
pub extern "win64" fn unload_image(_: Handle) -> Status {
    crate::log!("EFI_STUB: unload_image\n");
    Status::UNSUPPORTED
}

#[cfg(not(test))]
pub extern "win64" fn exit_boot_services(_: Handle, _: usize) -> Status {
    crate::log!("EFI_STUB: exit_boot_services\n");
    Status::UNSUPPORTED
}

#[cfg(not(test))]
pub extern "win64" fn get_next_monotonic_count(_: *mut u64) -> Status {
    crate::log!("EFI_STUB: get_next_monotonic_count\n");
    Status::UNSUPPORTED
}

#[cfg(not(test))]
pub extern "win64" fn stall(_: usize) -> Status {
    crate::log!("EFI_STUB: stall\n");
    Status::UNSUPPORTED
}

#[cfg(not(test))]
pub extern "win64" fn set_watchdog_timer(_: usize, _: u64, _: usize, _: *mut Char16) -> Status {
    crate::log!("EFI_STUB: set_watchdog_timer\n");
    Status::UNSUPPORTED
}

#[cfg(not(test))]
pub extern "win64" fn connect_controller(
    _: Handle,
    _: *mut Handle,
    _: *mut c_void,
    _: Boolean,
) -> Status {
    crate::log!("EFI_STUB: connect_controller\n");
    Status::UNSUPPORTED
}

#[cfg(not(test))]
pub extern "win64" fn disconnect_controller(_: Handle, _: Handle, _: Handle) -> Status {
    crate::log!("EFI_STUB: disconnect_controller\n");
    Status::UNSUPPORTED
}

#[cfg(not(test))]
pub extern "win64" fn open_protocol(
    _: Handle,
    _: *mut Guid,
    _: *mut *mut c_void,
    _: Handle,
    _: Handle,
    _: u32,
) -> Status {
    crate::log!("EFI_STUB: open_protocol\n");
    Status::UNSUPPORTED
}

#[cfg(not(test))]
pub extern "win64" fn close_protocol(_: Handle, _: *mut Guid, _: Handle, _: Handle) -> Status {
    crate::log!("EFI_STUB: close_protocol\n");
    Status::UNSUPPORTED
}

#[cfg(not(test))]
pub extern "win64" fn open_protocol_information(
    _: Handle,
    _: *mut Guid,
    _: *mut *mut OpenProtocolInformationEntry,
    _: *mut usize,
) -> Status {
    crate::log!("EFI_STUB: open_protocol_information\n");
    Status::UNSUPPORTED
}

#[cfg(not(test))]
pub extern "win64" fn protocols_per_handle(
    _: Handle,
    _: *mut *mut *mut Guid,
    _: *mut usize,
) -> Status {
    crate::log!("EFI_STUB: protocols_per_handle\n");
    Status::UNSUPPORTED
}

#[cfg(not(test))]
pub extern "win64" fn locate_handle_buffer(
    _: LocateSearchType,
    _: *mut Guid,
    _: *mut c_void,
    _: *mut usize,
    _: *mut *mut Handle,
) -> Status {
    crate::log!("EFI_STUB: locate_handle_buffer\n");
    Status::UNSUPPORTED
}

#[cfg(not(test))]
pub extern "win64" fn locate_protocol(_: *mut Guid, _: *mut c_void, _: *mut *mut c_void) -> Status {
    crate::log!("EFI_STUB: locate_protocol\n");
    Status::UNSUPPORTED
}

#[cfg(not(test))]
pub extern "win64" fn install_multiple_protocol_interfaces(
    _: *mut Handle,
    _: *mut c_void,
    _: *mut c_void,
) -> Status {
    crate::log!("EFI_STUB: install_multiple_protocol_interfaces\n");
    Status::UNSUPPORTED
}

#[cfg(not(test))]
pub extern "win64" fn uninstall_multiple_protocol_interfaces(
    _: *mut Handle,
    _: *mut c_void,
    _: *mut c_void,
) -> Status {
    crate::log!("EFI_STUB: uninstall_multiple_protocol_interfaces\n");
    Status::UNSUPPORTED
}

#[cfg(not(test))]
pub extern "win64" fn calculate_crc32(_: *mut c_void, _: usize, _: *mut u32) -> Status {
    crate::log!("EFI_STUB: calculate_crc32\n");
    Status::UNSUPPORTED
}

#[cfg(not(test))]
pub extern "win64" fn copy_mem(_: *mut c_void, _: *mut c_void, _: usize) {
    crate::log!("EFI_STUB: copy_mem\n");
}

#[cfg(not(test))]
pub extern "win64" fn set_mem(_: *mut c_void, _: usize, _: u8) {
    crate::log!("EFI_STUB: set_mem\n");
}

#[cfg(not(test))]
pub extern "win64" fn create_event_ex(
    _: u32,
    _: Tpl,
    _: EventNotify,
    _: *const c_void,
    _: *const Guid,
    _: *mut Event,
) -> Status {
    crate::log!("EFI_STUB: create_event_ex\n");
    Status::UNSUPPORTED
}

#[cfg(not(test))]
pub fn efi_exec(address: u64, _loaded_address: u64, _loaded_size: u64) {
    let mut stdin = SimpleTextInputProtocol {
        reset: stdin_reset,
        read_key_stroke: stdin_read_key_stroke,
        wait_for_key: 0 as Event,
    };

    let mut stdout_mode = SimpleTextOutputMode {
        max_mode: 1,
        mode: 0,
        attribute: 0,
        cursor_column: 0,
        cursor_row: 0,
        cursor_visible: Boolean::FALSE,
    };

    let mut stdout = SimpleTextOutputProtocol {
        reset: stdout_reset,
        output_string: stdout_output_string,
        test_string: stdout_test_string,
        query_mode: stdout_query_mode,
        set_mode: stdout_set_mode,
        set_attribute: stdout_set_attribute,
        clear_screen: stdout_clear_screen,
        set_cursor_position: stdout_set_cursor_position,
        enable_cursor: stdout_enable_cursor,
        mode: &mut stdout_mode,
    };

    let mut rs = efi::RuntimeServices {
        hdr: efi::TableHeader {
            signature: efi::RUNTIME_SERVICES_SIGNATURE,
            revision: efi::RUNTIME_SERVICES_REVISION,
            header_size: core::mem::size_of::<efi::RuntimeServices>() as u32,
            crc32: 0, // TODO
            reserved: 0,
        },
        get_time,
        set_time,
        get_wakeup_time,
        set_wakeup_time,
        set_virtual_address_map,
        convert_pointer,
        get_variable,
        get_next_variable_name,
        set_variable,
        get_next_high_mono_count,
        reset_system,
        update_capsule,
        query_capsule_capabilities,
        query_variable_info,
    };

    let mut bs = efi::BootServices {
        hdr: efi::TableHeader {
            signature: efi::BOOT_SERVICES_SIGNATURE,
            revision: efi::BOOT_SERVICES_REVISION,
            header_size: core::mem::size_of::<efi::BootServices>() as u32,
            crc32: 0, // TODO
            reserved: 0,
        },
        raise_tpl,
        restore_tpl,
        allocate_pages,
        free_pages,
        get_memory_map,
        allocate_pool,
        free_pool,
        create_event,
        set_timer,
        wait_for_event,
        signal_event,
        close_event,
        check_event,
        install_protocol_interface,
        reinstall_protocol_interface,
        uninstall_protocol_interface,
        handle_protocol,
        register_protocol_notify,
        locate_handle,
        locate_device_path,
        install_configuration_table,
        load_image,
        start_image,
        exit,
        unload_image,
        exit_boot_services,
        get_next_monotonic_count,
        stall,
        set_watchdog_timer,
        connect_controller,
        disconnect_controller,
        open_protocol,
        close_protocol,
        open_protocol_information,
        protocols_per_handle,
        locate_handle_buffer,
        locate_protocol,
        install_multiple_protocol_interfaces,
        uninstall_multiple_protocol_interfaces,
        calculate_crc32,
        copy_mem,
        set_mem,
        create_event_ex,
        reserved: core::ptr::null_mut(),
    };

    let mut ct = efi::ConfigurationTable {
        vendor_guid: Guid::from_fields(0, 0, 0, 0, 0, &[0; 6]), // TODO
        vendor_table: core::ptr::null_mut(),
    };

    let mut st = efi::SystemTable {
        hdr: efi::TableHeader {
            signature: efi::SYSTEM_TABLE_SIGNATURE,
            revision: efi::SYSTEM_TABLE_REVISION_2_70,
            header_size: core::mem::size_of::<efi::SystemTable>() as u32,
            crc32: 0, // TODO
            reserved: 0,
        },
        firmware_vendor: core::ptr::null_mut(), // TODO,
        firmware_revision: 0,
        console_in_handle: 0 as Handle,
        con_in: &mut stdin,
        console_out_handle: 1 as Handle, // TODO
        con_out: &mut stdout,
        standard_error_handle: 2 as Handle, // TODO
        std_err: &mut stdout,
        runtime_services: &mut rs,
        boot_services: &mut bs,
        number_of_table_entries: 0,
        configuration_table: &mut ct,
    };

    let ptr = address as *const ();
    let code: extern "win64" fn(Handle, *mut efi::SystemTable) -> Status =
        unsafe { core::mem::transmute(ptr) };
    // TODO: use something better for the handle
    (code)(3 as Handle, &mut st);
}