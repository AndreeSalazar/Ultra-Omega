; ==========================================
; Hola Mundo en NASM (Linux x64)
; ==========================================
; Este programa imprime un mensaje en la consola usando
; la función printf de la biblioteca estándar de C.

default rel
section .text
global main
extern printf

main:
    ; Prólogo de la función (alineación de pila)
    sub rsp, 8
    
    ; Cargar la dirección del mensaje en RDI (primer argumento en Linux)
    mov rdi, msg
    
    ; RAX debe ser 0 para variadic functions en Linux
    xor rax, rax
    
    ; Llamar a printf
    call printf
    
    ; Epílogo (restaurar pila)
    add rsp, 8
    
    ; Retornar 0 (éxito)
    xor rax, rax
    ret

section .data
    ; Definición del mensaje con salto de línea (10) y terminador nulo (0)
    msg db 'Hola NASM Linux!', 10, 0

