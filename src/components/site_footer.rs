use leptos::prelude::*;

#[component]
pub fn SiteFooter() -> impl IntoView {
    view! {
        <footer class="footer-row">
            <p class="footer-text">
                <strong>Disclaimer:</strong> This tool is intended for authorized testing and educational use only.
                Unauthorized use is strictly prohibited. The author assumes no liability for misuse or damage caused by this software.
                Use responsibly and comply with all applicable laws.
            </p>
            <a
            class="theme-toggle"
            href="https://github.com/justmike1/srok"
            target="_blank"
            rel="noopener noreferrer"
            >
                {"\u{2B50} Star on GitHub"}  // ⭐
            </a>
        </footer>
    }
}
