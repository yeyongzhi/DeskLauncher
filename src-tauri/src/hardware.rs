use serde::{Deserialize, Serialize};
use serde_json;
use std::process::Command;
use sysinfo::Components;

#[derive(Serialize, Debug)]
pub struct HardwareInfo {
    pub computer_name: String,
    pub computer_model: String,
    pub os: String,
    pub cpu: String,
    pub cpu_temp: String,
    pub motherboard: String,
    pub memory: String,
    pub main_disk: String,
    pub gpu: String,
    pub monitor: String,
    pub sound_card: String,
    pub network_card: String,
}

#[derive(Deserialize)]
struct WmiResult {
    computer_name: Option<String>,
    computer_model: Option<String>,
    os: Option<String>,
    cpu: Option<String>,
    motherboard: Option<String>,
    memory: Option<String>,
    main_disk: Option<String>,
    gpu: Option<String>,
    monitor: Option<String>,
    sound_card: Option<String>,
    network_card: Option<String>,
}

const PS_SCRIPT: &str = r#"
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8
$cs   = Get-CimInstance Win32_ComputerSystem
$os   = Get-CimInstance Win32_OperatingSystem
$cpu  = Get-CimInstance Win32_Processor       | Select-Object -First 1
$mb   = Get-CimInstance Win32_BaseBoard
$mem  = Get-CimInstance Win32_PhysicalMemory  | Select-Object -First 1
$disk = Get-CimInstance Win32_DiskDrive       | Select-Object -First 1
$gpu  = Get-CimInstance Win32_VideoController | Select-Object -First 1
$mon  = Get-CimInstance Win32_DesktopMonitor  | Where-Object { $_.Name -notlike '*Default*' } | Select-Object -First 1
$snd  = Get-CimInstance Win32_SoundDevice     | Select-Object -First 1
$net  = Get-CimInstance Win32_NetworkAdapter  | Where-Object { $_.NetConnectionStatus -eq 2 } | Select-Object -First 1

$mem_gb   = [math]::Round($cs.TotalPhysicalMemory / 1GB)
$disk_gb  = [math]::Round($disk.Size / 1GB)

