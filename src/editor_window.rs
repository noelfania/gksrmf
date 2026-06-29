// 에디터 창 생성 및 리사이즈/스크롤 관리 모듈

use windows::{
    core::*,
    Win32::{
        Foundation::*,
        System::LibraryLoader::GetModuleHandleW,
        UI::WindowsAndMessaging::*,
    },
};

pub const EDIT_OUTER_MARGIN: i32 = 8;
pub const STATUS_BAR_HEIGHT: i32 = 42;
pub const IDC_ALWAYS_ON_TOP_CHECK: u32 = 2001;
pub const IDC_MODE_HAN: u32 = 2002;
pub const IDC_MODE_ENG: u32 = 2003;
const SS_ETCHEDFRAME_RAW: u32 = 0x0000_0012;

pub struct EditorControls {
    pub hwnd_status_bar: HWND,
    pub hwnd_mode_group: HWND,
    pub hwnd_mode_group_label: HWND,
    pub hwnd_mode_han: HWND,
    pub hwnd_mode_eng: HWND,
    pub hwnd_topmost_check: HWND,
    pub hwnd_edit: HWND,
}

unsafe fn create_status_bar(parent: HWND, hinstance: HINSTANCE) -> HWND {
    CreateWindowExW(
        WINDOW_EX_STYLE(0),
        w!("STATIC"),
        None,
        WS_CHILD | WS_VISIBLE | WS_CLIPSIBLINGS,
        0,
        0,
        1,
        STATUS_BAR_HEIGHT,
        parent,
        None,
        hinstance,
        None,
    )
    .unwrap_or(HWND(0 as _))
}

unsafe fn create_mode_group(parent: HWND, hinstance: HINSTANCE) -> HWND {
    CreateWindowExW(
        WINDOW_EX_STYLE(0),
        w!("STATIC"),
        None,
        WS_CHILD | WS_VISIBLE | WINDOW_STYLE(SS_ETCHEDFRAME_RAW),
        EDIT_OUTER_MARGIN,
        10,
        108,
        STATUS_BAR_HEIGHT - 16,
        parent,
        None,
        hinstance,
        None,
    )
    .unwrap_or(HWND(0 as _))
}

unsafe fn create_mode_group_label(parent: HWND, hinstance: HINSTANCE) -> HWND {
    CreateWindowExW(
        WINDOW_EX_STYLE(0),
        w!("STATIC"),
        w!("F1"),
        WS_CHILD | WS_VISIBLE,
        EDIT_OUTER_MARGIN + 8,
        2,
        22,
        16,
        parent,
        None,
        hinstance,
        None,
    )
    .unwrap_or(HWND(0 as _))
}

unsafe fn create_mode_han_radio(parent: HWND, hinstance: HINSTANCE) -> HWND {
    CreateWindowExW(
        WINDOW_EX_STYLE(0),
        w!("BUTTON"),
        w!("ㅎ"),
        WS_CHILD
            | WS_VISIBLE
            | WS_TABSTOP
            | WINDOW_STYLE(BS_AUTORADIOBUTTON as u32)
            | WINDOW_STYLE(WS_GROUP.0),
        EDIT_OUTER_MARGIN + 20,
        EDIT_OUTER_MARGIN + 8,
        40,
        STATUS_BAR_HEIGHT - EDIT_OUTER_MARGIN - 8,
        parent,
        HMENU(IDC_MODE_HAN as usize as *mut core::ffi::c_void),
        hinstance,
        None,
    )
    .unwrap_or(HWND(0 as _))
}

unsafe fn create_mode_eng_radio(parent: HWND, hinstance: HINSTANCE) -> HWND {
    CreateWindowExW(
        WINDOW_EX_STYLE(0),
        w!("BUTTON"),
        w!("A"),
        WS_CHILD
            | WS_VISIBLE
            | WS_TABSTOP
            | WINDOW_STYLE(BS_AUTORADIOBUTTON as u32),
        EDIT_OUTER_MARGIN + 60,
        EDIT_OUTER_MARGIN + 8,
        40,
        STATUS_BAR_HEIGHT - EDIT_OUTER_MARGIN - 8,
        parent,
        HMENU(IDC_MODE_ENG as usize as *mut core::ffi::c_void),
        hinstance,
        None,
    )
    .unwrap_or(HWND(0 as _))
}

unsafe fn create_topmost_checkbox(parent: HWND, hinstance: HINSTANCE) -> HWND {
    CreateWindowExW(
        WINDOW_EX_STYLE(0),
        w!("BUTTON"),
        w!("Always on Top"),
        WS_CHILD
            | WS_VISIBLE
            | WS_TABSTOP
            | WINDOW_STYLE(BS_AUTOCHECKBOX as u32),
        0,
        EDIT_OUTER_MARGIN,
        140,
        STATUS_BAR_HEIGHT - EDIT_OUTER_MARGIN,
        parent,
        HMENU(IDC_ALWAYS_ON_TOP_CHECK as usize as *mut core::ffi::c_void),
        hinstance,
        None,
    )
    .unwrap_or(HWND(0 as _))
}

