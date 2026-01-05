enum OS {
    Windows(u32, String),
    Mac(u32, String),
    Linux(u32, String),
}

pub fn run() {
    let linux = OS::Linux(1991, String::from("Linus"));
    print_os_info(linux);
    let Windows = OS::Windows(1985, String::from("Microsoft"));
    print_os_info(Windows);
    let Mac = OS::Mac(2001, String::from("Apple"));
    print_os_info(Mac);
}

fn print_os_info(os: OS) {
    // パターンマッチング
    match os {
        OS::Windows(year, who) => {
            println!("Windows : {} {}", year, who);
        }
        OS::Linux(year, who) => {
            println!("Linux : {} {}", year, who);
        }
        OS::Mac(year, who) => {
            println!("Mac : {} {}", year, who);
        }
    }
}
