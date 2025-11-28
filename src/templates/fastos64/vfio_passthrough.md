# 🎮 GPU Passthrough - RTX 3060 con FastOS

## 📋 Descripción

Esta guía explica cómo usar **GPU Passthrough (VFIO/PCI Passthrough)** para pasar tu RTX 3060 12GB directamente a una máquina virtual ejecutando FastOS, permitiendo acceso completo a la GPU con drivers NVIDIA y Vulkan.

## 🔧 Requisitos

### Hardware
- **CPU**: AMD Ryzen 5 5600X (soporta IOMMU/AMD-Vi) ✅
- **GPU**: NVIDIA RTX 3060 12GB ✅
- **RAM**: 16 GB (mínimo 8GB para VM) ✅
- **Motherboard**: Con soporte IOMMU

### Software
- **Host OS**: Linux (Ubuntu 22.04+ / Fedora 38+ recomendado)
- **QEMU/KVM**: Para virtualización
- **VFIO drivers**: Para passthrough
- **OVMF**: UEFI firmware para VM

---

## 📝 Paso 1: Verificar soporte IOMMU

```bash
# Verificar grupos IOMMU
#!/bin/bash
shopt -s nullglob
for g in $(find /sys/kernel/iommu_groups/* -maxdepth 0 -type d | sort -V); do
    echo "IOMMU Group ${g##*/}:"
    for d in $g/devices/*; do
        echo -e "\t$(lspci -nns ${d##*/})"
    done;
done;
```

Busca tu RTX 3060:
```
IOMMU Group 15:
    01:00.0 VGA compatible controller [0300]: NVIDIA Corporation GA106 [10de:2503] (rev a1)
    01:00.1 Audio device [0403]: NVIDIA Corporation GA106 High Definition Audio [10de:228e] (rev a1)
```

---

## 📝 Paso 2: Configurar GRUB

Edita `/etc/default/grub`:

```bash
# Para AMD (Ryzen 5 5600X)
GRUB_CMDLINE_LINUX_DEFAULT="quiet splash amd_iommu=on iommu=pt vfio-pci.ids=10de:2503,10de:228e"
```

Actualiza GRUB:
```bash
sudo update-grub
sudo reboot
```

---

## 📝 Paso 3: Configurar VFIO

Crea `/etc/modprobe.d/vfio.conf`:
```bash
options vfio-pci ids=10de:2503,10de:228e
softdep nvidia pre: vfio-pci
```

Crea `/etc/modules-load.d/vfio.conf`:
```
vfio
vfio_iommu_type1
vfio_pci
vfio_virqfd
```

Regenera initramfs:
```bash
sudo update-initramfs -u
sudo reboot
```

---

## 📝 Paso 4: Verificar VFIO

```bash
# Verificar que VFIO controla la GPU
lspci -nnk -d 10de:2503
```

Debe mostrar:
```
01:00.0 VGA compatible controller [0300]: NVIDIA Corporation GA106 [10de:2503]
    Kernel driver in use: vfio-pci
```

---

## 📝 Paso 5: Crear VM con QEMU

### Script de inicio: `start_fastos_vm.sh`

```bash
#!/bin/bash

# FastOS VM con RTX 3060 Passthrough
# Autor: Eddi Andreé Salazar Matos

VMNAME="FastOS-64"
MEMORY="8G"
CPUS="4"
OVMF_CODE="/usr/share/OVMF/OVMF_CODE.fd"
OVMF_VARS="/var/lib/libvirt/qemu/nvram/FastOS_VARS.fd"

# Copiar OVMF vars si no existe
if [ ! -f "$OVMF_VARS" ]; then
    cp /usr/share/OVMF/OVMF_VARS.fd "$OVMF_VARS"
fi

# Descargar driver NVIDIA a la VM
# El driver se instala dentro de FastOS

qemu-system-x86_64 \
    -name "$VMNAME" \
    -machine q35,accel=kvm \
    -cpu host,kvm=on \
    -smp $CPUS,sockets=1,cores=$CPUS,threads=1 \
    -m $MEMORY \
    -drive if=pflash,format=raw,readonly=on,file="$OVMF_CODE" \
    -drive if=pflash,format=raw,file="$OVMF_VARS" \
    -drive file=fastos64.img,format=raw,if=virtio \
    -device vfio-pci,host=01:00.0,multifunction=on \
    -device vfio-pci,host=01:00.1 \
    -device virtio-net-pci,netdev=net0 \
    -netdev user,id=net0 \
    -device qemu-xhci \
    -device usb-kbd \
    -device usb-mouse \
    -vga none \
    -nographic \
    -monitor stdio
```

