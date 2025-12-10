# Condicionales en Python 3.12
# if/else y match/case (pattern matching)

# If/Else tradicional
edad = 18

if edad >= 18:
    print("Eres mayor de edad")
else:
    print("Eres menor de edad")

# If/Elif/Else
nota = 85

if nota >= 90:
    print("Calificación: A (Excelente)")
elif nota >= 80:
    print("Calificación: B (Muy bien)")
elif nota >= 70:
    print("Calificación: C (Bien)")
else:
    print("Calificación: D (Necesita mejorar)")

# Pattern Matching (Python 3.10+)
tipo = "usuario"

match tipo:
    case "admin":
        print("Acceso de administrador")
    case "usuario":
        print("Acceso de usuario normal")
    case "invitado":
        print("Acceso de invitado")
    case _:
        print("Tipo desconocido")

