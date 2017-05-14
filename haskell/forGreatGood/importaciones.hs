{-
Vamos a ver como importar paquetes en las diferentes formas posibles:
La más básica es

  import Data.List

Si queremos importar solo unas funciones del archivo

  import Data.List (nub,sort)

Ahora, si queremos importar todas las funciones menos una

  import Data.List hiding (nub)

¿Y qué pasa cuando dos módulos tienen una función que se llama igual? 

  import qualified Data.List as M

y cada vez que nos refiramos a la función de ese módulo lo haremos
  M.sort
-}
