use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
    process::{Command, ExitCode},
};

fn main() -> ExitCode {
    // Run `bun run build` to compile Tailwind CSS.
    // Write directly to /dev/tty to bypass cargo's stdout/stderr capture.
    let output = Command::new("bun")
        .args(["run", "build"])
        .stdin(std::process::Stdio::null())
        .output()
        .expect("failed to run bun");

    if let Ok(mut tty) = OpenOptions::new().write(true).open("/dev/tty") {
        let color = if output.status.success() {
            "\x1b[32m"
        } else {
            "\x1b[31m"
        };
        for line in String::from_utf8_lossy(&output.stdout).lines() {
            writeln!(tty, "{color}[bun]\x1b[0m {line}").ok();
        }
        for line in String::from_utf8_lossy(&output.stderr).lines() {
            writeln!(tty, "{color}[bun]\x1b[0m {line}").ok();
        }
    }

    if !output.status.success() {
        return output
            .status
            .code()
            .map(|c| ExitCode::from(c as u8))
            .unwrap_or(ExitCode::FAILURE);
    }

    println!("cargo:rerun-if-changed=assets/css/app.css");
    println!("cargo:rerun-if-changed=package.json");

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
