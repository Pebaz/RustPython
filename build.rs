use powershell_script;

fn main() {
    // Run scripts/symlinks-to-hardlinks.ps1 automatically on Windows
    if cfg!(windows) {
        let command = "scripts/symlinks-to-hardlinks.ps1";

        powershell_script::run(command, false).expect(
            "Failed to convert symlinks into hardlinks"
        );
    }
}
