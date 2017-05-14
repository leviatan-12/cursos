from tkinter import *

root = Tk()
#Práctica 1. Crear una cadena de texto y mostrarla en la ventana
#the_label = Label(root,text="Esto es un hola mundo")
#the_label.pack()

#Práctica 2.
top_frame = Frame(root)
top_frame.pack()
bottom_frame = Frame(root)
bottom_frame.pack(side=BOTTOM)

boton_one = Button(bottom_frame, text="Click me 1",fg="red")
boton_two = Button(bottom_frame, text="Click me 2",fg="blue")
boton_tree = Button(bottom_frame, text="Click me 3",fg="purple")
boton_four = Button(top_frame, text="Click me 4",fg="green")

boton_one.pack(side=LEFT)
boton_two.pack(side=LEFT)
boton_tree.pack(side=LEFT)
boton_four.pack(side=LEFT)

root.mainloop()
