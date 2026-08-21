; Inno Setup Script for Morsel Clipboard Manager
; Builds morsel-setup-v0.1.1-windows-x64.exe

#define MyAppName "Morsel Clipboard Manager"
#define MyAppVersion "0.1.1"
#define MyAppPublisher "Morsel Contributors"
#define MyAppURL "https://github.com/pomagrenate/morsel"
#define MyAppExeName "morsel-desktop.exe"

[Setup]
AppId={{9C52E58C-519A-4E38-A6B1-39E1B8C1D61D}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppPublisher={#MyAppPublisher}
AppPublisherURL={#MyAppURL}
AppSupportURL={#MyAppURL}
AppUpdatesURL={#MyAppURL}
DefaultDirName={autopf}\Morsel
DefaultGroupName={#MyAppName}
DisableProgramGroupPage=yes
LicenseFile=..\..\LICENSE
OutputBaseFilename=morsel-setup-v0.1.1-windows-x64
Compression=lzma
SolidCompression=yes
WizardStyle=modern

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked
Name: "autostart"; Description: "Launch Morsel automatically when Windows starts"; GroupDescription: "Startup Options:"

[Files]
Source: "..\..\target\release\morsel-desktop.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\..\target\release\morsel.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\..\target\release\morseld.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\..\target\release\morsel-tui.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\..\README.md"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"
Name: "{group}\Morsel Terminal Search (TUI)"; Filename: "{app}\morsel-tui.exe"
Name: "{group}\{cm:UninstallProgram,{#MyAppName}}"; Filename: "{uninstallexe}"
Name: "{autodesktop}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; Tasks: desktopicon

[Registry]
; Add to Windows User Startup Registry
Root: HKCU; Subkey: "Software\Microsoft\Windows\CurrentVersion\Run"; ValueType: string; ValueName: "MorselClipboardManager"; ValueData: """{app}\{#MyAppExeName}"" --minimized"; Tasks: autostart; Flags: uninsdeletevalue
; Add install directory to User Environment PATH
Root: HKCU; Subkey: "Environment"; ValueType: expandsz; ValueName: "Path"; ValueData: "{olddata};{app}"; Check: NeedsAddPath(ExpandConstant('{app}'))

[Run]
Filename: "{app}\{#MyAppExeName}"; Description: "{cm:LaunchProgram,{#StringChange(MyAppName, '&', '&&')}}"; Flags: nowait postinstall skipifsilent

[Code]
function NeedsAddPath(Param: string): boolean;
var
  OrigPath: string;
begin
  if RegQueryStringValue(HKEY_CURRENT_USER, 'Environment', 'Path', OrigPath) then
  begin
    Result := Pos(Uppercase(Param), Uppercase(OrigPath)) = 0;
  end
  else
    Result := True;
end;
