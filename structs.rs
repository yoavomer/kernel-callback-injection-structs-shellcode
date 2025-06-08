use std::ffi::c_void;
use winapi::shared::ntdef::{NTSTATUS, PVOID};

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct KernelCallbackTable {
    pub fn_copy_data: usize,
    pub fn_copy_global_data: usize,
    pub fn_dword: usize,
    pub fn_nc_destroy: usize,
    pub fn_dword_opt_in_lpmsg: usize,
    pub fn_inout_drag: usize,
    pub fn_get_text_lengths: usize,
    pub fn_in_cnt_out_string: usize,
    pub fn_p_out_lpint: usize,
    pub fn_in_lp_compare_item_struct: usize,
    pub fn_in_lp_create_struct: usize,
    pub fn_in_lp_delete_item_struct: usize,
    pub fn_in_lp_draw_item_struct: usize,
    pub fn_p_opt_in_lpu_int: usize,
    pub fn_p_opt_in_lpu_int2: usize,
    pub fn_in_lp_mdi_create_struct: usize,
    pub fn_inout_lp_measure_item_struct: usize,
    pub fn_in_lp_window_pos: usize,
    pub fn_inout_lp_point5: usize,
    pub fn_inout_lp_scroll_info: usize,
    pub fn_inout_lp_rect: usize,
    pub fn_inout_nc_calc_size: usize,
    pub fn_inout_lp_point5_: usize,
    pub fn_in_paint_clip_brd: usize,
    pub fn_in_size_clip_brd: usize,
    pub fn_in_destroy_clip_brd: usize,
    pub fn_in_string: usize,
    pub fn_in_string_null: usize,
    pub fn_in_device_change: usize,
    pub fn_power_broadcast: usize,
    pub fn_in_lp_ua_hdraw_menu: usize,
    pub fn_opt_out_lpdword_opt_out_lpdword: usize,
    pub fn_opt_out_lpdword_opt_out_lpdword_: usize,
    pub fn_out_dword_in_dword: usize,
    pub fn_out_lp_rect: usize,
    pub fn_out_string: usize,
    pub fn_p_opt_in_lpu_int3: usize,
    pub fn_p_out_lpint2: usize,
    pub fn_sent_ddemsg: usize,
    pub fn_inout_style_change: usize,
    pub fn_hk_in_dword: usize,
    pub fn_hk_in_lp_cb_tactivate_struct: usize,
    pub fn_hk_in_lp_cbt_create_struct: usize,
    pub fn_hk_in_lp_debug_hook_struct: usize,
    pub fn_hk_in_lp_mouse_hook_struct_ex: usize,
    pub fn_hk_in_lp_kbdll_hook_struct: usize,
    pub fn_hk_in_lp_msll_hook_struct: usize,
    pub fn_hk_in_lpmsg: usize,
    pub fn_hk_in_lp_rect: usize,
    pub fn_hk_opt_in_lp_event_msg: usize,
    pub xxx_client_call_delegate_thread: usize,
    pub client_call_dummy_callback: usize,
    pub fn_keyboard_correction_callout: usize,
    pub fn_out_lp_combo_box_info: usize,
    pub fn_in_lp_compare_item_struct2: usize,
    pub xxx_client_call_dev_callback_capture: usize,
    pub xxx_client_call_dit_thread: usize,
    pub xxx_client_enable_mmcss: usize,
    pub xxx_client_update_dpi: usize,
    pub xxx_client_expand_string_w: usize,
    pub client_copy_dde_in1: usize,
    pub client_copy_dde_in2: usize,
    pub client_copy_dde_out1: usize,
    pub client_copy_dde_out2: usize,
    pub client_copy_image: usize,
    pub client_event_callback: usize,
    pub client_find_mnem_char: usize,
    pub client_free_dde_handle: usize,
    pub client_free_library: usize,
    pub client_get_charset_info: usize,
    pub client_get_dde_flags: usize,
    pub client_get_dde_hook_data: usize,
    pub client_get_listbox_string: usize,
    pub client_get_message_mph: usize,
    pub client_load_image: usize,
    pub client_load_library: usize,
    pub client_load_menu: usize,
    pub client_load_local_t1_fonts: usize,
    pub client_psm_text_out: usize,
    pub client_lpk_draw_text_ex: usize,
    pub client_ext_text_out_w: usize,
    pub client_get_text_extent_point_w: usize,
    pub client_char_to_wchar: usize,
    pub client_add_font_resource_w: usize,
    pub client_thread_setup: usize,
    pub client_deliver_user_apc: usize,
    pub client_no_memory_popup: usize,
    pub client_monitor_enum_proc: usize,
    pub client_call_win_event_proc: usize,
    pub client_wait_message_ex_mph: usize,
    pub client_wow_get_proc_module: usize,
    pub client_wow_task16_sched_notify: usize,
    pub client_imm_load_layout: usize,
    pub client_imm_process_key: usize,
    pub fn_ime_control: usize,
    pub fn_in_wparam_dbcs_char: usize,
    pub fn_get_text_lengths2: usize,
    pub fn_in_lpk_draw_switch_wnd: usize,
    pub client_load_string_w: usize,
    pub client_load_ole: usize,
    pub client_register_drag_drop: usize,
    pub client_revoke_drag_drop: usize,
    pub fn_inout_menu_get_object: usize,
    pub client_printer_thunk: usize,
    pub fn_out_lp_combo_box_info2: usize,
    pub fn_out_lp_scrollbar_info: usize,
    pub fn_in_lp_ua_hdraw_menu2: usize,
    pub fn_in_lp_ua_hdraw_menu_item: usize,
    pub fn_in_lp_ua_hdraw_menu3: usize,
    pub fn_inout_lp_uah_measure_menu_item: usize,
    pub fn_in_lp_ua_hdraw_menu4: usize,
    pub fn_out_lp_title_bar_info_ex: usize,
    pub fn_touch: usize,
    pub fn_gesture: usize,
    pub fn_p_opt_in_lpu_int4: usize,
    pub fn_p_opt_in_lpu_int5: usize,
    pub xxx_client_call_default_input_handler: usize,
    pub fn_empty: usize,
    pub client_rim_dev_callback: usize,
    pub xxx_client_call_min_touch_hit_testing_callback: usize,
    pub client_call_local_mouse_hooks: usize,
    pub xxx_client_broadcast_theme_change: usize,
    pub xxx_client_call_dev_callback_simple: usize,
    pub xxx_client_alloc_window_class_extra_bytes: usize,
    pub xxx_client_free_window_class_extra_bytes: usize,
    pub fn_get_window_data: usize,
    pub fn_inout_style_change2: usize,
    pub fn_hk_in_lp_mouse_hook_struct_ex2: usize,
}
#[repr(C)]
#[derive(Debug)]
pub struct ProcessBasicInformation {
    pub exit_status: NTSTATUS,
    pub peb_base_address: *mut c_void,
    pub affinity_mask: usize,
    pub base_priority: i32,
    pub unique_process_id: usize,
    pub inherited_from_unique_process_id: usize,
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PEB {
    pub inherited_address_space: u8,                                     // 0x000
    pub read_image_file_exec_options: u8,                                // 0x001
    pub being_debugged: u8,                                              // 0x002
    pub bit_field: u8,                                                     // 0x003 (bitfield: ImageUsesLargePages, etc.)
    pub padding0: [u8; 4],                                               // 0x004
    pub mutant: *mut c_void,                                             // 0x008
    pub image_base_address: *mut c_void,                                 // 0x010
    pub ldr: *mut c_void,                                                // 0x018
    pub process_parameters: *mut c_void,                                 // 0x020
    pub subsystem_data: *mut c_void,                                     // 0x028
    pub process_heap: *mut c_void,                                       // 0x030
    pub fast_peb_lock: *mut c_void,                                      // 0x038
    pub atl_thunk_slist_ptr: *mut c_void,                                // 0x040
    pub ifeo_key: *mut c_void,                                           // 0x048
    pub cross_process_flags: u32,                                        // 0x050 (bitfield)
    pub padding1: [u8; 4],                                               // 0x054
    pub kernel_callback_table: *mut c_void,                              // 0x058 (union with UserSharedInfoPtr)
    pub system_reserved: u32,                                            // 0x060
    pub atl_thunk_slist_ptr32: u32,                                      // 0x064
    pub api_set_map: *mut c_void,                                        // 0x068
    pub tls_expansion_counter: u32,                                      // 0x070
    pub padding2: [u8; 4],                                               // 0x074
    pub tls_bitmap: *mut c_void,                                         // 0x078
    pub tls_bitmap_bits: [u32; 2],                                       // 0x080
    pub read_only_shared_memory_base: *mut c_void,                       // 0x088
    pub shared_data: *mut c_void,                                        // 0x090
    pub read_only_static_server_data: *mut *mut c_void,                  // 0x098
    pub ansi_code_page_data: *mut c_void,                                // 0x0A0
    pub oem_code_page_data: *mut c_void,                                 // 0x0A8
    pub unicode_case_table_data: *mut c_void,                            // 0x0B0
    pub number_of_processors: u32,                                       // 0x0B8
    pub nt_global_flag: u32,                                             // 0x0BC
    pub critical_section_timeout: i64,                                   // 0x0C0 (LARGE_INTEGER)
    pub heap_segment_reserve: u64,                                       // 0x0C8
    pub heap_segment_commit: u64,                                        // 0x0D0
    pub heap_de_commit_total_free_threshold: u64,                        // 0x0D8
    pub heap_de_commit_free_block_threshold: u64,                        // 0x0E0
    pub number_of_heaps: u32,                                            // 0x0E8
    pub maximum_number_of_heaps: u32,                                    // 0x0EC
    pub process_heaps: *mut *mut c_void,                                 // 0x0F0
    pub gdi_shared_handle_table: *mut c_void,                            // 0x0F8
    pub process_starter_helper: *mut c_void,                             // 0x100
    pub gdi_dc_attribute_list: u32,                                      // 0x108
    pub padding3: [u8; 4],                                               // 0x10C
    pub loader_lock: *mut c_void,                                        // 0x110
    pub os_major_version: u32,                                           // 0x118
    pub os_minor_version: u32,                                           // 0x11C
    pub os_build_number: u16,                                            // 0x120
    pub os_csd_version: u16,                                             // 0x122
    pub os_platform_id: u32,                                             // 0x124
    pub image_subsystem: u32,                                            // 0x128
    pub image_subsystem_major_version: u32,                              // 0x12C
    pub image_subsystem_minor_version: u32,                              // 0x130
    pub padding4: [u8; 4],                                               // 0x134
    pub active_process_affinity_mask: u64,                               // 0x138
    pub gdi_handle_buffer: [u32; 60],                                    // 0x140
    pub post_process_init_routine: *mut c_void,                          // 0x230
    pub tls_expansion_bitmap: *mut c_void,                               // 0x238
    pub tls_expansion_bitmap_bits: [u32; 32],                            // 0x240
    pub session_id: u32,                                                 // 0x2C0
    pub padding5: [u8; 4],                                               // 0x2C4
    pub app_compat_flags: u64,                                           // 0x2C8 (ULARGE_INTEGER)
    pub app_compat_flags_user: u64,                                      // 0x2D0 (ULARGE_INTEGER)
    pub shim_data: *mut c_void,                                          // 0x2D8
    pub app_compat_info: *mut c_void,                                    // 0x2E0
    pub csd_version: UnicodeString,                                     // 0x2E8
    pub activation_context_data: *mut c_void,                            // 0x2F8
    pub process_assembly_storage_map: *mut c_void,                       // 0x300
    pub system_default_activation_context_data: *mut c_void,            // 0x308
    pub system_assembly_storage_map: *mut c_void,                        // 0x310
    pub minimum_stack_commit: u64,                                       // 0x318
    pub fls_callback: *mut c_void,                                       // 0x320
    pub fls_list_head: ListEntry,                                       // 0x328
    pub fls_bitmap: *mut c_void,                                         // 0x338
    pub fls_bitmap_bits: [u32; 4],                                       // 0x340
    pub fls_high_index: u32,                                             // 0x350
    pub padding6: [u8; 4],                                               // 0x354
    pub wer_registration_data: *mut c_void,                              // 0x358
    pub wer_ship_assert_ptr: *mut c_void,                                // 0x360
    pub unused: *mut c_void,                                             // 0x368
    pub image_header_hash: *mut c_void,                                  // 0x370
    pub tracing_flags: u32,                                              // 0x378 (bitfield)
    pub padding7: [u8; 4],                                               // 0x37C
    pub csr_server_read_only_shared_memory_base: u64,                    // 0x380
    pub tpp_workerp_list_lock: u64,                                      // 0x388
    pub tpp_workerp_list: ListEntry,                                    // 0x390
    pub wait_on_address_hash_table: [*mut c_void; 128],                  // 0x3A0
    pub telemetry_coverage_header: *mut c_void,                          // 0x7A0
    pub cloud_file_flags: u32,                                           // 0x7A8
    pub cloud_file_diag_flags: u32,                                      // 0x7AC
    pub placeholder_compatibility_mode: u8,                              // 0x7B0
    pub placeholder_compatibility_mode_reserved: [u8; 7],                // 0x7B1
    pub leap_second_data: *mut c_void,                                   // 0x7B8
    pub leap_second_flags: u32,                                          // 0x7C0
    pub nt_global_flag2: u32,                                            // 0x7C4
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct UnicodeString {
    pub length: u16,
    pub maximum_length: u16,
    pub buffer: *mut u16,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ListEntry {
    pub flink: *mut ListEntry,
    pub blink: *mut ListEntry,
}
