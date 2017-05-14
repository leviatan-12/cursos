{-
factorial n = if n>1
              then n * factorial (n-1)
              else 1
-}
-- Cuando una función no tiene parametros es una definición o un nombre

nombreDefinicion = "Hola mundo, soy una definicion"

{-
Hay diferentes formas de construir funciones, una de ellas es a través de
patrones como en el ejemplo 1. La otra es a través de guards como en el ejemplo
2. Durante este último ejemplo hay un par de vertientes como es el uso de where
y let (ejemplos 3 y 4), con sus respectivas ventajas y desventajas.
-}

--Ejemplo 1

factorial :: Integer -> Integer
factorial 0 = 1
factorial n = n * factorial (n-1)

--Ejemplo 2

factorial1 :: Integer -> Integer
factorial1 a
  | a>1 = a * factorial1 (a-1)
  | True = 1

{-
Para estos ejemplos usare otro programa diferente al ejemplo 2.
-}
-- Original

imc :: Double -> String

imc indice
  | indice <= 18.5 = "Desnutrido"
  | indice <= 25.0 = "Perfecto"
  | indice <= 30.0 = "Gordito"
  | otherwise = "Eres demasiado pesado"

--Con peso y altura en lugar del indice

imc1 :: Float -> Float -> String

imc1 altura peso
  | peso / (altura^2) <= 18.5 = "Desnutrido"
  | peso / (altura^2) <= 25.0 = "Perfecto"
  | peso / (altura^2) <= 30.0 = "Gordito"
  | otherwise = "Eres demasiado pesado"

--Ejemplo 3

imc2 :: Float -> Float -> String

imc2 altura peso
  | indice <= ligero = "Desnutrido"
  | indice <= normal = "Perfecto"
  | indice <= pesado = "Gordito"
  | otherwise = "Eres demasiado pesado"
  where indice = peso / (altura^2)
        ligero = 18.5
        normal = 25.0
        pesado = 30.0

--Ejemplo 4
{-
La aparente diferencia entre where y let es que let te permite escribir la
equivalencia antes y el where lo hace después, sin embargo, let lo puedes usar
en un monton de lugares, aunque no en los guards.
-}
listaConLet =  (let a = 100; b = 200; c = 300 in a*b*c, let foo="Hey "; bar = "there!" in foo ++ bar)

--También podemos usar let en las list comprehension

calcularIMC :: (RealFloat a) => [(a,a)] -> [a]

calcularIMC xs = [imc3 | (w,h)<-xs,let imc3 = w/(h^2),imc3>=25.0]
