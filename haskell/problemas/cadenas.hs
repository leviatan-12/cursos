import Data.Char as DC
main =
  if hIsEOF linea
    then
      do
      putStrLn "Hola"
    else
      do
      linea <- getLine
      putStrLn $ mayusculas linea
      putStrLn $ minusculas linea
      main

mayusculas :: [Char] -> [Char]
mayusculas (x:xs) = [DC.toUpper x] ++ (mayusculas xs)
mayusculas [] = []

minusculas :: [Char] -> [Char]
minusculas (x:xs) = [DC.toLower x] ++ (minusculas xs)
minusculas [] = []
