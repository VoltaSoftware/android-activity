#![allow(dead_code)]

fn build_glue_for_game_activity() {
    for f in [
        "GameActivity.h",
        "GameActivity.cpp",
        "GameActivityEvents.h",
        "GameActivityEvents.cpp",
        "GameActivityLog.h",
    ] {
        println!("cargo:rerun-if-changed=game-activity-csrc/game-activity/{f}");
    }
    cc::Build::new()
        .cpp(true)
        .include("game-activity-csrc")
        .file("game-activity-csrc/game-activity/GameActivity.cpp")
        .file("game-activity-csrc/game-activity/GameActivityEvents.cpp")
        .extra_warnings(false)
        .cpp_link_stdlib("c++_static")
        .compile("libgame_activity.a");

    for f in ["gamecommon.h", "gametextinput.h", "gametextinput.cpp"] {
        println!("cargo:rerun-if-changed=game-activity-csrc/game-text-input/{f}");
    }
    cc::Build::new()
        .cpp(true)
        .include("game-activity-csrc")
        .file("game-activity-csrc/game-text-input/gametextinput.cpp")
        .cpp_link_stdlib("c++_static")
        .compile("libgame_text_input.a");

    for f in ["android_native_app_glue.h", "android_native_app_glue.c"] {
        println!("cargo:rerun-if-changed=game-activity-csrc/native_app_glue/{f}");
    }
    cc::Build::new()
        .include("game-activity-csrc")
        .include("game-activity-csrc/game-activity/native_app_glue")
        .file("game-activity-csrc/game-activity/native_app_glue/android_native_app_glue.c")
        .extra_warnings(false)
        .cpp_link_stdlib("c++_static")
        .compile("libnative_app_glue.a");

    // We need to link to both c++_static and c++abi for the static C++ library.
    // Ideally we'd link directly to libc++.a.
    println!("cargo:rustc-link-lib=c++abi");
}

fn main() {
    let os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    if os != "android" {
        // For some reason using target_os in the cfg attribute breaks my app at runtime
        // We ignore this so we dont have a failed build command. We don't build android projects on windows unless we use the cargo ndk toolchain.
        return;
    }
    #[cfg(all(feature = "game-activity"))]
    build_glue_for_game_activity();
}
