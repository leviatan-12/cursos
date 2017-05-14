#String con saltos de línea

str_con_salto = """Este es un string
con saltos de linea
directamente"""

print(str_con_salto)

str_con_salto = "Este es otro string\ncon saltos de linea\npero con el diagonal invertida n"

print(str_con_salto)
print("-----------------------")

#Métodos para concatenar cadenas
cadena_simple_uno = "Simple"
cadena_simple_dos = "Complejo"

concatenar_uno = cadena_simple_uno +" es mejor que "+cadena_simple_dos

print(concatenar_uno)

mejor_metodo = "%s mejor que %s" %(cadena_simple_uno,cadena_simple_dos)

print(mejor_metodo)

#metodos de cadenas
print("-----------------------")

cadena_uno = "Prueba"
cadena_dos = "CÓDIGO"

#Otra forma de concatenar cadenas de código
respuesta = "{a} de {b}".format(b=cadena_dos,a=cadena_uno)
print(respuesta)

#para minúsculas
respuesta = respuesta.lower()
print(respuesta)

#para mayúsculas
respuesta = respuesta.upper()
print(respuesta)

#para que la primera letra de cada palabra sea mayúscula
respuesta = respuesta.title()
print(respuesta)

#para hacer búsquedas
pos = respuesta.find(cadena_dos.title())
print(pos)

#contar cuantas veces sale una cadena o un caracter
cuenta = respuesta.count('e')
print(cuenta)

#Métoodo para reemplazar una cadena o caracter
reemplazada = respuesta.replace('e','x')
print(reemplazada)

#Método split
dividida = respuesta.split(' ')
for i in dividida:
    print(i)
