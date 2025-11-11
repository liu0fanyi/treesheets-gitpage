use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
};

// 引入构建脚本生成的文件列表
include!(concat!(env!("OUT_DIR"), "/doc_files.rs"));

// 获取基础路径
fn base_path() -> &'static str {
    if cfg!(debug_assertions) {
        ""
    } else {
        "/treesheets-gitpage"
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router base=base_path()>
            <Routes fallback=|| "Page not found.">
                <Route path=StaticSegment("") view=Home/>
                {/* 静态文件由 GitHub Pages 直接服务，无需额外路由 */}
            </Routes>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    let (value, set_value) = signal(0);

    view! {
        <Title text="Leptos + Tailwindcss"/>
        <main>
            <div class="bg-gradient-to-tl from-blue-800 to-blue-500 text-white font-mono flex flex-col min-h-screen">
                {/* 原有计数器 */}
                <div class="flex flex-row-reverse flex-wrap m-auto">
                    <button on:click=move |_| set_value.update(|v| *v += 1) class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-700 border-blue-800 text-white">
                        "+"
                    </button>
                    <button class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-800 border-blue-900 text-white">
                        {value}
                    </button>
                    <button
                        on:click=move |_| set_value.update(|v| *v -= 1)
                        class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-700 border-blue-800 text-white"
                        class:invisible=move || value.get() < 1
                    >
                        "-"
                    </button>
                </div>

                {/* 文档列表 - 添加到页面 */}
                <div class="m-auto mt-8 w-full max-w-2xl px-4">
                    <h2 class="text-2xl font-bold mb-4">Documentation Files</h2>
                    <DocList/>
                </div>
            </div>
        </main>
    }
}

#[component]
fn DocList() -> impl IntoView {
    let base = base_path();

    view! {
        <Show
            when=move || !DOC_FILES.is_empty()
            fallback=|| view! { <p class="text-blue-200">No documentation files found.</p> }
        >
            <ul class="space-y-2">
                {DOC_FILES.iter().map(|filename| {
                    // 生成完整链接（GitHub Pages 会直接服务这些静态文件）
                    let href = format!("{}/doc/{}", base, filename);
                    // 美化显示名称：移除 .html 后缀，替换 -/_ 为空格
                    let display_name = filename
                        .strip_suffix(".html")
                        .unwrap_or(filename)
                        .replace(['-', '_'], " ");

                    view! {
                        <li>
                            <a
                                href=href
                                target="_blank" // 在新标签页打开
                                class="block p-3 bg-blue-600 hover:bg-blue-700 rounded transition-colors duration-200 shadow-md"
                            >
                                {display_name}
                            </a>
                        </li>
                    }
                }).collect_view()}
            </ul>
        </Show>
    }
}
