# Listas y Diccionarios en Python 3.12
# Estructuras de datos básicas

# Listas
print("=== LISTAS ===")
numeros = [1, 2, 3, 4, 5]
print(f"Lista original: {numeros}")

# Agregar elementos
numeros.append(6)
print(f"Después de append: {numeros}")

# Acceder a elementos
print(f"Primer elemento: {numeros[0]}")
print(f"Último elemento: {numeros[-1]}")

# Slicing
print(f"Primeros 3: {numeros[:3]}")
print(f"Últimos 2: {numeros[-2:]}")

# List comprehension
cuadrados = [x**2 for x in range(5)]
print(f"Cuadrados: {cuadrados}")

# Diccionarios
print("\n=== DICCIONARIOS ===")
persona = {
    "nombre": "Alice",
    "edad": 25,
    "ciudad": "Lima"
}

print(f"Persona: {persona}")
print(f"Nombre: {persona['nombre']}")
print(f"Edad: {persona.get('edad', 'No especificada')}")

# Agregar/modificar
persona["profesion"] = "Programadora"
print(f"Con profesión: {persona}")

# Iterar sobre diccionario
print("\nClaves y valores:")
for clave, valor in persona.items():
    print(f"  {clave}: {valor}")

# Sets (conjuntos)
print("\n=== SETS ===")
colores = {"rojo", "verde", "azul"}
print(f"Colores: {colores}")
colores.add("amarillo")
print(f"Con amarillo: {colores}")

# Tuplas
print("\n=== TUPLAS ===")
coordenadas = (10, 20)
print(f"Coordenadas: {coordenadas}")
x, y = coordenadas  # Desempaquetado
print(f"x: {x}, y: {y}")

