; ARKHE OS Bootloader (x86_64)
; Substrato 996
; Responsabilidades:
; 1. Carregar kernel do IPFS via CID (simulado em bare metal)
; 2. Verificar assinatura Ed25519 (stub)
; 3. Configurar modo protegido/longo e paginação
; 4. Saltar para o kernel

section .text
global _start

_start:
    ; Configurar ambiente básico de boot (modo real 16-bit)
    cli

    ; Passo 1: Habilitar linha A20
    in al, 0x92
    or al, 2
    out 0x92, al

    ; Passo 2: Configurar GDT (Global Descriptor Table)
    lgdt [gdt_descriptor]

    ; Passo 3: Mudar para Protected Mode (32-bit)
    mov eax, cr0
    or eax, 1
    mov cr0, eax

    ; Saltar para código 32-bit (limpar pipeline)
    jmp 0x08:protected_mode_entry

[bits 32]
protected_mode_entry:
    ; Configurar segmentos
    mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax
    mov esp, 0x90000 ; Stack

    ; Passo 4: Configurar Paginação e Long Mode (64-bit)

    ; Limpar tabelas de página
    mov edi, 0x1000
    mov cr3, edi
    xor eax, eax
    mov ecx, 4096
    rep stosd
    mov edi, cr3

    ; Configurar PML4, PDP, PD e PT
    mov dword [edi], 0x2003
    mov dword [edi + 0x1000], 0x3003
    mov dword [edi + 0x2000], 0x4003
    mov dword [edi + 0x3000], 0x00000083

    ; Habilitar PAE
    mov eax, cr4
    or eax, 1 << 5
    mov cr4, eax

    ; Habilitar Long Mode
    mov ecx, 0xC0000080
    rdmsr
    or eax, 1 << 8
    wrmsr

    ; Habilitar Paginação
    mov eax, cr0
    or eax, 1 << 31
    mov cr0, eax

    ; Carregar GDT de 64-bit
    lgdt [gdt64_descriptor]

    ; Saltar para código 64-bit
    jmp 0x08:long_mode_entry

[bits 64]
long_mode_entry:
    ; Inicializar registradores de segmento
    mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax

    ; Aqui seria o código para carregar via IPFS e verificar Ed25519
    ; Simulação do stub de verificação

    ; Saltar para o entry point do kernel (endereço fixo para exemplo)
    ; mov rax, KERNEL_ENTRY_POINT
    ; call rax

    hlt
    jmp $

section .data
align 4
gdt_start:
    dq 0x0 ; Null descriptor
    dq 0x00CF9A000000FFFF ; 32-bit Code
    dq 0x00CF92000000FFFF ; 32-bit Data
gdt_end:

gdt_descriptor:
    dw gdt_end - gdt_start - 1
    dd gdt_start

align 4
gdt64_start:
    dq 0x0 ; Null
    dq 0x0020980000000000 ; 64-bit Code
    dq 0x0000920000000000 ; 64-bit Data
gdt64_end:

gdt64_descriptor:
    dw gdt64_end - gdt64_start - 1
    dq gdt64_start
