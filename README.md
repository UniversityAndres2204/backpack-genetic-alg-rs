# 🧬 Algoritmo Genético para el Problema de la Mochila (Knapsack)

Este proyecto implementa el **clásico problema de optimización de la mochila (Knapsack Problem)** utilizando **algoritmos genéticos** en **Rust**.
El objetivo es maximizar el beneficio de los elementos seleccionados sin superar el peso máximo permitido.

---

## ✅ Descripción del Problema

Dado un conjunto de **bloques** (elementos), cada uno con un **peso** y un **beneficio**, el reto consiste en elegir una combinación que:

* **Maximice el beneficio total**.
* **No supere el peso máximo permitido** (`MAX_WEIGHT`).

Este programa resuelve el problema mediante un **algoritmo genético** que utiliza los siguientes operadores:

✔ **Inicialización aleatoria** de cromosomas.
✔ **Selección** de padres según probabilidad proporcional al fitness.
✔ **Cruce homogéneo** para generar hijos.
✔ **Mutación aleatoria** de genes.
✔ **Reemplazo** del peor individuo en la población.

---

## ⚙️ Parámetros principales

Se definen mediante constantes en el código:

| Constante                | Descripción                                   | Valor por defecto |
| ------------------------ | --------------------------------------------- | ----------------- |
| `BLOCKS`                 | Número de elementos disponibles               | `5`               |
| `MAX_WEIGTH`             | Peso máximo permitido en la mochila           | `16.0`            |
| `MAX_POPULATION`         | Tamaño de la población (número de cromosomas) | `5`               |
| `MUTATE_PROBABILITY`     | Probabilidad de mutación (0.8 = 80%)          | `0.8`             |
| `GENERATION_RESTRICTION` | Número máximo de generaciones                 | `8`               |

---

## 🛠 Estructuras principales

* **`Element`**: Representa un objeto con `weight` (peso) y `benefit` (beneficio).
* **`Score`**: Contiene la evaluación de un cromosoma (`index`, `total_benefit`, `probability`).
* **Cromosomas**: Representados como `Vec<bool>` donde `true` indica que el bloque está seleccionado.

---

## 🔍 Flujo del Algoritmo Genético

1. **Generación de la población inicial** (cromosomas aleatorios).
2. Para cada generación:

    * **Calcular fitness** (beneficio total sin exceder el peso).
    * **Seleccionar padres** basados en probabilidad proporcional.
    * **Cruzar** los padres para obtener un hijo.
    * **Mutar** el hijo con cierta probabilidad.
    * **Reemplazar** el peor cromosoma por el nuevo hijo.
3. **Después de todas las generaciones**, mostrar:

    * La población final con sus beneficios.
    * La mejor combinación encontrada.

---

## ▶ Ejecución del Programa

### **Compilar y ejecutar**

```bash
cargo run
```

---

## 📌 Ejemplo de salida

```
[+] Generating random chromosome...
[+] Initial population of 5 chromosomes
    Chromosome 0: [true, false, true, false, false]
    Chromosome 1: [false, true, false, false, true]
    Chromosome 2: [true, false, false, false, true]
    Chromosome 3: [true, true, false, true, false]
    Chromosome 4: [false, true, false, true, true]

<====== [ GENERACIÓN 0 ] ======>
[+] Mixed Chromosome
    [true, false, true, false, false] +
    [false, true, false, false, true] +
    [true, true, true, false, true] =
[!] Mutated Chromosome: [true, true, true, false, false]

...

[+] Mejor combinación encontrada en 7 generaciones: [Element { weight: 2.0, benefit: 2.0 }, Element { weight: 1.0, benefit: 2.0 }, Element { weight: 4.0, benefit: 10.0 }]
```