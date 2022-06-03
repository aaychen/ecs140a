% given code from lecture
my_append([], L2, L2).
my_append([H1 | T1], L2, [H1 | T3]) :- my_append(T1 , L2 , T3).
prefix(L1, L2) :- my_append(L1, _, L2).

common_prefix(X, [H | Ys]) :- prefix(X, H), common_prefix(X, Ys). % check if X is prefix of first list and repeat for rest of the lists
common_prefix(X, [H | []]) :- prefix(X, H). % base case if only have 1 list to compare against left
