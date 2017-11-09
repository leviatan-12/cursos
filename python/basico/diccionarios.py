"""
Un diccionario es del tipo llave-valor
si dos elementos tienen la misma llave solo muestra el último
"""
dic_uno = {"uno" : 1 ,"dos" : 44, "uno" : False, 5 : 10}
print(dic_uno)

#Para agregar elementos al diccionario
dic_uno["tres"] = True
print(dic_uno)

#El mismo método sirve para editar un valor
dic_uno["tres"] = "Comida corrida"
print(dic_uno)

#Obtener valores del diccionario
print(dic_uno["dos"])

#El problema es que si no existe la llave sale un error, por lo que se recomienda...
valor = dic_uno.get('q',False)
print(valor)
#El segundo valor es lo que regresará en caso de no encontrar el elemento

#Para eliminar elementos del diccionario
del dic_uno["dos"]
print(dic_uno)

#para obtener una lista con las llaves o los valores
llaves = list (dic_uno.keys())
valores = list (dic_uno.values())
print(llaves)

#Para unir dos diccionarios
dic_dos = {"jaja" : 11, 12 : "jeje"}
dic_uno.update(dic_dos)
print(dic_uno)
