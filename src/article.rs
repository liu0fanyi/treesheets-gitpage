use leptos::prelude::*;
use leptos_meta::*;

// 引入构建脚本生成的文件列表和内容
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
pub fn ArticleView() -> impl IntoView {
    let search = leptos_router::hooks::use_query_map();
    let name = move || {
        search
            .get()
            .get("name")
            .map(|s| s.to_string())
            .unwrap_or_default()
    };

    // 查找文章内容
    let content = move || {
        let name = name();
        DOC_CONTENTS
            .iter()
            .find(|(filename, _)| filename.trim_end_matches(".html") == name)
            .map(|(_, content)| content.to_string())
    };

    let display_title = move || name().replace(['-', '_'], " ");

    view! {
        <Title text=move || format!("{} - 食谱", display_title())/>
        <div class="bg-gradient-to-tl from-blue-800 to-blue-500 text-white font-mono min-h-screen">
            {/* 导航栏 */}
            <nav class="bg-blue-700 shadow-lg">
                <div class="max-w-6xl mx-auto px-4">
                    <div class="flex justify-between items-center py-4">
                        <a href=base_path() class="text-xl font-bold hover:text-blue-200 transition-colors">
                            "咕咕"
                        </a>
                        <a href=base_path() class="text-blue-200 hover:text-white transition-colors">
                            "返回首页"
                        </a>
                    </div>
                </div>
            </nav>

            {/* 文章内容 */}
            <main class="max-w-6xl mx-auto px-4 py-8">
                <Show
                    when=move || content().is_some()
                    fallback=|| view! {
                        <div class="text-center py-12">
                            <h1 class="text-4xl font-bold mb-4">"文章未找到"</h1>
                            <p class="text-xl text-blue-200 mb-6">"抱歉，您请求的文章不存在。"</p>
                            <a href=base_path() class="inline-block px-6 py-3 bg-blue-600 hover:bg-blue-700 rounded transition-colors">
                                "返回首页"
                            </a>
                        </div>
                    }
                >
                    <article class="bg-white/10 backdrop-blur-sm rounded-lg p-6 md:p-8">
                        <h1 class="text-3xl md:text-4xl font-bold mb-6 text-center">
                            {display_title}
                        </h1>
                        <div class="prose prose-invert max-w-none">
                            <div inner_html=move || content().unwrap_or_default()></div>
                        </div>
                    </article>
                </Show>
            </main>

            {/* 页脚 */}
            <footer class="bg-blue-900 mt-12">
                <div class="max-w-6xl mx-auto px-4 py-6 text-center">
                    <p class="text-blue-200">"食谱收集"</p>
                </div>
            </footer>
        </div>
    }
}
