use std::process::Command;
use sysinfo::{Components, Disks, Networks, System};
use colored::*;
use users::{get_user_by_uid, get_current_uid};

static LOGO: &str = "[38;5;52m [38;5;1m [38;5;88m [38;5;160m [38;5;9m  [38;5;197m   [38;5;162m [38;5;17m'[38;5;88mxk[38;5;160mX[38;5;88mxdxdd[38;5;52md[38;5;17m,[38;5;4mo[38;5;18mdd[38;5;17mc[38;5;232m. . [38;5;0m [38;5;17m'c:[38;5;232m [38;5;0m   [38;5;232m   [38;5;0m   [38;5;232m [0;0m
[38;5;233m [38;5;52m  [38;5;124m [38;5;160m  [38;5;9m [38;5;160mM[38;5;124mM[38;5;53m0[38;5;18md[38;5;17m'[38;5;0m [38;5;52m:[38;5;88mkk[38;5;52m'[38;5;232m [38;5;17m'[38;5;4ml[38;5;18mdddx[38;5;53m0O[38;5;4mk[38;5;160mM[38;5;9mMMM[38;5;160mMM[38;5;124mXO[38;5;88mx[38;5;1md[38;5;52mx[38;5;53mKk[38;5;52m.[38;5;232m [38;5;0m  [0;0m
[38;5;232m  [38;5;233m [38;5;52m [38;5;1m [38;5;124m [38;5;160m [38;5;124mN[38;5;88mO[38;5;17m;[38;5;18mo[38;5;4mo[38;5;17m.[38;5;0m  [38;5;232m [38;5;17m.c[38;5;4mo[38;5;17m;;:c[38;5;53mx[38;5;160mM[38;5;9mMMMMMMMMMMMMMMMM[38;5;52m [38;5;232m [38;5;0m [0;0m
[38;5;232m   [38;5;233m [38;5;52m   [38;5;1md[38;5;88mx[38;5;52mc[38;5;17m;[38;5;18md[38;5;53mx[38;5;52mcc[38;5;17mc[38;5;18mo[38;5;17m:[38;5;235ml[38;5;52m,''.[38;5;233m.[38;5;232m..[38;5;124mX[38;5;9mMMMMM[38;5;52m',[38;5;53mxK0[38;5;1ml[38;5;124mk[38;5;9mMM[38;5;232m [38;5;0m  [0;0m
[38;5;232m    [38;5;233m   '[38;5;1md[38;5;124mX[38;5;52m:[38;5;17mc[38;5;160mM[38;5;9mMM[38;5;17ml[38;5;235ml[38;5;52m;[38;5;160mMM[38;5;9mMMMMMMMMM[38;5;4mkk[38;5;17m:[38;5;0m  [38;5;232m.[38;5;233m.[38;5;17m::o[38;5;53mXx[38;5;0m   [0;0m
[38;5;232m        .[38;5;9mM[38;5;1ml[38;5;52mc[38;5;9mMM[38;5;124m0[38;5;232m [38;5;9mM[38;5;124m0[38;5;0m       [38;5;232m      [38;5;17mc[38;5;4mo[38;5;17m:[38;5;0m  [38;5;232m.[38;5;233m..[38;5;17m.[38;5;232m [38;5;0m   [0;0m
[38;5;232m       [38;5;0m [38;5;88mx[38;5;9mM[38;5;232m [38;5;160mW[38;5;9mMM[38;5;52m'[38;5;1mo[38;5;9mM[38;5;52m,[38;5;0m             [38;5;232m  [38;5;17m;[38;5;4mooo[38;5;17mx[38;5;160mMM[38;5;0m    [0;0m
[38;5;0m [38;5;232m      [38;5;0m [38;5;88mO[38;5;1mo[38;5;233m.[38;5;160mM[38;5;9mM[38;5;160mM[38;5;232m [38;5;160mMM[38;5;0m              [38;5;232m    [38;5;0m   [38;5;232m  [38;5;0m    [0;0m
[38;5;0m    [38;5;232m       [38;5;235ml[38;5;89mXN[38;5;9mMM[38;5;124mW[38;5;17moc[38;5;88mK[38;5;160mMMMM[38;5;9mMMMMMMMMMMMMMM[38;5;160mM[38;5;1md[38;5;0m    [0;0m
[38;5;0m     [38;5;232m   [38;5;17m.c[38;5;4mo[38;5;53m0[38;5;160mM[38;5;9mM[38;5;160mM[38;5;89m0[38;5;17m:[38;5;4mco[38;5;18md[38;5;160mM[38;5;9mMMM[38;5;53mK[38;5;9mM[38;5;160mW[38;5;88mdd[38;5;1mdo[38;5;9mMMMMMM[38;5;52m.[38;5;233m.[38;5;232m [38;5;0m    [0;0m
[38;5;0m       [38;5;232m.[38;5;4mo[38;5;18md[38;5;53m0[38;5;124mM[38;5;160mM[38;5;9mM[38;5;125mM[38;5;17m:[38;5;0m   [38;5;232m [38;5;53mO[38;5;9mMM[38;5;125mM[38;5;18mx[38;5;89mN[38;5;52m:[38;5;0m   [38;5;232m [38;5;1mol[38;5;52mc:;[38;5;233m.[38;5;0m  [38;5;232m  [38;5;0m [38;5;232m [38;5;0m [0;0m
[38;5;0m       [38;5;17m::;:::[38;5;1mx[38;5;89mX[38;5;17m:[38;5;0m  [38;5;4ml[38;5;124mM[38;5;160mM[38;5;9mMM[38;5;53mK[38;5;18mdd[38;5;17m.[38;5;232m.[38;5;233m...[38;5;0m     [38;5;17m,[38;5;4mll[38;5;232m     [0;0m
[38;5;0m     [38;5;232m.[38;5;17m:[38;5;233m.[38;5;0m     [38;5;17mcc[38;5;232m [38;5;0m  [38;5;232m [38;5;53mO[38;5;52ml[38;5;88mk[38;5;125mW[38;5;4mk[38;5;18mddddo[38;5;4ml[38;5;17m:.',;:.[38;5;232m.  .[38;5;233m  [38;5;232m [0;0m
[38;5;0m     [38;5;233m.[38;5;0m       [38;5;17m;[38;5;18md[38;5;232m [38;5;0m [38;5;17m:[38;5;4moo[38;5;0m [38;5;17m.[38;5;4mo[38;5;18mdd[38;5;17m:[38;5;0m    [38;5;4ml[38;5;18mddddd[38;5;0m  [38;5;232m [38;5;233m..[38;5;234m   [0;0m
[38;5;0m             [38;5;233m.[38;5;17m;[38;5;0m [38;5;232m [38;5;18md[38;5;17m,[38;5;233m.[38;5;0m  [38;5;17m'[38;5;18md[38;5;0m      [38;5;18md[38;5;4mol[38;5;17m:;.[38;5;0m  [38;5;233m.'[38;5;234m,[38;5;17m   [0;0m
[38;5;0m          [38;5;232m [38;5;17m:,[38;5;0m [38;5;232m [38;5;0m  [38;5;233m.[38;5;0m    [38;5;17mc:[38;5;0m              [38;5;233m.'[38;5;234m:[38;5;17m  [38;5;4m [0;0m
[38;5;0m          [38;5;1md[38;5;53mK[38;5;18md[38;5;17m'[38;5;18md[38;5;4mo[38;5;232m [38;5;17m.[38;5;233m.[38;5;232m [38;5;0m  [38;5;232m  [38;5;0m              [38;5;233m'[38;5;234m;[38;5;17mc [38;5;18m [38;5;19m [0;0m
[38;5;0m         [38;5;232m [38;5;9mM[38;5;124mM[38;5;53m0[38;5;18mddd[38;5;17m;[38;5;18md[38;5;4mk[38;5;18mx[38;5;17m'[38;5;0m                [38;5;232m [38;5;233m.[38;5;234m,[38;5;17mc [38;5;18m [38;5;19m [0;0m
[38;5;0m         [38;5;52mc[38;5;9mMMM[38;5;232m .[38;5;89mX[38;5;53mKX[38;5;9mMM[38;5;232m.[38;5;0m                [38;5;232m .[38;5;233m.[38;5;234m,[38;5;17m [38;5;4m [38;5;19m [0;0m";

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    let user = get_user_by_uid(get_current_uid()).unwrap();

    let shell = String::from_utf8(
        Command::new("bash").arg("-c").arg("echo $SHELL").output().expect("failed to exec").stdout
    ).unwrap().split("/").last().unwrap().trim().to_string();

    let term = String::from_utf8(
        Command::new("bash").arg("-c").arg("echo $TERM").output().expect("failed to exec").stdout
    ).unwrap().trim().to_string();

    let de = String::from_utf8(
        Command::new("bash").arg("-c").arg("echo $XDG_CURRENT_DESKTOP").output().expect("failed to exec").stdout
    ).unwrap().trim().to_string();

    let my_pid = sysinfo::get_current_pid().expect("unable to get PID of the current process");
    let parent_pid = sys.process(my_pid).expect("no self process?").parent().expect("unable to get parent process");
    let parent_name = sys.process(parent_pid).expect("unable to get parent process").name().to_str().unwrap().to_string();

    let info = vec![
        format!(""),
        format!("┌────────────"),
        //format!("│ {} : {}", "Username  ".to_string().truecolor(90, 100, 255), user.name().to_str().unwrap().to_string()),
        format!("│ {} : {}", "OS Name   ".to_string().truecolor(90, 100, 255), System::name().unwrap()),
        format!("│ {} : {}", "Kernel    ".to_string().truecolor(90, 100, 255), System::kernel_version().unwrap()),
        format!("│ {} : {}", "OS Version".to_string().truecolor(90, 100, 255), System::os_version().unwrap()),
        format!("│ {} : {}", "Hostname  ".to_string().truecolor(90, 100, 255), System::host_name().unwrap()),
        format!("│ {} : {}", "CPU Arch  ".to_string().truecolor(90, 100, 255), System::cpu_arch().unwrap()),
        format!("│ {} : {}", "CPU Cores ".to_string().truecolor(90, 100, 255),
            sys.physical_core_count().unwrap().to_string()
        ),
        format!("│ {} : {}", "Shell     ".to_string().truecolor(90, 100, 255), parent_name),
        format!("│ {} : {}", "Terminal  ".to_string().truecolor(90, 100, 255), term),
        format!("│ {} : {}", "Desktop   ".to_string().truecolor(90, 100, 255), de),
        format!("│ {} : {}", "Uptime    ".to_string().truecolor(90, 100, 255), (|| {
            let u_s = System::uptime();
            format!(
                "{} Hours {} Minutes {} Seconds",
                u_s / 3600,
                u_s % 3600 / 60,
                u_s % 60
            )
        })()),
        format!(
            "│ {} : {}MiB / {}MiB", "Memory Use".to_string().truecolor(90, 100, 255),
            sys.used_memory() / 1000000,
            sys.total_memory() / 1000000
        ),
        format!("└────────────")
    ];

   let info_kernel_out = Command::new("uname")
        .arg("-r")
        .output()
        .expect("failed to fetch kernel ver");
    let info_kernel = String::from_utf8_lossy(&info_kernel_out.stdout);
    println!("\n");

    let binding = String::new();
    let mut iter = LOGO
        .lines()
        .zip(info.iter().chain(std::iter::repeat(&binding)));
    while let Some((logo, info)) = iter.next() {
        println!("{} {}", logo, info);
    }
}
