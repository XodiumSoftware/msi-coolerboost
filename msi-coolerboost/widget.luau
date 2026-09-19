-- Noctalia bar widget for MSI CoolerBoost.
--
-- Shows a propeller glyph in the bar. The glyph uses the palette's error
-- color when CoolerBoost is enabled and the normal text color when off.
-- Left click toggles CoolerBoost by running `sudo isw -b on/off`.
--
-- The widget also answers the "toggle" IPC event, so a niri keybind can call
-- the same toggle logic the widget uses:
--   noctalia msg plugin xodium/msi-coolerboost:coolerboost all toggle

-- Mirrors isw's internal state: file present with content = ON, absent = OFF.
-- Lives in /tmp so the state resets with the hardware on reboot.
local STATE_FILE = "/tmp/isw_coolerboost"

-- Guards against overlapping toggles; cleared by the runAsync callback.
local toggling = false

local function isOn()
  local contents = noctalia.readFile(STATE_FILE)
  return contents ~= nil and noctalia.string.trim(contents) ~= ""
end

local function render()
  local on = isOn()
  barWidget.setGlyph(on and "propeller" or "propeller-off")
  barWidget.setGlyphColor(on and "error" or "on_surface")
  barWidget.setTooltip("MSI CoolerBoost: " .. (on and "ON" or "OFF"))
end

local function toggle()
  if toggling then
    return
  end
  toggling = true

  local on = isOn()
  -- Run isw and update the mirrored state file in one shell command so the
  -- widget state stays in lock-step with the hardware command.
  local cmd
  if on then
    cmd = "sudo isw -b off && rm -f " .. STATE_FILE
  else
    cmd = "sudo isw -b on && echo on > " .. STATE_FILE
  end

  noctalia.runAsync(cmd, function(result)
    toggling = false
    if result.exitCode == 0 then
      noctalia.notify(
        "CoolerBoost " .. (on and "OFF" or "ON"),
        on and "Fan boost disabled" or "Fan boost enabled"
      )
    else
      local message = noctalia.string.trim(result.stderr)
      if message == "" then
        message = "isw exited with code " .. result.exitCode
      end
      noctalia.notifyError("CoolerBoost failed to toggle", message)
    end
    render()
  end)
end

function update()
  noctalia.setUpdateInterval(1000)
  render()
end

function onClick()
  toggle()
end

function onIpc(event, _payload)
  if event == "toggle" then
    toggle()
  end
end
