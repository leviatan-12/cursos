try:
    print(1/0)
except Exception as ex:
    print(ex)
finally:
    print("Acabo el script")
