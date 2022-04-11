extern crate cc;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn source_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("xraylib")
}

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub struct Build {
    out_dir: Option<PathBuf>,
    target: Option<String>,
    host: Option<String>,
}

pub struct Artifacts {
    include_dir: PathBuf,
    lib_dir: PathBuf,
    lib: String,
}

impl Build {
    pub fn new() -> Build {
        Build {
            out_dir: env::var_os("OUT_DIR").map(|s| PathBuf::from(s).join("xraylib-src")),
            target: env::var("TARGET").ok(),
            host: env::var("HOST").ok(),
        }
    }

    pub fn out_dir<P: AsRef<Path>>(&mut self, path: P) -> &mut Build {
        self.out_dir = Some(path.as_ref().to_path_buf());
        self
    }

    pub fn target(&mut self, target: &str) -> &mut Build {
        self.target = Some(target.to_string());
        self
    }

    pub fn host(&mut self, host: &str) -> &mut Build {
        self.host = Some(host.to_string());
        self
    }

    pub fn build(&mut self) -> Artifacts {
        let target = &self.target.as_ref().expect("TARGET dir not set")[..];
        let host = &self.host.as_ref().expect("HOST dir not set")[..];
        let out_dir = self.out_dir.as_ref().expect("OUT_DIR not set");
        let build_dir = out_dir.join("build");
        let install_dir = out_dir.join("install");

        if build_dir.exists() {
            fs::remove_dir_all(&build_dir).unwrap();
        }
        fs::create_dir_all(&build_dir).unwrap();
        if install_dir.exists() {
            fs::remove_dir_all(&install_dir).unwrap();
        }
        fs::create_dir_all(&install_dir).unwrap();

        let meson_program = env::var("MESON").unwrap_or("meson".to_string());
        let mut configure = Command::new(meson_program);
        configure.arg(&format!("--prefix={}", install_dir.display()));
        configure.arg("-Ddefault_library=static");
        configure.arg("-Dpython-bindings=disabled");
        configure.arg("-Dpython-numpy-bindings=disabled");
        configure.arg(source_dir());

        let mut cc = cc::Build::new();
        cc.target(target).host(host).warnings(false).opt_level(2);
        let compiler = cc.get_compiler();
        configure.env("CC", compiler.path());

        configure.current_dir(&build_dir);
        self.run_command(configure, "configuring meson build");

        let ninja_program = env::var("NINJA").unwrap_or("ninja".to_string());
        let mut build = Command::new(ninja_program);
        build.arg("install").current_dir(&build_dir);
        self.run_command(build, "building and installing xraylib");

        let lib = if target.contains("msvc") {
            "libxrl".to_string()
        } else {
            "xrl".to_string()
        };

        Artifacts {
            lib_dir: install_dir.join("lib"),
            include_dir: install_dir.join("include"),
            lib: lib,
        }
    }

    fn run_command(&self, mut command: Command, desc: &str) {
        println!("running {:?}", command);
        let status = command.status();

        let (status_or_failed, error) = match status {
            Ok(status) if status.success() => return,
            Ok(status) => ("Exit status", format!("{}", status)),
            Err(failed) => ("Failed to execute", format!("{}", failed)),
        };
        panic!(
            "
Error {}:
    Command: {:?}
    {}: {}
    ",
            desc, command, status_or_failed, error
        );
    }
}

impl Artifacts {
    pub fn include_dir(&self) -> &Path {
        &self.include_dir
    }

    pub fn lib_dir(&self) -> &Path {
        &self.lib_dir
    }

    pub fn lib(&self) -> &String {
        &self.lib
    }

    pub fn print_cargo_metadata(&self) {
        println!("cargo:rustc-link-search=native={}", self.lib_dir.display());
        println!("cargo:rustc-link-lib=static={}", self.lib);
        println!("cargo:include={}", self.include_dir.display());
        println!("cargo:lib={}", self.lib_dir.display());
    }
}
