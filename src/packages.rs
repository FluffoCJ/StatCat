use std::fs;

pub fn detect_package_manager() -> &'static str {
    let managers = [
        ("pacman", "/usr/bin/pacman"),
        ("dpkg", "/usr/bin/dpkg"),
        ("rpm", "/usr/bin/rpm"),
        ("zypper", "/usr/bin/zypper"),
    ];

    for (name, path) in &managers {
        if fs::metadata(path).is_ok() {
            return *name;
        }
    }

    "None"
}

pub fn get_package_count() -> usize {
    match detect_package_manager() {
        "pacman" => count_pacman_packages(),
        "dpkg" => count_dpkg_packages(),
        "rpm" => count_rpm_packages(),
        "zypper" => count_zypper_packages(),
        _ => 0,
    }
}

fn count_pacman_packages() -> usize {
    let path = "/var/lib/pacman/local";
    fs::read_dir(path).map_or(0, |entries| entries.count())
}

fn count_dpkg_packages() -> usize {
    let path = "/var/lib/dpkg/status";
    let content = fs::read_to_string(path).unwrap_or_default();
    content
        .lines()
        .filter(|line| line.starts_with("Package:"))
        .count()
}

fn count_rpm_packages() -> usize {
    let path = "/var/lib/rpm/Packages";
    if fs::metadata(path).is_ok() {
        1
    } else {
        0
    }
}

fn count_zypper_packages() -> usize {
    let path = "/var/lib/zypp/Db/packages";
    fs::read_dir(path).map_or(0, |entries| entries.count())
}
