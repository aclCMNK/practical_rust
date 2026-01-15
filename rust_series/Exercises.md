Serie de Prácticas - Rust
Nivel 1: Fundamentos (Ownership y Borrowing)

Gestor de Tareas Simple: Crea un Vec de tareas (strings) con funciones para agregar, eliminar y listar. Practica pasar referencias mutables e inmutables.
Contador de Palabras: Lee un texto y cuenta la frecuencia de cada palabra usando HashMap. Practica iteradores y ownership de Strings.
Validador de Paréntesis: Verifica si los paréntesis/llaves están balanceados en una expresión usando un Vec como stack.

Nivel 2: Structs y Enums

Sistema de Biblioteca: Crea structs para Libro y Biblioteca. Implementa métodos para prestar/devolver libros. Practica métodos &self y &mut self.
Calculadora con Result: Implementa operaciones básicas que retornen Result<f64, String> para manejar errores como división por cero.
Sistema de Inventario: Usa enums para diferentes tipos de productos y pattern matching para calcular precios con descuentos según el tipo.

Nivel 3: Traits e Iteradores

Ordenador Genérico: Crea una función genérica que ordene cualquier tipo que implemente PartialOrd. Pruébala con números, strings y structs personalizados.
Filtro de Datos: Procesa un Vec de structs usando filter, map, fold. Por ejemplo, filtra estudiantes por nota y calcula promedio.
Trait Personalizado: Crea un trait Describible con método describir() e impleméntalo para 3 structs diferentes.

Nivel 4: Manejo de Errores y Archivos

Lector de CSV Simple: Lee un archivo CSV y convierte cada línea en un struct, manejando errores con Result y ?.
Registro de Eventos: Crea un sistema que escriba eventos en un archivo con timestamp, manejando todos los posibles errores de I/O.
Conversor de Formatos: Lee JSON simple (puedes usar serde_json) y conviértelo a otro formato, encadenando operaciones con ?.

Nivel 5: Proyectos Integradores

CLI de Notas: Aplicación de línea de comandos para gestionar notas (crear, buscar, editar, eliminar) guardadas en archivo JSON.
Analizador de Logs: Lee archivos de log, filtra por severidad, cuenta errores por tipo y genera un reporte.
Mini Web Scraper: Usa reqwest para hacer peticiones HTTP y extraer información simple de una página (títulos, enlaces).