# VRAM: WMI AdapterRAM overflows for >4GB GPUs; try registry first
$vram_key = Get-ItemProperty "HKLM:\SYSTEM\ControlSet001\Control\Class\{4d36e968-e325-11ce-bfc1-08002be10318}\0000" `
    -Name "HardwareInformation.qwMemorySize" -ErrorAction SilentlyContinue
$gpu_gb = if ($vram_key) {
    [math]::Round($vram_key."HardwareInformation.qwMemorySize" / 1GB)
} else {
    [math]::Round($gpu.AdapterRAM / 1GB)
}

[ordered]@{
    computer_name = $cs.Name
    computer_model = $cs.Model
    os            = "$($os.Caption) ($($os.OSArchitecture))"
    cpu           = $cpu.Name
    motherboard   = "$($mb.Manufacturer) $($mb.Product)"
    memory        = "${mem_gb}GB ($($mem.Speed) MHz)"
    main_disk     = "${disk_gb} GB ($($disk.Model))"
    gpu           = "$($gpu.Name) (${gpu_gb} GB)"
    monitor       = if ($mon)  { $mon.Name  } else { 'N/A' }
    sound_card    = if ($snd)  { $snd.Name  } else { 'N/A' }
    network_card  = if ($net)  { $net.Name  } else { 'N/A' }
} | ConvertTo-Json
"#;

fn na(s: Option<String>) -> String {
    s.filter(|v| !v.trim().is_empty()).unwrap_or_else(|| "N/A".to_string())
}

fn cpu_temp_str() -> String {
    let components = Components::new_with_refreshed_list();
    components
        .iter()
        .find(|c| {
            let l = c.label().to_lowercase();
            l.contains("package") || l.contains("tctl") || l.contains("cpu")
        })
        .and_then(|c| c.temperature().map(|t| format!("{:.0}°C", t)))
        .unwrap_or_else(|| "N/A".to_string())
}

pub fn collect() -> HardwareInfo {
    let components = Components::new_with_refreshed_list();
    let cpu_temp = components
        .iter()
        .find(|c| {
            let l = c.label().to_lowercase();
            l.contains("package") || l.contains("tctl") || l.contains("cpu")
        })
        .and_then(|c| c.temperature().map(|t| format!("{:.0}°C", t)))
        .unwrap_or_else(|| "N/A".to_string());

    // All WMI queries in one PowerShell call
    let output = Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", PS_SCRIPT])
        .output();

    let wmi: Option<WmiResult> = output.ok().and_then(|o| {
        let json = String::from_utf8_lossy(&o.stdout).to_string();
        serde_json::from_str(&json).ok()
    });

    match wmi {
        Some(w) => HardwareInfo {
            computer_name: na(w.computer_name),
            computer_model: na(w.computer_model),
            os: na(w.os),
            cpu: na(w.cpu),
            cpu_temp,
            motherboard: na(w.motherboard),
            memory: na(w.memory),
            main_disk: na(w.main_disk),
            gpu: na(w.gpu),
            monitor: na(w.monitor),
            sound_card: na(w.sound_card),
            network_card: na(w.network_card),
        },
        None => HardwareInfo {
            computer_name: "N/A".into(),
            computer_model: "N/A".into(),
            os: "N/A".into(),
            cpu: "N/A".into(),
            cpu_temp,
            motherboard: "N/A".into(),
            memory: "N/A".into(),
            main_disk: "N/A".into(),
            gpu: "N/A".into(),
            monitor: "N/A".into(),
            sound_card: "N/A".into(),
            network_card: "N/A".into(),
        },
    }
}

// ──────────────────────────────────────────────
//  Detailed hardware info (single PS call → JSON)
// ──────────────────────────────────────────────

const PS_DETAIL: &str = r#"
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8

function vmap { param($n) switch -Wildcard ($n) {
    '*NVIDIA*' {'英伟达'} '*AMD*' {'AMD'} '*Radeon*' {'AMD'}
    '*Intel*'  {'英特尔'} '*Realtek*' {'瑞昱'} '*Broadcom*' {'Broadcom'}
    '*Qualcomm*' {'高通'} '*MSI*' {'微星'} '*Micro-Star*' {'微星'}
    '*ASUS*' {'华硕'} '*ASUSTeK*' {'华硕'} '*Gigabyte*' {'技嘉'}
    default {$n}
}}

# GPU
$gpus = @(Get-CimInstance Win32_VideoController | ForEach-Object {
    $vk = Get-ItemProperty "HKLM:\SYSTEM\ControlSet001\Control\Class\{4d36e968-e325-11ce-bfc1-08002be10318}\0000" -Name "HardwareInformation.qwMemorySize" -EA SilentlyContinue
    $vram = if ($vk) { "$([math]::Round($vk.'HardwareInformation.qwMemorySize'/1MB)) MB" } else { "$([math]::Round($_.AdapterRAM/1MB)) MB" }
    @{ name=$_.Name; vendor=(vmap $_.Name); vram=$vram; video_processor=$_.VideoProcessor; driver_version=$_.DriverVersion;
       current_resolution=if($_.CurrentHorizontalResolution){"$($_.CurrentHorizontalResolution) x $($_.CurrentVerticalResolution)"}else{'N/A'}
       refresh_rate=if($_.CurrentRefreshRate){"$($_.CurrentRefreshRate) Hz"}else{'N/A'} }
})

# Memory
$memory_sticks = @(Get-CimInstance Win32_PhysicalMemory | ForEach-Object {
    @{ name="$($_.Speed) MHz"; vendor=if($_.Manufacturer){$_.Manufacturer.Trim()}else{'N/A'}
       size="$([math]::Round($_.Capacity/1GB)) GB"; speed="$($_.Speed) MHz"
       slot=$_.DeviceLocator; data_width="$($_.DataWidth)" }
})

# CPU
$c = Get-CimInstance Win32_Processor | Select-Object -First 1
$cpu = @{ name=$c.Name; vendor=(vmap $c.Name); cores="$($c.NumberOfCores)"
    max_speed="$($c.MaxClockSpeed) MHz"; ext_clock=if($c.ExtClock){"$($c.ExtClock) MHz"}else{'N/A'}
    current_speed="$($c.CurrentClockSpeed) MHz"
    l2_cache=if($c.L2CacheSize){"$($c.L2CacheSize) KB"}else{'N/A'}
    l3_cache=if($c.L3CacheSize){"$($c.L3CacheSize) KB"}else{'N/A'}
    voltage=if($c.CurrentVoltage){"$([math]::Round($c.CurrentVoltage/10,3)) V"}else{'N/A'}
    processor_id=$c.ProcessorId; data_width="$($c.DataWidth)"; temperature='' }

# Motherboard
$mb=$mbinfo=Get-CimInstance Win32_BaseBoard
$bios=Get-CimInstance Win32_BIOS
$cs=Get-CimInstance Win32_ComputerSystem
$os=Get-CimInstance Win32_OperatingSystem
$motherboard = @{ name="$($mb.Manufacturer) $($mb.Product)"; vendor=(vmap $mb.Manufacturer)
    serial=if($mb.SerialNumber -and $mb.SerialNumber -notlike '*Default*'){$mb.SerialNumber}else{'N/A'}
    bios_vendor=$bios.Manufacturer; bios_version=$bios.SMBIOSBIOSVersion
    bios_date=$bios.ReleaseDate.ToString('yyyy/MM/dd')
    computer_model=$cs.Model; computer_vendor=$cs.Manufacturer
    install_date=$os.InstallDate.ToString('yyyy/MM/dd')
    last_boot=$os.LastBootUpTime.ToString('yyyy/MM/dd HH:mm') }

# Disks
$disks = @(Get-CimInstance Win32_DiskDrive | Sort-Object Index | ForEach-Object {
    $mfr=if($_.Manufacturer -and $_.Manufacturer -ne '(Standard disk drives)'){$_.Manufacturer}else{'N/A'}
    @{ name=$_.Model; vendor=$mfr; size="$([math]::Round($_.Size/1GB)) GB"
       is_primary=if($_.Index -eq 0){'是'}else{'否'}
       serial=if($_.SerialNumber){$_.SerialNumber.Trim()}else{'N/A'} }
})

# Monitors
$monitors = @()
try {
    $mids  = @(Get-CimInstance -Namespace root\wmi -ClassName WmiMonitorID -EA Stop)
    $mpars = @(Get-CimInstance -Namespace root\wmi -ClassName WmiMonitorBasicDisplayParams -EA SilentlyContinue)
    for ($i=0;$i -lt $mids.Count;$i++) {
        $m=$mids[$i]
        $mfr=[System.Text.Encoding]::ASCII.GetString(($m.ManufacturerName|Where-Object{$_ -ne 0})).Trim()
        $prd=[System.Text.Encoding]::ASCII.GetString(($m.ProductCodeID|Where-Object{$_ -ne 0})).Trim()
        $mfr_cn=switch($mfr){'HKC'{'惠科'}'AOC'{'冠捷'}'DEL'{'戴尔'}'SAM'{'三星'}'LGD'{'LG'} default{$mfr}}
        $p=if($i -lt $mpars.Count){$mpars[$i]}else{$null}
        $wc=if($p){[math]::Round($p.MaxHorizontalImageSize)}else{0}
        $hc=if($p){[math]::Round($p.MaxVerticalImageSize)}else{0}
        $diag=if($wc -gt 0 -and $hc -gt 0){"$([math]::Round([math]::Sqrt($wc*$wc+$hc*$hc)/2.54,1)) 英寸 (${wc}厘米X${hc}厘米)"}else{'N/A'}
        $yr=$m.YearOfManufacture; $mo=if($m.WeekOfManufacture -gt 0){[math]::Max(1,[math]::Ceiling($m.WeekOfManufacture/4.33))}else{0}
        $monitors += @{ name="$mfr_cn $prd"; vendor=$mfr_cn; code=$prd; size=$diag
            manufacture_date=if($yr -gt 0){"$yr/$mo"}else{'N/A'} }
    }
} catch {}

# Networks
$networks = @(Get-CimInstance Win32_NetworkAdapter | Where-Object { $_.PhysicalAdapter -eq $true -and $_.MACAddress } | ForEach-Object {
    @{ name=$_.Name; vendor=(vmap $_.Name); mac=$_.MACAddress }
})

# Sound
$sounds = @(Get-CimInstance Win32_SoundDevice | ForEach-Object {
    @{ name=$_.Name; vendor=(vmap $_.Name) }
})

@{ gpus=$gpus; memory_sticks=$memory_sticks; cpu=$cpu; motherboard=$motherboard
   disks=$disks; monitors=$monitors; networks=$networks; sounds=$sounds
} | ConvertTo-Json -Depth 5 -Compress
"#;

pub fn collect_detail() -> serde_json::Value {
    let temp = cpu_temp_str();

    let output = Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", PS_DETAIL])
        .output();

    let mut result: serde_json::Value = output
        .ok()
        .and_then(|o| serde_json::from_str(&String::from_utf8_lossy(&o.stdout)).ok())
        .unwrap_or(serde_json::json!({}));

    if let Some(obj) = result.get_mut("cpu").and_then(|v| v.as_object_mut()) {
        obj.insert("temperature".into(), serde_json::json!(temp));
    }

    result
}
