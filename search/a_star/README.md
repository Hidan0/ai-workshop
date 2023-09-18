# Ricerca informata, A*

## Ricerca *non informata* e *informata*

Una ricerca è **non informata** se utilizza solo la conoscenza del problema che è specificata nella sua definizione: il grafo, le sue connessioni, e il criterio con cui scegliere il prossimo nodo. Non considera la bontà del nodo stesso.



Una ricerca **informata** va oltre la definizione del problema sfruttando della conoscenza aggiuntiva: ciò che quel grafo, quelle connessioni e quei costi rappresentano nel mondo reale, oltre il formalismo agnostico che li esprime.

Dato un generico stato $S$, usando questa conoscenza, un algoritmo informato **stima** la bontà di $S$ attraverso una funzione $f(S)$ e guida la ricerca usando $f$.

Approccio **best-first** (*greedy*): **espandere prima gli stati che hanno una $f$ migliore**.

> N.B.: esistono diversi algoritmi di ricerca best-first, la differenza la fa il come $f$ è definita



## A*

L'idea alla base di **A\*** è quella di eseguire una UCS, ma invece di considerare soltanto il costo $g$, considerare la funzione $f(s)=g(s) + h(s)$. L'algoritmo di ricerca selezione per l'espansione i nodi sulla frontiera che minimizzano $f$.

- Con $g(s)$ indichiamo, come prima, il costo accumulato lungo il percorso che arriva nello stato $s$

- Con $h(s)$ indichiamo una *stima* del costo ancora da spendere per arrivare al goal lungo il percorso detta **euristica**

### Euristica

#### Ammissibilità

Una proprietà fondamentale che una buona euristica deve avere è l'**ammissibilità**: 

- un'euristica $h$ è ammissibile se per ogni possibile stato $s$, $h(s)$ **non sovrastima il costo del percorso minimo $s$ al goal**

La stima essere ottimista, se questa proprietà non vale, l'algoritmo di ricerca potrebbe non riconoscere il percorso ottimo!

#### Consistenza

L'ammissibilità di per sé non garantisce l'ottimalità! Per risolvere questo problema dobbiamo chiedere all'euristica una proprietà più stringente dell'ammissibilità: la **consistenza**.

Siano $V$ e $U$ due stati connessi da una azione $a$

- una euristica $h$ è **consistente** se per ogni possibile coppia di $V$ e $U$ vale la seguente diseguaglianza: $h(V) \leq c(V, a, U) + h(U)$ (è una diseguaglianza triangolare)

Un'euristica consistente è anche ammissibile






