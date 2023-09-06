# Gestione dei Grafi

## Descrizione

Questo progetto è una libreria di base per la gestione dei grafi. È stata sviluppata per supportare l'implementazione degli algoritmi di ricerca visti nel corso di Intelligenza Artificiale. In particolare offre una struttura dati che rappresenta il grafo degli stati e il *running example* (vedi `README`sotto `search/`).

## Implementazione

Questa libreria è stata sviluppata seguendo il tutorial di **Laket** come punto di partenza ([Lakret/gir](https://github.com/Lakret/gir/tree/graphs)).

L'implementazione del grafo si basa sulla struttura della lista di adiacenze, che è comunemente utilizzata per rappresentare grafi in modo efficiente.

Dato che gli identificatori dei nodi (`VId`) nel *running example* sono costituiti da stringhe di dimensioni ridotte ($\{A, B, C, D, E, F, G\}$), per garantire un'efficienza ottimale, ho scelto di utilizzare le hash map `FnvHashMap`, che fanno uso della funzione di hash *Fowler-Noll-Vo* (più efficiente per chiavi di dimensioni limitate).