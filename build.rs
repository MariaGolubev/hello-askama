use std::{
    fs,
    path::Path,
    process::{Command, ExitCode},
};

fn main() -> ExitCode {
    println!("cargo:rerun-if-env-changed=SKIP_ASSET_BUILD");
    println!("cargo:rerun-if-env-changed=ASSET_BUILD_TOOL");
    println!("cargo:rerun-if-changed=assets/css/app.css");
    println!("cargo:rerun-if-changed=package.json");

    let skip_asset_build = std::env::var("SKIP_ASSET_BUILD")
        .map(|value| value == "1" || value.eq_ignore_ascii_case("true"))
        .unwrap_or(false);

    if !skip_asset_build && !run_asset_build() {
        return ExitCode::FAILURE;
    }

    let src = Path::new("assets/templates");
    let dst = Path::new("templates");

    let cfg = minify_html::Cfg {
        keep_closing_tags: true,
        keep_html_and_head_opening_tags: true,
        minify_css: false,
        minify_js: false,
        ..minify_html::Cfg::default()
    };

    process_dir(src, src, dst, &cfg);

    ExitCode::SUCCESS
}

fn run_asset_build() -> bool {
    let tool = std::env::var("ASSET_BUILD_TOOL").unwrap_or_else(|_| "bun".to_string());
    let output = Command::new(&tool)
        .args(["run", "build"])
        .stdin(std::process::Stdio::null())
        .output();

    match output {
        Ok(output) if output.status.success() => {
            for line in String::from_utf8_lossy(&output.stdout).lines() {
                println!("cargo:warning=[asset-build] {line}");
            }
            for line in String::from_utf8_lossy(&output.stderr).lines() {
                println!("cargo:warning=[asset-build] {line}");
            }
            true
        }
        Ok(output) => {
            println!(
                "cargo:warning=asset build command failed with status: {}",
                output.status
            );
            for line in String::from_utf8_lossy(&output.stderr).lines() {
                println!("cargo:warning=[asset-build] {line}");
            }
            assets_ready_on_disk()
        }
        Err(err) => {
            println!(
                "cargo:warning=failed to start asset build tool '{tool}': {err}. Set SKIP_ASSET_BUILD=1 to skip."
            );
            assets_ready_on_disk()
        }
    }
}

fn assets_ready_on_disk() -> bool {
    let css_exists = Path::new("static/css/app.css").exists();
    let htmx_exists = Path::new("static/js/htmx.min.js").exists();
    let templates_exist = Path::new("assets/templates").exists();

    if css_exists && htmx_exists && templates_exist {
        println!(
            "cargo:warning=continuing with existing static assets because build output is already present"
        );
        true
    } else {
        println!(
            "cargo:warning=missing static assets. Run 'bun run build' (or set ASSET_BUILD_TOOL) before building"
        );
        false
    }
}

fn process_dir(base: &Path, dir: &Path, dst_base: &Path, cfg: &minify_html::Cfg) {
    for entry in fs::read_dir(dir).expect("failed to read assets/templates") {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            process_dir(base, &path, dst_base, cfg);
        } else if path.extension().and_then(|e| e.to_str()) == Some("html") {
            // Emit per-file so cargo detects content changes too
            println!("cargo:rerun-if-changed={}", path.display());

            let rel = path.strip_prefix(base).unwrap();
            let dst = dst_base.join(rel);
            fs::create_dir_all(dst.parent().unwrap()).unwrap();
            let src = fs::read(&path).unwrap();
            let minified = minify_html::minify(&src, cfg);
            fs::write(dst, minified).unwrap();
        }
    }
}
