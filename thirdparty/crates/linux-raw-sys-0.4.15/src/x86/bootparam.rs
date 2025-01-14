/* automatically generated by rust-bindgen 0.70.1 */

pub type __s8 = crate::ctypes::c_schar;
pub type __u8 = crate::ctypes::c_uchar;
pub type __s16 = crate::ctypes::c_short;
pub type __u16 = crate::ctypes::c_ushort;
pub type __s32 = crate::ctypes::c_int;
pub type __u32 = crate::ctypes::c_uint;
pub type __s64 = crate::ctypes::c_longlong;
pub type __u64 = crate::ctypes::c_ulonglong;
pub type __kernel_key_t = crate::ctypes::c_int;
pub type __kernel_mqd_t = crate::ctypes::c_int;
pub type __kernel_mode_t = crate::ctypes::c_ushort;
pub type __kernel_ipc_pid_t = crate::ctypes::c_ushort;
pub type __kernel_uid_t = crate::ctypes::c_ushort;
pub type __kernel_gid_t = crate::ctypes::c_ushort;
pub type __kernel_old_dev_t = crate::ctypes::c_ushort;
pub type __kernel_long_t = crate::ctypes::c_long;
pub type __kernel_ulong_t = crate::ctypes::c_ulong;
pub type __kernel_ino_t = __kernel_ulong_t;
pub type __kernel_pid_t = crate::ctypes::c_int;
pub type __kernel_suseconds_t = __kernel_long_t;
pub type __kernel_daddr_t = crate::ctypes::c_int;
pub type __kernel_uid32_t = crate::ctypes::c_uint;
pub type __kernel_gid32_t = crate::ctypes::c_uint;
pub type __kernel_old_uid_t = __kernel_uid_t;
pub type __kernel_old_gid_t = __kernel_gid_t;
pub type __kernel_size_t = crate::ctypes::c_uint;
pub type __kernel_ssize_t = crate::ctypes::c_int;
pub type __kernel_ptrdiff_t = crate::ctypes::c_int;
pub type __kernel_off_t = __kernel_long_t;
pub type __kernel_loff_t = crate::ctypes::c_longlong;
pub type __kernel_old_time_t = __kernel_long_t;
pub type __kernel_time_t = __kernel_long_t;
pub type __kernel_time64_t = crate::ctypes::c_longlong;
pub type __kernel_clock_t = __kernel_long_t;
pub type __kernel_timer_t = crate::ctypes::c_int;
pub type __kernel_clockid_t = crate::ctypes::c_int;
pub type __kernel_caddr_t = *mut crate::ctypes::c_char;
pub type __kernel_uid16_t = crate::ctypes::c_ushort;
pub type __kernel_gid16_t = crate::ctypes::c_ushort;
pub type __le16 = __u16;
pub type __be16 = __u16;
pub type __le32 = __u32;
pub type __be32 = __u32;
pub type __le64 = __u64;
pub type __be64 = __u64;
pub type __sum16 = __u16;
pub type __wsum = __u32;
pub type __poll_t = crate::ctypes::c_uint;
pub type apm_event_t = crate::ctypes::c_ushort;
pub type apm_eventinfo_t = crate::ctypes::c_ushort;
#[repr(C)]
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::core::marker::PhantomData<T>, [T; 0]);
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct screen_info {
pub orig_x: __u8,
pub orig_y: __u8,
pub ext_mem_k: __u16,
pub orig_video_page: __u16,
pub orig_video_mode: __u8,
pub orig_video_cols: __u8,
pub flags: __u8,
pub unused2: __u8,
pub orig_video_ega_bx: __u16,
pub unused3: __u16,
pub orig_video_lines: __u8,
pub orig_video_isVGA: __u8,
pub orig_video_points: __u16,
pub lfb_width: __u16,
pub lfb_height: __u16,
pub lfb_depth: __u16,
pub lfb_base: __u32,
pub lfb_size: __u32,
pub cl_magic: __u16,
pub cl_offset: __u16,
pub lfb_linelength: __u16,
pub red_size: __u8,
pub red_pos: __u8,
pub green_size: __u8,
pub green_pos: __u8,
pub blue_size: __u8,
pub blue_pos: __u8,
pub rsvd_size: __u8,
pub rsvd_pos: __u8,
pub vesapm_seg: __u16,
pub vesapm_off: __u16,
pub pages: __u16,
pub vesa_attributes: __u16,
pub capabilities: __u32,
pub ext_lfb_base: __u32,
pub _reserved: [__u8; 2usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct apm_bios_info {
pub version: __u16,
pub cseg: __u16,
pub offset: __u32,
pub cseg_16: __u16,
pub dseg: __u16,
pub flags: __u16,
pub cseg_len: __u16,
pub cseg_16_len: __u16,
pub dseg_len: __u16,
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct edd_device_params {
pub length: __u16,
pub info_flags: __u16,
pub num_default_cylinders: __u32,
pub num_default_heads: __u32,
pub sectors_per_track: __u32,
pub number_of_sectors: __u64,
pub bytes_per_sector: __u16,
pub dpte_ptr: __u32,
pub key: __u16,
pub device_path_info_length: __u8,
pub reserved2: __u8,
pub reserved3: __u16,
pub host_bus_type: [__u8; 4usize],
pub interface_type: [__u8; 8usize],
pub interface_path: edd_device_params__bindgen_ty_1,
pub device_path: edd_device_params__bindgen_ty_2,
pub reserved4: __u8,
pub checksum: __u8,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct edd_device_params__bindgen_ty_1__bindgen_ty_1 {
pub base_address: __u16,
pub reserved1: __u16,
pub reserved2: __u32,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct edd_device_params__bindgen_ty_1__bindgen_ty_2 {
pub bus: __u8,
pub slot: __u8,
pub function: __u8,
pub channel: __u8,
pub reserved: __u32,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct edd_device_params__bindgen_ty_1__bindgen_ty_3 {
pub reserved: __u64,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct edd_device_params__bindgen_ty_1__bindgen_ty_4 {
pub reserved: __u64,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct edd_device_params__bindgen_ty_1__bindgen_ty_5 {
pub reserved: __u64,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct edd_device_params__bindgen_ty_1__bindgen_ty_6 {
pub reserved: __u64,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct edd_device_params__bindgen_ty_2__bindgen_ty_1 {
pub device: __u8,
pub reserved1: __u8,
pub reserved2: __u16,
pub reserved3: __u32,
pub reserved4: __u64,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct edd_device_params__bindgen_ty_2__bindgen_ty_2 {
pub device: __u8,
pub lun: __u8,
pub reserved1: __u8,
pub reserved2: __u8,
pub reserved3: __u32,
pub reserved4: __u64,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct edd_device_params__bindgen_ty_2__bindgen_ty_3 {
pub id: __u16,
pub lun: __u64,
pub reserved1: __u16,
pub reserved2: __u32,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct edd_device_params__bindgen_ty_2__bindgen_ty_4 {
pub serial_number: __u64,
pub reserved: __u64,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct edd_device_params__bindgen_ty_2__bindgen_ty_5 {
pub eui: __u64,
pub reserved: __u64,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct edd_device_params__bindgen_ty_2__bindgen_ty_6 {
pub wwid: __u64,
pub lun: __u64,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct edd_device_params__bindgen_ty_2__bindgen_ty_7 {
pub identity_tag: __u64,
pub reserved: __u64,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct edd_device_params__bindgen_ty_2__bindgen_ty_8 {
pub array_number: __u32,
pub reserved1: __u32,
pub reserved2: __u64,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct edd_device_params__bindgen_ty_2__bindgen_ty_9 {
pub device: __u8,
pub reserved1: __u8,
pub reserved2: __u16,
pub reserved3: __u32,
pub reserved4: __u64,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct edd_device_params__bindgen_ty_2__bindgen_ty_10 {
pub reserved1: __u64,
pub reserved2: __u64,
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct edd_info {
pub device: __u8,
pub version: __u8,
pub interface_support: __u16,
pub legacy_max_cylinder: __u16,
pub legacy_max_head: __u8,
pub legacy_sectors_per_track: __u8,
pub params: edd_device_params,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct edd {
pub mbr_signature: [crate::ctypes::c_uint; 16usize],
pub edd_info: [edd_info; 6usize],
pub mbr_signature_nr: crate::ctypes::c_uchar,
pub edd_info_nr: crate::ctypes::c_uchar,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ist_info {
pub signature: __u32,
pub command: __u32,
pub event: __u32,
pub perf_level: __u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct edid_info {
pub dummy: [crate::ctypes::c_uchar; 128usize],
}
#[repr(C)]
#[derive(Debug)]
pub struct setup_data {
pub next: __u64,
pub type_: __u32,
pub len: __u32,
pub data: __IncompleteArrayField<__u8>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct setup_indirect {
pub type_: __u32,
pub reserved: __u32,
pub len: __u64,
pub addr: __u64,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct setup_header {
pub setup_sects: __u8,
pub root_flags: __u16,
pub syssize: __u32,
pub ram_size: __u16,
pub vid_mode: __u16,
pub root_dev: __u16,
pub boot_flag: __u16,
pub jump: __u16,
pub header: __u32,
pub version: __u16,
pub realmode_swtch: __u32,
pub start_sys_seg: __u16,
pub kernel_version: __u16,
pub type_of_loader: __u8,
pub loadflags: __u8,
pub setup_move_size: __u16,
pub code32_start: __u32,
pub ramdisk_image: __u32,
pub ramdisk_size: __u32,
pub bootsect_kludge: __u32,
pub heap_end_ptr: __u16,
pub ext_loader_ver: __u8,
pub ext_loader_type: __u8,
pub cmd_line_ptr: __u32,
pub initrd_addr_max: __u32,
pub kernel_alignment: __u32,
pub relocatable_kernel: __u8,
pub min_alignment: __u8,
pub xloadflags: __u16,
pub cmdline_size: __u32,
pub hardware_subarch: __u32,
pub hardware_subarch_data: __u64,
pub payload_offset: __u32,
pub payload_length: __u32,
pub setup_data: __u64,
pub pref_address: __u64,
pub init_size: __u32,
pub handover_offset: __u32,
pub kernel_info_offset: __u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sys_desc_table {
pub length: __u16,
pub table: [__u8; 14usize],
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct olpc_ofw_header {
pub ofw_magic: __u32,
pub ofw_version: __u32,
pub cif_handler: __u32,
pub irq_desc_table: __u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct efi_info {
pub efi_loader_signature: __u32,
pub efi_systab: __u32,
pub efi_memdesc_size: __u32,
pub efi_memdesc_version: __u32,
pub efi_memmap: __u32,
pub efi_memmap_size: __u32,
pub efi_systab_hi: __u32,
pub efi_memmap_hi: __u32,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct boot_e820_entry {
pub addr: __u64,
pub size: __u64,
pub type_: __u32,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct jailhouse_setup_data {
pub hdr: jailhouse_setup_data__bindgen_ty_1,
pub v1: jailhouse_setup_data__bindgen_ty_2,
pub v2: jailhouse_setup_data__bindgen_ty_3,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct jailhouse_setup_data__bindgen_ty_1 {
pub version: __u16,
pub compatible_version: __u16,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct jailhouse_setup_data__bindgen_ty_2 {
pub pm_timer_address: __u16,
pub num_cpus: __u16,
pub pci_mmconfig_base: __u64,
pub tsc_khz: __u32,
pub apic_khz: __u32,
pub standard_ioapic: __u8,
pub cpu_ids: [__u8; 255usize],
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct jailhouse_setup_data__bindgen_ty_3 {
pub flags: __u32,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ima_setup_data {
pub addr: __u64,
pub size: __u64,
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct boot_params {
pub screen_info: screen_info,
pub apm_bios_info: apm_bios_info,
pub _pad2: [__u8; 4usize],
pub tboot_addr: __u64,
pub ist_info: ist_info,
pub acpi_rsdp_addr: __u64,
pub _pad3: [__u8; 8usize],
pub hd0_info: [__u8; 16usize],
pub hd1_info: [__u8; 16usize],
pub sys_desc_table: sys_desc_table,
pub olpc_ofw_header: olpc_ofw_header,
pub ext_ramdisk_image: __u32,
pub ext_ramdisk_size: __u32,
pub ext_cmd_line_ptr: __u32,
pub _pad4: [__u8; 112usize],
pub cc_blob_address: __u32,
pub edid_info: edid_info,
pub efi_info: efi_info,
pub alt_mem_k: __u32,
pub scratch: __u32,
pub e820_entries: __u8,
pub eddbuf_entries: __u8,
pub edd_mbr_sig_buf_entries: __u8,
pub kbd_status: __u8,
pub secure_boot: __u8,
pub _pad5: [__u8; 2usize],
pub sentinel: __u8,
pub _pad6: [__u8; 1usize],
pub hdr: setup_header,
pub _pad7: [__u8; 36usize],
pub edd_mbr_sig_buffer: [__u32; 16usize],
pub e820_table: [boot_e820_entry; 128usize],
pub _pad8: [__u8; 48usize],
pub eddbuf: [edd_info; 6usize],
pub _pad9: [__u8; 276usize],
}
pub const SETUP_NONE: u32 = 0;
pub const SETUP_E820_EXT: u32 = 1;
pub const SETUP_DTB: u32 = 2;
pub const SETUP_PCI: u32 = 3;
pub const SETUP_EFI: u32 = 4;
pub const SETUP_APPLE_PROPERTIES: u32 = 5;
pub const SETUP_JAILHOUSE: u32 = 6;
pub const SETUP_CC_BLOB: u32 = 7;
pub const SETUP_IMA: u32 = 8;
pub const SETUP_RNG_SEED: u32 = 9;
pub const SETUP_ENUM_MAX: u32 = 9;
pub const SETUP_INDIRECT: u32 = 2147483648;
pub const SETUP_TYPE_MAX: u32 = 2147483657;
pub const RAMDISK_IMAGE_START_MASK: u32 = 2047;
pub const RAMDISK_PROMPT_FLAG: u32 = 32768;
pub const RAMDISK_LOAD_FLAG: u32 = 16384;
pub const LOADED_HIGH: u32 = 1;
pub const KASLR_FLAG: u32 = 2;
pub const QUIET_FLAG: u32 = 32;
pub const KEEP_SEGMENTS: u32 = 64;
pub const CAN_USE_HEAP: u32 = 128;
pub const XLF_KERNEL_64: u32 = 1;
pub const XLF_CAN_BE_LOADED_ABOVE_4G: u32 = 2;
pub const XLF_EFI_HANDOVER_32: u32 = 4;
pub const XLF_EFI_HANDOVER_64: u32 = 8;
pub const XLF_EFI_KEXEC: u32 = 16;
pub const XLF_5LEVEL: u32 = 32;
pub const XLF_5LEVEL_ENABLED: u32 = 64;
pub const VIDEO_TYPE_MDA: u32 = 16;
pub const VIDEO_TYPE_CGA: u32 = 17;
pub const VIDEO_TYPE_EGAM: u32 = 32;
pub const VIDEO_TYPE_EGAC: u32 = 33;
pub const VIDEO_TYPE_VGAC: u32 = 34;
pub const VIDEO_TYPE_VLFB: u32 = 35;
pub const VIDEO_TYPE_PICA_S3: u32 = 48;
pub const VIDEO_TYPE_MIPS_G364: u32 = 49;
pub const VIDEO_TYPE_SGI: u32 = 51;
pub const VIDEO_TYPE_TGAC: u32 = 64;
pub const VIDEO_TYPE_SUN: u32 = 80;
pub const VIDEO_TYPE_SUNPCI: u32 = 81;
pub const VIDEO_TYPE_PMAC: u32 = 96;
pub const VIDEO_TYPE_EFI: u32 = 112;
pub const VIDEO_FLAGS_NOCURSOR: u32 = 1;
pub const VIDEO_CAPABILITY_SKIP_QUIRKS: u32 = 1;
pub const VIDEO_CAPABILITY_64BIT_BASE: u32 = 2;
pub const APM_STATE_READY: u32 = 0;
pub const APM_STATE_STANDBY: u32 = 1;
pub const APM_STATE_SUSPEND: u32 = 2;
pub const APM_STATE_OFF: u32 = 3;
pub const APM_STATE_BUSY: u32 = 4;
pub const APM_STATE_REJECT: u32 = 5;
pub const APM_STATE_OEM_SYS: u32 = 32;
pub const APM_STATE_OEM_DEV: u32 = 64;
pub const APM_STATE_DISABLE: u32 = 0;
pub const APM_STATE_ENABLE: u32 = 1;
pub const APM_STATE_DISENGAGE: u32 = 0;
pub const APM_STATE_ENGAGE: u32 = 1;
pub const APM_SYS_STANDBY: u32 = 1;
pub const APM_SYS_SUSPEND: u32 = 2;
pub const APM_NORMAL_RESUME: u32 = 3;
pub const APM_CRITICAL_RESUME: u32 = 4;
pub const APM_LOW_BATTERY: u32 = 5;
pub const APM_POWER_STATUS_CHANGE: u32 = 6;
pub const APM_UPDATE_TIME: u32 = 7;
pub const APM_CRITICAL_SUSPEND: u32 = 8;
pub const APM_USER_STANDBY: u32 = 9;
pub const APM_USER_SUSPEND: u32 = 10;
pub const APM_STANDBY_RESUME: u32 = 11;
pub const APM_CAPABILITY_CHANGE: u32 = 12;
pub const APM_USER_HIBERNATION: u32 = 13;
pub const APM_HIBERNATION_RESUME: u32 = 14;
pub const APM_SUCCESS: u32 = 0;
pub const APM_DISABLED: u32 = 1;
pub const APM_CONNECTED: u32 = 2;
pub const APM_NOT_CONNECTED: u32 = 3;
pub const APM_16_CONNECTED: u32 = 5;
pub const APM_16_UNSUPPORTED: u32 = 6;
pub const APM_32_CONNECTED: u32 = 7;
pub const APM_32_UNSUPPORTED: u32 = 8;
pub const APM_BAD_DEVICE: u32 = 9;
pub const APM_BAD_PARAM: u32 = 10;
pub const APM_NOT_ENGAGED: u32 = 11;
pub const APM_BAD_FUNCTION: u32 = 12;
pub const APM_RESUME_DISABLED: u32 = 13;
pub const APM_NO_ERROR: u32 = 83;
pub const APM_BAD_STATE: u32 = 96;
pub const APM_NO_EVENTS: u32 = 128;
pub const APM_NOT_PRESENT: u32 = 134;
pub const APM_DEVICE_BIOS: u32 = 0;
pub const APM_DEVICE_ALL: u32 = 1;
pub const APM_DEVICE_DISPLAY: u32 = 256;
pub const APM_DEVICE_STORAGE: u32 = 512;
pub const APM_DEVICE_PARALLEL: u32 = 768;
pub const APM_DEVICE_SERIAL: u32 = 1024;
pub const APM_DEVICE_NETWORK: u32 = 1280;
pub const APM_DEVICE_PCMCIA: u32 = 1536;
pub const APM_DEVICE_BATTERY: u32 = 32768;
pub const APM_DEVICE_OEM: u32 = 57344;
pub const APM_DEVICE_OLD_ALL: u32 = 65535;
pub const APM_DEVICE_CLASS: u32 = 255;
pub const APM_DEVICE_MASK: u32 = 65280;
pub const APM_MAX_BATTERIES: u32 = 2;
pub const APM_CAP_GLOBAL_STANDBY: u32 = 1;
pub const APM_CAP_GLOBAL_SUSPEND: u32 = 2;
pub const APM_CAP_RESUME_STANDBY_TIMER: u32 = 4;
pub const APM_CAP_RESUME_SUSPEND_TIMER: u32 = 8;
pub const APM_CAP_RESUME_STANDBY_RING: u32 = 16;
pub const APM_CAP_RESUME_SUSPEND_RING: u32 = 32;
pub const APM_CAP_RESUME_STANDBY_PCMCIA: u32 = 64;
pub const APM_CAP_RESUME_SUSPEND_PCMCIA: u32 = 128;
pub const _IOC_NRBITS: u32 = 8;
pub const _IOC_TYPEBITS: u32 = 8;
pub const _IOC_SIZEBITS: u32 = 14;
pub const _IOC_DIRBITS: u32 = 2;
pub const _IOC_NRMASK: u32 = 255;
pub const _IOC_TYPEMASK: u32 = 255;
pub const _IOC_SIZEMASK: u32 = 16383;
pub const _IOC_DIRMASK: u32 = 3;
pub const _IOC_NRSHIFT: u32 = 0;
pub const _IOC_TYPESHIFT: u32 = 8;
pub const _IOC_SIZESHIFT: u32 = 16;
pub const _IOC_DIRSHIFT: u32 = 30;
pub const _IOC_NONE: u32 = 0;
pub const _IOC_WRITE: u32 = 1;
pub const _IOC_READ: u32 = 2;
pub const IOC_IN: u32 = 1073741824;
pub const IOC_OUT: u32 = 2147483648;
pub const IOC_INOUT: u32 = 3221225472;
pub const IOCSIZE_MASK: u32 = 1073676288;
pub const IOCSIZE_SHIFT: u32 = 16;
pub const EDDNR: u32 = 489;
pub const EDDBUF: u32 = 3328;
pub const EDDMAXNR: u32 = 6;
pub const EDDEXTSIZE: u32 = 8;
pub const EDDPARMSIZE: u32 = 74;
pub const CHECKEXTENSIONSPRESENT: u32 = 65;
pub const GETDEVICEPARAMETERS: u32 = 72;
pub const LEGACYGETDEVICEPARAMETERS: u32 = 8;
pub const EDDMAGIC1: u32 = 21930;
pub const EDDMAGIC2: u32 = 43605;
pub const READ_SECTORS: u32 = 2;
pub const EDD_MBR_SIG_OFFSET: u32 = 440;
pub const EDD_MBR_SIG_BUF: u32 = 656;
pub const EDD_MBR_SIG_MAX: u32 = 16;
pub const EDD_MBR_SIG_NR_BUF: u32 = 490;
pub const EDD_EXT_FIXED_DISK_ACCESS: u32 = 1;
pub const EDD_EXT_DEVICE_LOCKING_AND_EJECTING: u32 = 2;
pub const EDD_EXT_ENHANCED_DISK_DRIVE_SUPPORT: u32 = 4;
pub const EDD_EXT_64BIT_EXTENSIONS: u32 = 8;
pub const EDD_INFO_DMA_BOUNDARY_ERROR_TRANSPARENT: u32 = 1;
pub const EDD_INFO_GEOMETRY_VALID: u32 = 2;
pub const EDD_INFO_REMOVABLE: u32 = 4;
pub const EDD_INFO_WRITE_VERIFY: u32 = 8;
pub const EDD_INFO_MEDIA_CHANGE_NOTIFICATION: u32 = 16;
pub const EDD_INFO_LOCKABLE: u32 = 32;
pub const EDD_INFO_NO_MEDIA_PRESENT: u32 = 64;
pub const EDD_INFO_USE_INT13_FN50: u32 = 128;
pub const E820_MAX_ENTRIES_ZEROPAGE: u32 = 128;
pub const JAILHOUSE_SETUP_REQUIRED_VERSION: u32 = 1;
#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum x86_hardware_subarch {
X86_SUBARCH_PC = 0,
X86_SUBARCH_LGUEST = 1,
X86_SUBARCH_XEN = 2,
X86_SUBARCH_INTEL_MID = 3,
X86_SUBARCH_CE4100 = 4,
X86_NR_SUBARCHS = 5,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union edd_device_params__bindgen_ty_1 {
pub isa: edd_device_params__bindgen_ty_1__bindgen_ty_1,
pub pci: edd_device_params__bindgen_ty_1__bindgen_ty_2,
pub ibnd: edd_device_params__bindgen_ty_1__bindgen_ty_3,
pub xprs: edd_device_params__bindgen_ty_1__bindgen_ty_4,
pub htpt: edd_device_params__bindgen_ty_1__bindgen_ty_5,
pub unknown: edd_device_params__bindgen_ty_1__bindgen_ty_6,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union edd_device_params__bindgen_ty_2 {
pub ata: edd_device_params__bindgen_ty_2__bindgen_ty_1,
pub atapi: edd_device_params__bindgen_ty_2__bindgen_ty_2,
pub scsi: edd_device_params__bindgen_ty_2__bindgen_ty_3,
pub usb: edd_device_params__bindgen_ty_2__bindgen_ty_4,
pub i1394: edd_device_params__bindgen_ty_2__bindgen_ty_5,
pub fibre: edd_device_params__bindgen_ty_2__bindgen_ty_6,
pub i2o: edd_device_params__bindgen_ty_2__bindgen_ty_7,
pub raid: edd_device_params__bindgen_ty_2__bindgen_ty_8,
pub sata: edd_device_params__bindgen_ty_2__bindgen_ty_9,
pub unknown: edd_device_params__bindgen_ty_2__bindgen_ty_10,
}
impl<T> __IncompleteArrayField<T> {
#[inline]
pub const fn new() -> Self {
__IncompleteArrayField(::core::marker::PhantomData, [])
}
#[inline]
pub fn as_ptr(&self) -> *const T {
self as *const _ as *const T
}
#[inline]
pub fn as_mut_ptr(&mut self) -> *mut T {
self as *mut _ as *mut T
}
#[inline]
pub unsafe fn as_slice(&self, len: usize) -> &[T] {
::core::slice::from_raw_parts(self.as_ptr(), len)
}
#[inline]
pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
::core::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
}
}
impl<T> ::core::fmt::Debug for __IncompleteArrayField<T> {
fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
fmt.write_str("__IncompleteArrayField")
}
}
