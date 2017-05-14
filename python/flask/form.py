from wtforms import (Form,StringField,TextField,validators,HiddenField)
from wtforms.fields.html5 import EmailField

def length_honeypot(form, field):
    if len(field.data) > 0:
        raise validators.ValidationError('El campo debe estar vacío.')


class CommentForm(Form):
    username = StringField('Username',[
        validators.Required(message='Se requiere un usuario'),
        validators.length(min=4,max=10,message='Ingrese un usuario válido.')
    ])
    email = EmailField('Correo electrónico',[
        validators.Required(message="Se requiere un correo"),
        validators.Email(message="Se requiere un correo válido")
    ])
    comment = TextField('Comentario')
    honeypot = HiddenField('',[length_honeypot])
