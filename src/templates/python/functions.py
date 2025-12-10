# Funciones en Python 3.12
# Definición y uso de funciones

# Función simple
def saludar(nombre: str) -> str:
    """Función que saluda a una persona"""
    return f"¡Hola, {nombre}!"

mensaje = saludar("Python")
print(mensaje)

# Función con múltiples parámetros
def calcular_area(largo: float, ancho: float) -> float:
    """Calcula el área de un rectángulo"""
    return largo * ancho

area = calcular_area(5.0, 3.0)
print(f"Área del rectángulo: {area}")

# Función con valores por defecto
def presentar(nombre: str, edad: int = 25, ciudad: str = "Desconocida"):
    """Presenta a una persona con información opcional"""
    print(f"{nombre}, {edad} años, de {ciudad}")

presentar("Alice")
presentar("Bob", 30)
presentar("Charlie", 28, "Lima")

# Función con *args (argumentos variables)
def sumar(*numeros: int) -> int:
    """Suma una cantidad variable de números"""
    total = 0
    for num in numeros:
        total += num
    return total

resultado = sumar(1, 2, 3, 4, 5)
print(f"Suma: {resultado}")

# Función con **kwargs (argumentos con nombre)
def mostrar_info(**datos: str):
    """Muestra información usando argumentos con nombre"""
    for clave, valor in datos.items():
        print(f"{clave}: {valor}")

mostrar_info(nombre="Alice", edad="25", ciudad="Lima")

