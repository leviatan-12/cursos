#Asi se definen funciones
def factorial():
    pass

#lambdas

mi_funcion = lambda valor_u,valor_d : valor_u + valor_d
res = mi_funcion(3,5)
print(res)

otra_fun = lambda sentencia : "¿" + sentencia + "?"
res = otra_fun("Quién es Sherlock Holmes")
print(res)

no_valor = lambda : 10
res = no_valor()
print(res)

"""
DECORADORES
Es una función a la que le mandas como parametro
otra función, y le agregas funcionalidad extra
a la función que le pasas como parámetro
"""
#Función decoradora
def funcion_decorador(func):
    def nueva_fun(*args):
        print("Antes")
        func(args[0],args[1])
        print("Después")
    return nueva_fun

#Función decorada
@funcion_decorador
def funcion_original(a,b):
    print(a+b)

funcion_original(1,5)
