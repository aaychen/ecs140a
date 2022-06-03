my_reverse([], []). % base case for empty list
my_reverse([H | T], Y) :- my_reverse_helper(T, Y, [H]).
my_reverse_helper([], Y, Y). % base case when run out of chars to add to buffer
my_reverse_helper([H | T], Y, B) :- my_reverse_helper(T, Y, [H | B]). % keep adding first char to buffer and repeat on tail
