**1 de Octubre**

Hoy intente terminar los structs para la consulta de un comprobante mediante SOAP y aprendi lo siguiente

* Los atributos de las tags del xml van con un @
* Response no tiene el trait Terminate implementado por lo que no es buena idea ponerlo como el return de
la función main, al menos aqui es preferible devolver un Ok()
* Parece que los servicios del SRI funcionan de una manera peculiar, no puedes hacer consultas de comprobantes 
cuyo rango de fechas esta +10 dias de la fecha actual, pero parece que si lo puedes hacer directamente en la pagina web
* Otro de estos comportamientos peculiares es que diferencia el servicio expuesto por la clave de acceso del comprobante
porque parece que tiene un endpoint para consultar comprobantes emitidos por personas naturales y otro para personas juridicas 
