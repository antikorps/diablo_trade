# Diablo Trade
## Descripción y funcionamiento
El programa presenta dos funcionalidades:
- El modo scraper permite recuperar toda la información disponible en el histórico de objetos vendidos de diablo trade y guardarlo en una base de datos sqlite3. 
- El modo comparación permite analizar todos los objetos de un usuario y compararlos con los registros del histórico mostrando aquellos que comparten 3 o 4 afijos con un valor comprendido entre el rango de comparación establecido


Para ejecutar el programa no es necesaria ninguna instalación adicional. Si no se quiere compilar el código fuente puede descargarse el ejecutable desde "Releases". También se recomienda descarga la base de datos [diablo_trade.zip](https://drive.google.com/file/d/1TWtRagiSV-jdQACLQe6-wXybljeXJGWv/view?usp=sharing) para evitar todo el proceso de scraping desde el principio (recuerda que debe descomprimirse en el mismo directorio que el binario).

Antes de su ejecutar se debe preparar el archivo "configuracion.json" (también en el mismo directorio que el binario) con la siguiente estructura:
```json
{
    "usuario": "111XXX111",
    "rango_comparacion": 20,
    "max_base_datos": 300000
}
```
La clave **usuario** es la cadena alfanumérica que aparece en las URL de este tipo: https://diablo.trade/user/111XXX111/items Si la URL fuera correcta, el usuario sería 111XXX111 

La clave **rango_comparacin** debe ser de tipo numérico e indica el porcentaje de valores aceptados (inferior o superior) que deben cumplir los afijos que se comparan.

Finalmente, **max_base_datos** permite controlar el tamaño de la base de datos e indica el máximo de registros que puede tener.

## Ejecución
Es importante recordar que el programa debe tener permisos de ejecución. En sistemas GNU/Linux puede hacerse con el comando:
```bash
sudo chmod +x diablo_trade
```
El modo scraper se inicia si se incorpora el argumento scraper, es decir:
```bash
./diablo_trade --scraper
```
También permite el uso de argumentos minificados:
```bash
./diablo_trade -s
```

## Resultado
Una vez ejecutado creará un archivo HTML para consultar los resultados obtenidos. Este archivo se llamará **index.html** y se creará automáticamente en el directorio del ejecutable. **Si existe un archivo previamente con ese nombre se sobrescribirá** 
