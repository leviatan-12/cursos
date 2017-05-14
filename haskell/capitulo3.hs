-- Definir un tipo de dato nuevo de tipo libro
data BookInfo = Book Int String [String]
                deriving (Show)

-- Definir un tipo de dato nuevo de tipo revista
data MagazineInfo = Magazine Int String [String]
                    deriving (Show)

myInfo = Book 12345678 "Prueba de titulo" ["Autor 1","Autor 2"]

-- Pero hay una mejor forma para que podamos entender que significa
-- cada elemento
-- La tecla type sirve para poner "sinonimos" a variables ya existentes
type CustomerID = Int
type ReviewBody = String

data BetterReview = BetterReview BookInfo CustomerID ReviewBody

--Hay algo llamado algebraic data type y puede tener más de un constructor
--El | es un or entre una definición del elemento o en otra
type CardHolder = String
type CardNumber = String
type Address = [String]

data BillingInfo = CreditCard CardNumber CardHolder Address
                  | CashOnDelivery
                  | Invoice CustomerID
                  deriving (Show)

--Reconocimiento de patrones
sumList (x:xs) = x + sumList xs
sumList [] = 0
