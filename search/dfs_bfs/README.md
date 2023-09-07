# DFS e BFS

**DFS** (*Depth-First Search*) e **BFS** (*Breadth-First Search*) sono due algoritmi di ricerca su alberi e grafi.

## Proprietà comuni

- I loop sullo stesso ramo vengono evitati;
- In caso di pareggio tra nodi alla stessa profondità, viene adottato l'ordine lessicografico (qualsiasi altro criterio va bene);
- **Backtracking**: riconsiderare decisioni valutate in precedenza e ripartire su un percorso alternativo;

Con queste due proprietà si applicano delle buone norme di efficienza, ma non bastano! Infatti evitare i loop impedisce di visitare nodi già visti solo lungo lo stesso ramo. L'idea è quindi quella di scartare un nodo appena generato se già presente da qualche parte nell'albero.

- **Lista degli accodamenti** (*Enqueued list*, **EQL**): se un nodo è già presente nella lista lo scarto, "potando" un ramo dell'albero, questa operazione è detta ***pruning***, perchè evito di generare una parte dell'albero di ricerca;

## Depth-First Search (DFS)

La ricerca in profondità sceglie il nodo più profondo (più in basso) non ancora esplorato nell'albero (grafo) di ricerca.

- La DFS con rimozione dei loop, backtracking e EQL è **corretta** e **completa**.

> Chiamiamo $b$ il massimo *branching factor*, cioè il massimo numero di azioni disponibili in uno stato (numero di nodi uscenti)
>
> Chiamiamo $d$ la profondità massima di una soluzione, cioè il massimo numero di azioni in un percorso dallo stato iniziale al goal

- Complessità spaziale: $O(d)$
- Complessità temporale: $1 + b + b^2 + b^3 + \dots + b^d = O(b^d)$

 ### Esecuzione sul running example

L'*EQL* evolve nel seguente modo:

1. EQL = {A}
2. EQL = {A, B, F}
3. EQL = {A, B, F, C, D}
4. EQL = {A, B, F, C, D, G}
5. EQL = {A, B, F, C, D, G, E}

Albero di ricerca:

```mermaid
graph TD;
	a((A))-->b((B))
	a-->f((F))
	b-->c((fa:fa-frown C))
    c -.->b
    b-->d((D))
    d-- fa:fa-cut -->ff((F))
    d-->g((G))
    g-->e((fa:fa-smile E))
```





