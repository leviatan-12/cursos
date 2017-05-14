lista_uno = ["uno","dos","tres",1,2,3,1,1.0,2.0,3.0,True]
print(lista_uno)

#Agregar un elemento al final de la lista
lista_uno.append(6)
print(lista_uno)

#Agregar un elemento a una posición elegida
lista_uno.insert(1,"Otro uno")
print(lista_uno)

"""
Eliminar elementos de la lista
Nota: Elimina el primer elemento que coincidan con el método de búsqueda
"""
lista_uno.remove(1)
print(lista_uno)

#Obtener el último valor de la lista
ultimo_ele = lista_uno.pop()
print(ultimo_ele)

"""
Método de ordenamiento de listas
solo funciona con datos del mismo tipo
el reverse es para indicar que no lo quiere de menor a mayor, sino al revés (es bastante obvio)
"""
lista_dos = [1,2,3,4,5]
lista_dos.sort(reverse = True)
print(lista_dos)

#crear lista de listas

lista_uno.extend(lista_dos)
print(lista_uno)
