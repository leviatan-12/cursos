#If
numero_azar = 2

if numero_azar == 1:
    print("igual a uno")
elif numero_azar > 1 and True:
    print("mayor a uno")
elif numero_azar < 0 or numero_azar == 0:
    #Que pase la condición sin hacer nada
    pass
else:
    print("no es igual a uno")

mensaje = "El valor de este if inline es == 1" if numero_azar == 1 else "No sirve"
print(mensaje)

#While
i = 0
while i<10:
    print(i)
    i += 1
else:
    print("acabo")

#for

lista_rango = range(0,10)
for i in lista_rango:
    print("Ahora vamos en: ",i)

#comprehension list, se pueden hacer listas, tuplas y diccionarios

lista_rapida = [valor for valor in range(0,10) if valor % 2 ==0 ]
print(lista_rapida)

tupla_rap = tuple((valor for valor in range(0,10) if valor % 2 ==0))
print(tupla_rap)

diccionario_rap = {indice:valor for indice,valor in enumerate(lista_rapida)}
print(diccionario_rap)
