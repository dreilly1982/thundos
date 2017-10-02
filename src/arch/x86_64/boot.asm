global start

section .text
bits 32
start:
    ; print 'OK' to screen
    mov dword [0xb80000], 0x2f4b244f
    hlt