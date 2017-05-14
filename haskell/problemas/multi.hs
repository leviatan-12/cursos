main = do
      numeros <- getLine
      print (suma $ words numeros)
suma :: [String] -> Int
suma (x:xs) = (read x) *  (suma xs)
suma [] = 1
