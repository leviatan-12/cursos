lastButOne n = if tail n == [last n]
              then head n
              else lastButOne (tail n)
