{-
Una función de order superior es una función que puede tomar como parámetros
otras funciones o regresar funciones. Antes hay que entender una regla básica de
Haskell: solo toma un valor y le aplica la función. El -> significa que dates
regresa y decidir poner otra vez 'a' significa que con el resultado anterior al
nuevo valor le aplica la función
-}
isUpperCase :: Char -> Bool

isUpperCase = (`elem` ['A'..'Z'])

--Esta que viene es una función de order superior
dosVeces :: (a -> a) -> a -> a

dosVeces f x = f (f x)

--Aplicados al quicksort

qt :: (Ord a) => [a] -> [a]
qt [] = []
qt (x:xs) =
    let smallerSorted = qt (filter (<=x) xs)
        biggerSorted = qt (filter (>x) xs)
    in  smallerSorted ++ [x] ++ biggerSorted
