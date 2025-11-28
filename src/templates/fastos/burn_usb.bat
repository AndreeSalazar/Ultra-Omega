@echo off
REM ═══════════════════════════════════════════════════════════════════════════
REM FastOS - Quemar imagen a USB
REM ═══════════════════════════════════════════════════════════════════════════
REM ADVERTENCIA: Esto borrará TODOS los datos del USB seleccionado
REM ═══════════════════════════════════════════════════════════════════════════

setlocal enabledelayedexpansion

set "PROJECT_DIR=%~dp0"
set "BUILD_DIR=%PROJECT_DIR%build"
set "IMAGE=%BUILD_DIR%\os-image.bin"

set "RED=[91m"
set "GREEN=[92m"
set "YELLOW=[93m"
set "CYAN=[96m"
set "RESET=[0m"

echo %CYAN%═══════════════════════════════════════════════════════════════════════════%RESET%
echo %CYAN%                    FastOS - Quemar a USB                                  %RESET%
echo %CYAN%═══════════════════════════════════════════════════════════════════════════%RESET%
echo.

REM Verificar que existe la imagen
if not exist "%IMAGE%" (
    echo %RED%[ERROR]%RESET% No se encontro os-image.bin
    echo         Ejecuta primero: build.bat
    goto end
)

echo %YELLOW%ADVERTENCIA: Esto BORRARA todos los datos del USB seleccionado!%RESET%
echo.

REM Listar discos disponibles
echo %CYAN%Discos disponibles:%RESET%
echo.
wmic diskdrive get Index,Size,Model,InterfaceType | findstr /i "usb"
echo.

echo %YELLOW%Escribe el numero del disco USB (ej: 1, 2, etc.) o 'q' para salir:%RESET%
set /p DISK_NUM="> "

if /i "%DISK_NUM%"=="q" goto end

REM Confirmar
echo.
echo %RED%═══════════════════════════════════════════════════════════════════════════%RESET%
echo %RED%  ATENCION: Vas a escribir en el DISCO %DISK_NUM%                          %RESET%
echo %RED%  Esto BORRARA TODOS los datos del disco!                                  %RESET%
echo %RED%═══════════════════════════════════════════════════════════════════════════%RESET%
echo.
echo %YELLOW%Escribe 'SI' para confirmar:%RESET%
set /p CONFIRM="> "

if /i not "%CONFIRM%"=="SI" (
    echo %YELLOW%Operacion cancelada.%RESET%
    goto end
)

echo.
echo %CYAN%Quemando imagen a disco %DISK_NUM%...%RESET%

REM Usar dd si está disponible (Git Bash, Cygwin, WSL)
where dd >nul 2>&1
if %errorlevel%==0 (
    echo Usando dd...
    dd if="%IMAGE%" of=\\.\PhysicalDrive%DISK_NUM% bs=512
    if %errorlevel%==0 (
        echo.
        echo %GREEN%Imagen quemada exitosamente!%RESET%
        echo %GREEN%Ahora puedes arrancar desde el USB.%RESET%
    ) else (
        echo %RED%Error al quemar la imagen.%RESET%
    )
    goto end
)

REM Alternativa: usar PowerShell
echo Usando PowerShell...
powershell -Command "& {$img = [System.IO.File]::ReadAllBytes('%IMAGE%'); $disk = '\\.\PhysicalDrive%DISK_NUM%'; $stream = [System.IO.File]::Open($disk, 'Open', 'Write'); $stream.Write($img, 0, $img.Length); $stream.Close(); Write-Host 'Imagen quemada exitosamente!'}"

if %errorlevel%==0 (
    echo.
    echo %GREEN%Imagen quemada exitosamente!%RESET%
    echo %GREEN%Ahora puedes arrancar desde el USB.%RESET%
) else (
    echo.
    echo %RED%Error al quemar la imagen.%RESET%
    echo %YELLOW%Alternativas:%RESET%
    echo   1. Usa Rufus: https://rufus.ie
    echo   2. Usa balenaEtcher: https://www.balena.io/etcher
    echo   3. Ejecuta como Administrador
)

:end
echo.
pause
endlocal

