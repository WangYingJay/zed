#[cfg(feature = "calls")]
use collab_ui::collab_panel;
use gpui::{App, Menu, MenuItem, OsAction};
use terminal_view::terminal_panel;
use zed_actions::{Quit, git_panel, project_panel};

/// B档硬删：菜单只保留本地编辑器壳 + 项目/大纲/终端/Git（+ calls 时 Collab）。
/// 只改用户可见文案；菜单项仍绑定原来的 action，快捷键不变。
pub fn app_menus(_cx: &mut App) -> Vec<Menu> {
    let mut view_items = vec![
        MenuItem::action(
            "放大",
            zed_actions::IncreaseBufferFontSize { persist: false },
        ),
        MenuItem::action(
            "缩小",
            zed_actions::DecreaseBufferFontSize { persist: false },
        ),
        MenuItem::action(
            "重置缩放",
            zed_actions::ResetBufferFontSize { persist: false },
        ),
        MenuItem::action("重置全部缩放", zed_actions::ResetAllZoom { persist: false }),
        MenuItem::separator(),
        MenuItem::action("切换左侧栏", workspace::ToggleLeftDock),
        MenuItem::action("切换右侧栏", workspace::ToggleRightDock),
        MenuItem::action("切换底部栏", workspace::ToggleBottomDock),
        MenuItem::action("切换全部侧栏", workspace::ToggleAllDocks),
        MenuItem::submenu(Menu {
            name: "编辑器布局".into(),
            disabled: false,
            items: vec![
                MenuItem::action("向上拆分", workspace::SplitUp::default()),
                MenuItem::action("向下拆分", workspace::SplitDown::default()),
                MenuItem::action("向左拆分", workspace::SplitLeft::default()),
                MenuItem::action("向右拆分", workspace::SplitRight::default()),
            ],
        }),
        MenuItem::separator(),
        MenuItem::action("项目面板", project_panel::ToggleFocus),
        MenuItem::action("大纲", outline_panel::ToggleFocus),
    ];
    #[cfg(feature = "calls")]
    view_items.push(MenuItem::action("协作面板", collab_panel::ToggleFocus));
    view_items.extend([
        MenuItem::action("终端", terminal_panel::Toggle),
        MenuItem::action("Git", git_panel::ToggleFocus),
        MenuItem::separator(),
    ]);

    vec![
        Menu {
            name: "Zed".into(),
            disabled: false,
            items: vec![
                MenuItem::action("关于 Zed", zed_actions::About),
                // B档第五刀：去掉 Check for Updates（无 auto_update）
                MenuItem::separator(),
                MenuItem::submenu(Menu::new("设置").items([
                    MenuItem::action("打开设置", zed_actions::OpenSettings),
                    MenuItem::action("打开设置文件", super::OpenSettingsFile),
                    MenuItem::action("打开项目设置", zed_actions::OpenProjectSettings),
                    MenuItem::action("打开项目设置文件", super::OpenProjectSettingsFile),
                    MenuItem::action("打开默认设置", super::OpenDefaultSettings),
                    MenuItem::separator(),
                    MenuItem::action("打开快捷键文件", zed_actions::OpenKeymapFile),
                    MenuItem::action("打开默认快捷键", zed_actions::OpenDefaultKeymap),
                    MenuItem::separator(),
                    MenuItem::action("选择主题…", zed_actions::theme_selector::Toggle::default()),
                    MenuItem::action(
                        "选择图标主题…",
                        zed_actions::icon_theme_selector::Toggle::default(),
                    ),
                ])),
                MenuItem::separator(),
                #[cfg(target_os = "macos")]
                MenuItem::os_submenu("服务", gpui::SystemMenuType::Services),
                MenuItem::separator(),
                // B档第八刀：卸 install_cli（菜单不再提供「安装命令行工具」）
                #[cfg(target_os = "macos")]
                MenuItem::action("隐藏 Zed", super::Hide),
                #[cfg(target_os = "macos")]
                MenuItem::action("隐藏其他", super::HideOthers),
                #[cfg(target_os = "macos")]
                MenuItem::action("全部显示", super::ShowAll),
                MenuItem::separator(),
                MenuItem::action("退出 Zed", Quit),
            ],
        },
        Menu {
            name: "文件".into(),
            disabled: false,
            items: vec![
                MenuItem::action("新建文件", workspace::NewFile),
                MenuItem::action("新建窗口", workspace::NewWindow),
                MenuItem::separator(),
                #[cfg(not(target_os = "macos"))]
                MenuItem::action("打开文件…", workspace::OpenFiles),
                MenuItem::action(
                    if cfg!(not(target_os = "macos")) {
                        "打开文件夹…"
                    } else {
                        "打开…"
                    },
                    workspace::Open::default(),
                ),
                MenuItem::action("打开最近使用的…", zed_actions::OpenRecent::default()),
                MenuItem::separator(),
                MenuItem::action("将文件夹添加到项目…", workspace::AddFolderToProject),
                MenuItem::separator(),
                MenuItem::action("保存", workspace::Save { save_intent: None }),
                MenuItem::action("另存为…", workspace::SaveAs),
                MenuItem::action("全部保存", workspace::SaveAll { save_intent: None }),
                MenuItem::separator(),
                MenuItem::action(
                    "关闭编辑器",
                    workspace::CloseActiveItem {
                        save_intent: None,
                        close_pinned: true,
                    },
                ),
                MenuItem::action("关闭项目", workspace::CloseProject),
                MenuItem::action("关闭窗口", workspace::CloseWindow),
            ],
        },
        Menu {
            name: "编辑".into(),
            disabled: false,
            items: vec![
                MenuItem::os_action("撤销", editor::actions::Undo, OsAction::Undo),
                MenuItem::os_action("重做", editor::actions::Redo, OsAction::Redo),
                MenuItem::separator(),
                MenuItem::os_action("剪切", editor::actions::Cut, OsAction::Cut),
                MenuItem::os_action("复制", editor::actions::Copy, OsAction::Copy),
                MenuItem::action("复制并去除空白", editor::actions::CopyAndTrim),
                MenuItem::os_action("粘贴", editor::actions::Paste, OsAction::Paste),
                MenuItem::separator(),
                MenuItem::action("查找", search::buffer_search::Deploy::find()),
                MenuItem::action("在项目中查找", workspace::DeploySearch::default()),
                MenuItem::separator(),
                MenuItem::action("切换行注释", editor::actions::ToggleComments::default()),
            ],
        },
        Menu {
            name: "选择".into(),
            disabled: false,
            items: vec![
                MenuItem::os_action("全选", editor::actions::SelectAll, OsAction::SelectAll),
                MenuItem::action("扩大选区", editor::actions::SelectLargerSyntaxNode),
                MenuItem::action("缩小选区", editor::actions::SelectSmallerSyntaxNode),
                MenuItem::action("选择下一个同级", editor::actions::SelectNextSyntaxNode),
                MenuItem::action("选择上一个同级", editor::actions::SelectPreviousSyntaxNode),
                MenuItem::separator(),
                MenuItem::action(
                    "在上方添加光标",
                    editor::actions::AddSelectionAbove {
                        skip_soft_wrap: true,
                    },
                ),
                MenuItem::action(
                    "在下方添加光标",
                    editor::actions::AddSelectionBelow {
                        skip_soft_wrap: true,
                    },
                ),
                MenuItem::action(
                    "选择下一处",
                    editor::actions::SelectNext {
                        replace_newest: false,
                    },
                ),
                MenuItem::action(
                    "选择上一处",
                    editor::actions::SelectPrevious {
                        replace_newest: false,
                    },
                ),
                MenuItem::action("选择全部匹配", editor::actions::SelectAllMatches),
                MenuItem::separator(),
                MenuItem::action("上移一行", editor::actions::MoveLineUp),
                MenuItem::action("下移一行", editor::actions::MoveLineDown),
                MenuItem::action("复制选区", editor::actions::DuplicateLineDown),
            ],
        },
        Menu {
            name: "查看".into(),
            disabled: false,
            items: view_items,
        },
        Menu {
            name: "转到".into(),
            disabled: false,
            items: vec![
                MenuItem::action("返回", workspace::GoBack),
                MenuItem::action("前进", workspace::GoForward),
                MenuItem::separator(),
                MenuItem::action("命令面板…", zed_actions::command_palette::Toggle),
                MenuItem::separator(),
                MenuItem::action("转到文件…", workspace::ToggleFileFinder::default()),
                MenuItem::action("转到编辑器中的符号…", zed_actions::outline::ToggleOutline),
                MenuItem::action("转到行/列…", editor::actions::ToggleGoToLine),
                MenuItem::separator(),
                MenuItem::action("转到定义", editor::actions::GoToDefinition::default()),
                MenuItem::action("转到声明", editor::actions::GoToDeclaration::default()),
                MenuItem::action(
                    "转到类型定义",
                    editor::actions::GoToTypeDefinition::default(),
                ),
                MenuItem::action(
                    "查找所有引用",
                    editor::actions::FindAllReferences::default(),
                ),
                MenuItem::separator(),
                MenuItem::action("下一个问题", editor::actions::GoToDiagnostic::default()),
                MenuItem::action(
                    "上一个问题",
                    editor::actions::GoToPreviousDiagnostic::default(),
                ),
            ],
        },
        Menu {
            name: "运行".into(),
            disabled: false,
            items: vec![
                MenuItem::action(
                    "运行任务",
                    zed_actions::Spawn::ViaModal {
                        reveal_target: None,
                    },
                ),
                MenuItem::separator(),
                MenuItem::action("编辑 tasks.json…", zed_actions::OpenProjectTasks),
            ],
        },
        Menu {
            // macOS 靠这个标题登记系统窗口菜单；gpui_macos 同时认 "Window" 和 "窗口"。
            name: "窗口".into(),
            disabled: false,
            items: vec![
                MenuItem::action("最小化", super::Minimize),
                MenuItem::action("缩放", super::Zoom),
                MenuItem::separator(),
            ],
        },
        Menu {
            name: "帮助".into(),
            disabled: false,
            items: vec![
                MenuItem::action("查看依赖许可证", zed_actions::OpenLicenses),
                MenuItem::separator(),
                MenuItem::action(
                    "文档",
                    super::OpenBrowser {
                        url: "https://zed.dev/docs".into(),
                    },
                ),
            ],
        },
    ]
}
