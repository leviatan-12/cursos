#Esto es una clase
class Lapiz:
    #Métodos
    def __init__(self, color, contiene_borrador, usa_grafito):
        self.color = color
        self.contiene_borrador = contiene_borrador
        self.usa_grafito = usa_grafito

    def __privada(self):
        print("Esta es una función que solo se accede internamente")
    def dibujar(self):
        print("Tu lápiz ha dibujado")
        self.__privada()

#Esto es un objeto
lapiz_generico = Lapiz("Amarillo",True,False)
lapiz_generico.dibujar()

#Herencia

#Padre. Las variables privadas no pueden ser accedidas por los hijos (como todo lenguaje OO)
class Felino:
    #Para que puedan acceder al valor de una variable, pero no editarlo, los property son una buena opcion
    @property
    def garras_retractiles(self):
        return True

    def __init__(self,nombre):
        self.nombre = nombre

    def nombre_felino(self):
        print(self.nombre)

    def cazar(self):
        print("El felino ha cazado")

#Hijos
class Gato(Felino):
    """docstring forGato."""
    def nombre_felino(self):
        #Felino.nombre_felino(self)
        print("El nombre de tu gato es {}".format(self.nombre))

class Jaguar(Felino):
    """docstring forJaguar."""

gato = Gato("Pepe")
jaguar = Jaguar("Pedro")
gato.nombre_felino()
#Se pueden crear atributos para los objetos sin necesidad de declararlo en la clase