---

## 📝 Paso 6: Instalar Driver NVIDIA en FastOS

Una vez que FastOS arranca con la GPU pasada:

### Opción A: Usar framebuffer UEFI GOP
- Ya funciona automáticamente
- 1920x1080 disponible
- Sin aceleración 3D

### Opción B: Driver NVIDIA (requiere Windows API layer)
Para usar el driver completo de NVIDIA, FastOS necesitaría implementar:
- Windows NT Kernel APIs
- DirectX/WDDM interfaces
- Esto es equivalente a crear un "mini-Windows"

### Opción C: Nouveau (open-source)
```bash
# En FastOS, cargar módulo nouveau
insmod nouveau.ko
```
- Soporte parcial para RTX 30xx
- Rendimiento limitado

---

## 🎯 Arquitectura Final

```
┌─────────────────────────────────────────────────────────────────┐
│                     HOST (Linux)                                │
│  ┌─────────────┐                                                │
│  │   CPU       │  AMD Ryzen 5 5600X                            │
│  │   RAM       │  16 GB (8 GB para host, 8 GB para VM)         │
│  │   Storage   │  1 TB NVMe                                    │
│  └─────────────┘                                                │
│                                                                 │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │                    QEMU/KVM VM                           │   │
│  │  ┌─────────────────────────────────────────────────┐    │   │
│  │  │              FastOS 64-bit                       │    │   │
│  │  │                                                  │    │   │
│  │  │  ┌──────────────────────────────────────────┐   │    │   │
│  │  │  │         RTX 3060 12GB (VFIO)             │   │    │   │
│  │  │  │  - Acceso directo via PCI passthrough    │   │    │   │
│  │  │  │  - Vulkan disponible                     │   │    │   │
│  │  │  │  - CUDA disponible                       │   │    │   │
│  │  │  │  - Ray Tracing disponible                │   │    │   │
│  │  │  └──────────────────────────────────────────┘   │    │   │
│  │  │                                                  │    │   │
│  │  │  ┌──────────────────────────────────────────┐   │    │   │
│  │  │  │         Desktop FastOS                    │   │    │   │
│  │  │  │  - Window Manager                         │   │    │   │
│  │  │  │  - Taskbar estilo Win11                   │   │    │   │
│  │  │  │  - Task Manager                           │   │    │   │
│  │  │  └──────────────────────────────────────────┘   │    │   │
│  │  └─────────────────────────────────────────────────┘    │   │
│  └─────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
```

---

## ⚠️ Notas Importantes

1. **Necesitas Linux como host** - Windows no soporta VFIO
2. **La GPU no estará disponible en el host** mientras la VM la use
3. **Algunos motherboards** tienen problemas de grupos IOMMU
4. **El driver NVIDIA en FastOS** requiere implementar APIs de Windows

---

## 🔗 Referencias

- [Arch Wiki - PCI Passthrough](https://wiki.archlinux.org/title/PCI_passthrough_via_OVMF)
- [VFIO Tips and Tricks](https://vfio.blogspot.com/)
- [NVIDIA Open GPU Kernel Modules](https://github.com/NVIDIA/open-gpu-kernel-modules)

---

## 👤 Autor

**Eddi Andreé Salazar Matos**
Desarrollador Peruano 🇵🇪

*FastOS 64-bit + Vulkan + RTX 3060 Project*

