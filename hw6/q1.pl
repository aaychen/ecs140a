my_last(X, [X]). % single item left check
my_last(X, [_ | T]) :- my_last(X, T). % compare to tail of list