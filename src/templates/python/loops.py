# Bucles en Python 3.12
# for y while loops

# Bucle for con range
print("Bucle for (0-4):")
for i in range(5):
    print(f"  Iteración {i}")

# Bucle for con lista
print("\nElementos de una lista:")
frutas = ["manzana", "banana", "naranja"]
for fruta in frutas:
    print(f"  - {fruta}")

# Bucle for con enumerate
print("\nBucle con índice:")
for indice, fruta in enumerate(frutas):
    print(f"  [{indice}] {fruta}")

# Bucle while
print("\nBucle while:")
contador = 0
while contador < 3:
    print(f"  Contador: {contador}")
    contador += 1

# Bucle con break y continue
print("\nBucle con break:")
for i in range(10):
    if i == 5:
        break  # Sale del bucle
    print(f"  {i}")

print("\nBucle con continue:")
for i in range(5):
    if i == 2:
        continue  # Salta esta iteración
    print(f"  {i}")

