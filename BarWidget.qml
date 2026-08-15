import QtQuick
import Quickshell
import Quickshell.Io

// Self-contained Omarchy 4 bar widget for MSI CoolerBoost.
//
// Shows a fan icon in the Omarchy bar. The icon uses the bar's urgent color
// when CoolerBoost is enabled and the normal foreground when off.
// Left click toggles CoolerBoost by running `sudo isw -b on/off`.
// Right click refreshes the configured Hyprland shortcut tooltip.
//
// The widget registers an IPC target under its plugin id, so a Hyprland
// shortcut can call the same toggle logic the widget uses:
//   o.bind("XF86Launch7", "Toggle CoolerBoost", "omarchy-shell xodium.msi-coolerboost toggle")
Item {
  id: root

  // Injected by the Omarchy bar host.
  property var bar: null
  property string moduleName: "xodium.msi-coolerboost"
  property var settings: ({})

  // Expected by the Omarchy bar click/tooltip routing.
  property bool interactive: true
  property bool pressable: true
  property bool concealed: false
  readonly property bool tooltipHovered: visible && interactive && !concealed && mouseArea.containsMouse

  function registerWithBar() {
    if (root.bar && root.bar.registerClickTarget) root.bar.registerClickTarget(root)
  }
  function unregisterFromBar() {
    if (root.bar && root.bar.unregisterClickTarget) root.bar.unregisterClickTarget(root)
  }
  function triggerPress(button) {
    if (root.bar) root.bar.hideTooltip(root)
    if (button === Qt.RightButton) {
      root.refreshShortcut()
    } else {
      root.toggle()
    }
  }

  onBarChanged: {
    unregisterFromBar()
    registerWithBar()
  }
  Component.onCompleted: {
    registerWithBar()
    root.refreshShortcut()
  }
  Component.onDestruction: unregisterFromBar()

  readonly property bool coolerBoostEnabled: String(stateFile.text() || "").trim() !== ""
  readonly property int barSize: bar ? bar.barSize : 26
  property string shortcut: "Unknown"
  property bool toggling: false
  property string tooltipText: "MSI CoolerBoost: " + (root.coolerBoostEnabled ? "ON" : "OFF")
    + "\nShortcut: " + root.shortcut

  implicitWidth: root.barSize
  implicitHeight: root.barSize

  function toggle() {
    if (root.toggling) return
    root.toggling = true
    toggleCooldown.restart()
    syncTimer.start()

    // Run isw and update the mirrored state file in one detached shell so the
    // widget state stays in lock-step with the hardware command.
    if (root.coolerBoostEnabled) {
      Quickshell.execDetached([
        "bash", "-c",
        "sudo isw -b off && rm -f /tmp/isw_coolerboost && notify-send -u low 'CoolerBoost OFF' 'Fan boost disabled'"
      ])
    } else {
      Quickshell.execDetached([
        "bash", "-c",
        "sudo isw -b on && echo on > /tmp/isw_coolerboost && notify-send -u low 'CoolerBoost ON' 'Fan boost enabled'"
      ])
    }
  }

  function refreshShortcut() {
    if (!shortcutProc.running) shortcutProc.running = true
  }

  // Expose the same toggle the UI uses to Hyprland via omarchy-shell.
  IpcHandler {
    target: root.moduleName

    function toggle(): void {
      root.toggle()
    }
  }

  FileView {
    id: stateFile
    path: "/tmp/isw_coolerboost"
    watchChanges: true
    printErrors: false
  }

  FileView {
    id: luaBindingsFile
    path: Quickshell.env("HOME") + "/.config/hypr/bindings.lua"
    watchChanges: true
    printErrors: false
    onLoaded: root.refreshShortcut()
    onFileChanged: root.refreshShortcut()
  }

  Process {
    id: shortcutProc
    command: [
      "bash", "-c",
      "grep -m1 'xodium\\.msi-coolerboost.*toggle' ~/.config/hypr/bindings.lua | sed -n 's/o\\.bind(\"\\([^\"]*\\)\".*)/\\1/p'"
    ]
    stdout: StdioCollector {
      waitForEnd: true
      onStreamFinished: {
        var keys = String(text || "").trim()
        if (keys) root.shortcut = keys
      }
    }
  }

  Timer {
    id: toggleCooldown
    interval: 2000
    repeat: false
    onTriggered: root.toggling = false
  }

  Timer {
    id: syncTimer
    interval: 500
    repeat: true
    triggeredOnStart: false
    running: root.toggling
    onTriggered: stateFile.reload()
  }

  Text {
    id: icon
    anchors.centerIn: parent
    text: "󰈐"
    font.family: root.bar ? root.bar.fontFamily : "monospace"
    font.pixelSize: root.barSize * 0.58
    color: root.coolerBoostEnabled
      ? (root.bar ? root.bar.urgent : "red")
      : (root.bar ? root.bar.foreground : "white")
    horizontalAlignment: Text.AlignHCenter
    verticalAlignment: Text.AlignVCenter
  }

  MouseArea {
    id: mouseArea
    anchors.fill: parent
    acceptedButtons: Qt.LeftButton | Qt.RightButton
    hoverEnabled: true
    cursorShape: Qt.PointingHandCursor

    onEntered: {
      if (root.bar) root.bar.showTooltip(root, root.tooltipText)
    }
    onExited: {
      if (root.bar) root.bar.hideTooltip(root)
    }
  }
}
