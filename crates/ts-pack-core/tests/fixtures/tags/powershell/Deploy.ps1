$ErrorActionPreference = 'Stop'
$Targets = @('api', 'worker')

enum Stage {
    Build
    Deploy = 2
}

class Service {
    [string] $Name
    [int] $Port = 8080

    Service([string] $name) {
        $this.Name = $name
    }

    [string] Url() {
        return "http://localhost:$($this.Port)/$($this.Name)"
    }
}

function Invoke-Build {
    param([string] $Name)
    $out = Join-Path $PSScriptRoot "bin/$Name"
    Write-Host "building $Name into $out"
}

foreach ($target in $Targets) {
    Invoke-Build -Name $target
    [Service]::new($target).Url()
}
