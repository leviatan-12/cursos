--Esta función regresa verdadero cuando el elemento es par
esPar n = mod n 2 == 0
--Esta función regresa verdadero cuando el elemento es impar
esImpar n = not (esPar n)
--Con esta función, se sabe si un elemento es par o impar
saberTipo n = if esPar n
            then "es par"
            else "es impar"
