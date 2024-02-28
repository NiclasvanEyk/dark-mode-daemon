// Inspired by https://github.com/bouk/dark-mode-notify

import Cocoa

let fileManager = FileManager.default

func xdgConfigDirectory() -> URL? {
    if let xdgConfigHome = ProcessInfo.processInfo.environment["XDG_CONFIG_HOME"] {
        let baseDirectory = URL(fileURLWithPath: xdgConfigHome).appendingPathComponent("dark-mode-daemon")
        if fileManager.fileExists(atPath: baseDirectory.path) {
            return baseDirectory
        }
    }

    return nil
}

func defaultConfigDirectory() -> URL? {
    let homeDirectory = fileManager.homeDirectoryForCurrentUser
    let fallbackPath = homeDirectory.appendingPathComponent(".config/dark-mode-daemon")
    if fileManager.fileExists(atPath: fallbackPath.path) {
        return fallbackPath
    }

    return nil
}

func baseDirectory() -> URL? {
    var configDirectory = xdgConfigDirectory()

    if configDirectory == nil {
        configDirectory = defaultConfigDirectory()
    }

    return configDirectory
}

func scriptsDirectory(base: URL) -> URL? {
    let scriptsDirectory = base.appendingPathComponent("scripts")
    if fileManager.fileExists(atPath: scriptsDirectory.path) {
        return scriptsDirectory
    }

    return nil
}

func gatherConfiguredScripts() -> [String] {
    guard let baseDirectory = baseDirectory() else {
        print("Did not find any dark-mode-daemon directory...")
        return []
    }

    guard let scriptsDirectory = scriptsDirectory(base: baseDirectory) else {
        print("Did not find any scripts in \(baseDirectory.path)/scripts...")
        return []
    }
    print("📂 Using scripts in \(scriptsDirectory.path)...")

    var runnableScripts = [String]()
    do {
        let items = try fileManager.contentsOfDirectory(atPath: scriptsDirectory.path)
        for item in items {
            let path = scriptsDirectory.appendingPathComponent(item).path
            if !fileManager.isExecutableFile(atPath: path) {
                print("⏭️ Skipping \(item), since it is not executable")
                continue
            }

            runnableScripts.append(path)
        }
    } catch {
        print("❌ Failed to read scripts in \(scriptsDirectory.path)")
        return []
    }

    return runnableScripts
}

func executeShellScript(script: String, mode: String) {
    let process = Process()
    process.launchPath = script
    process.arguments = []

    var env = ProcessInfo.processInfo.environment
    env["DMD_COLOR_MODE"] = mode
    process.environment = env

    let pipe = Pipe()
    process.standardOutput = pipe
    print("🚀 Launching \(script)...")
    process.launch()

    let data = pipe.fileHandleForReading.readDataToEndOfFile()
    var lastOutput = ""
    if let output = String(data: data, encoding: .utf8) {
        lastOutput = output
    }

    process.waitUntilExit()
    let status = process.terminationStatus
    if status != 0 {
        print("❌ Script \(script) failed failed with status \(status). \nLast output: \(lastOutput)")
    }
}

func runConfiguredScripts(mode: String) {
    let scripts = gatherConfiguredScripts()
    let dispatchGroup = DispatchGroup()
    let queue = DispatchQueue.global()

    for script in scripts {
        dispatchGroup.enter()
        queue.async {
            executeShellScript(script: script, mode: mode)
            dispatchGroup.leave()
        }
    }

    dispatchGroup.notify(queue: .main) {
        print("🔄 Notified \(scripts.count) scripts about the change to \(mode) mode")
    }
}

func runConfiguredScriptsWhileInferringColorMode() {
    let style = UserDefaults.standard.string(forKey: "AppleInterfaceStyle")
    let mode = style == "Dark" ? "dark" : "light"

    runConfiguredScripts(mode: mode)
}

func runDaemon() {
    print("😈 Starting daemon...")

    // We run it once on system startup...
    runConfiguredScriptsWhileInferringColorMode()

    // and when we wake from sleep ...
    NSWorkspace.shared.notificationCenter.addObserver(
        forName: NSWorkspace.didWakeNotification,
        object: nil,
        queue: nil) { (notification) in
            runConfiguredScriptsWhileInferringColorMode()
    }

    // and of course when the color mode changes!
    DistributedNotificationCenter.default.addObserver(
        forName: Notification.Name("AppleInterfaceThemeChangedNotification"),
        object: nil,
        queue: nil) { (notification) in
            runConfiguredScriptsWhileInferringColorMode()
    }

    NSApplication.shared.run()
}

func printHelp() {
    print("dark-mode-daemon runs scripts when the system changes between light and dark mode.")
    print("")
    print("Usage: dark-mode-daemon [command]")
    print("Commands:")
    print("  help                    Displays this help message")
    print("  daemon                  Starts the daemon")
    print("  list                    Prints the scripts that would be run")
    print("")
    print("It looks into the first existing directory of")
    print("  - $XDG_CONFIG_HOME/dark-mode-daemon/scripts")
    print("  - $HOME/.config/dark-mode-daemon/scripts")
    print("in that order, and executes all executable scripts it can find.")
    print("The environment variable DMD_COLOR_MODE will be either set to 'light' or 'dark',")
    print("depending on the new mode")
}

let arguments = CommandLine.arguments
if arguments.count == 1 || arguments.contains("--help") || arguments.contains("-h") || arguments.contains("help") {
    printHelp()
    exit(0)
}

let command = arguments[1]

if command == "list" {
    let scripts = gatherConfiguredScripts()
    print("")

    for script in scripts {
        print(script)
    }
    exit(0)
}

if command == "daemon" {
    runDaemon()
    exit(0)
}

print("Unknown command: \(command)")
exit(1)