/// 상태바와 에디터 컨트롤을 생성한다.
pub unsafe fn create_editor_controls(parent: HWND) -> EditorControls {
    let mut rect = RECT::default();
    GetClientRect(parent, &mut rect).ok();
    let hinstance: HINSTANCE = GetModuleHandleW(None).unwrap().into();

    let hwnd_status_bar = create_status_bar(parent, hinstance);

    let hedit = CreateWindowExW(
        WINDOW_EX_STYLE(0),
        w!("EDIT"),
        None,
        WS_CHILD
            | WS_VISIBLE
            | WS_CLIPSIBLINGS
            | WS_HSCROLL
            | WS_VSCROLL
            | WINDOW_STYLE(ES_MULTILINE as u32)
            | WINDOW_STYLE(ES_AUTOHSCROLL as u32)
            | WINDOW_STYLE(ES_AUTOVSCROLL as u32)
            | WINDOW_STYLE(ES_WANTRETURN as u32),
        EDIT_OUTER_MARGIN,
        STATUS_BAR_HEIGHT + EDIT_OUTER_MARGIN,
        (rect.right - EDIT_OUTER_MARGIN * 2).max(1),
        (rect.bottom - STATUS_BAR_HEIGHT - EDIT_OUTER_MARGIN * 2).max(1),
        parent,
        None,
        hinstance,
        None,
    )
    .unwrap_or(HWND(0 as _));

    let hwnd_mode_group = create_mode_group(parent, hinstance);
    let hwnd_mode_group_label = create_mode_group_label(parent, hinstance);
    let hwnd_mode_han = create_mode_han_radio(parent, hinstance);
    let hwnd_mode_eng = create_mode_eng_radio(parent, hinstance);
    let hwnd_topmost_check = create_topmost_checkbox(parent, hinstance);

    EditorControls {
        hwnd_status_bar,
        hwnd_mode_group,
        hwnd_mode_group_label,
        hwnd_mode_han,
        hwnd_mode_eng,
        hwnd_topmost_check,
        hwnd_edit: hedit,
    }
}

/// 상태바와 에디터 컨트롤 크기를 새 클라이언트 영역에 맞게 조정한다.
pub unsafe fn resize_editor_controls(
    hwnd_status_bar: HWND,
    hwnd_mode_group: HWND,
    hwnd_mode_group_label: HWND,
    hwnd_mode_han: HWND,
    hwnd_mode_eng: HWND,
    hwnd_topmost_check: HWND,
    hwnd_edit: HWND,
    width: i32,
    height: i32,
) {
    if hwnd_status_bar != HWND(0 as _) {
        SetWindowPos(
            hwnd_status_bar,
            None,
            0,
            0,
            width.max(1),
            STATUS_BAR_HEIGHT,
            SWP_NOZORDER,
        )
        .ok();
    }

    if hwnd_mode_group != HWND(0 as _) {
        SetWindowPos(
            hwnd_mode_group,
            HWND_TOP,
            EDIT_OUTER_MARGIN,
            10,
            108,
            (STATUS_BAR_HEIGHT - 16).max(24),
            SET_WINDOW_POS_FLAGS(0),
        )
        .ok();
    }

    if hwnd_mode_group_label != HWND(0 as _) {
        SetWindowPos(
            hwnd_mode_group_label,
            HWND_TOP,
            EDIT_OUTER_MARGIN + 8,
            2,
            22,
            16,
            SET_WINDOW_POS_FLAGS(0),
        )
        .ok();
    }

    if hwnd_mode_han != HWND(0 as _) {
        SetWindowPos(
            hwnd_mode_han,
            HWND_TOP,
            EDIT_OUTER_MARGIN + 20,
            EDIT_OUTER_MARGIN + 8,
            40,
            (STATUS_BAR_HEIGHT - EDIT_OUTER_MARGIN - 8).max(16),
            SET_WINDOW_POS_FLAGS(0),
        )
        .ok();
    }

    if hwnd_mode_eng != HWND(0 as _) {
        SetWindowPos(
            hwnd_mode_eng,
            HWND_TOP,
            EDIT_OUTER_MARGIN + 60,
            EDIT_OUTER_MARGIN + 8,
            40,
            (STATUS_BAR_HEIGHT - EDIT_OUTER_MARGIN - 8).max(16),
            SET_WINDOW_POS_FLAGS(0),
        )
        .ok();
    }

    if hwnd_topmost_check != HWND(0 as _) {
        let check_w = 140;
        SetWindowPos(
            hwnd_topmost_check,
            HWND_TOP,
            (width - check_w - EDIT_OUTER_MARGIN).max(EDIT_OUTER_MARGIN),
            EDIT_OUTER_MARGIN,
            check_w,
            (STATUS_BAR_HEIGHT - EDIT_OUTER_MARGIN).max(16),
            SET_WINDOW_POS_FLAGS(0),
        )
        .ok();
    }

    if hwnd_edit != HWND(0 as _) {
        SetWindowPos(
            hwnd_edit,
            None,
            EDIT_OUTER_MARGIN,
            STATUS_BAR_HEIGHT + EDIT_OUTER_MARGIN,
            (width - EDIT_OUTER_MARGIN * 2).max(1),
            (height - STATUS_BAR_HEIGHT - EDIT_OUTER_MARGIN * 2).max(1),
            SWP_NOZORDER,
        )
        .ok();
    }
}

/// 항상 위 설정을 창에 즉시 반영한다.
pub unsafe fn apply_always_on_top(hwnd: HWND, on_top: bool) {
    let insert_after = if on_top { HWND_TOPMOST } else { HWND_NOTOPMOST };
    SetWindowPos(hwnd, insert_after, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE).ok();
}
