use libbpf_cargo::SkeletonBuilder;
use std::env;
use std::path::PathBuf;

const CPROG: &str = "src/bpf/cgroup_fw.bpf.c";

fn main() {
    let mut out = PathBuf::from(env::var_os("OUT_DIR").expect("Can't infer OUT_DIR in build.rs"));
    out.push("cgroup_fw.skel.rs");
    SkeletonBuilder::new()
        .source(CPROG)
        .build_and_generate(&out)
        .unwrap();
    println!("cargo:rerun-if-changed={CPROG}");
}
