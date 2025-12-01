; ==========================================
; Hola Mundo en NASM (Windows x64)
; ==========================================
; Este programa imprime un mensaje en la consola usando
; la función printf de la biblioteca estándar de C.

default rel
section .text
global main
extern printf

main:
    ; Prólogo de la función (alineación de pila)
    sub rsp, 40
    
    ; Cargar la dirección del mensaje en RCX (primer argumento)
    mov rcx, msg
    
    ; Llamar a printf
    call printf
    
    ; Epílogo (restaurar pila)
    add rsp, 40
    
    ; Retornar 0 (éxito)
    xor eax, eax
    ret

section .data
    ; Definición del mensaje con salto de línea (10) y terminador nulo (0)
    msg db 'Hola NASM con comentarios!', 10, 0